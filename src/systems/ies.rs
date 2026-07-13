use core::panic;
use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
};

use dashmap::DashMap;
use tokio::{
    net::UdpSocket,
    sync::{
        mpsc::{Receiver, Sender},
        oneshot,
    },
    task::JoinSet,
};

use crate::{
    MAX_PACKET_SIZE, OWN_IP_SOCKET_ADDR, buffer::PacketBuffer, isc::HalfDuplexMessage,
    protocol::packet::DnsPacket, query_state::QueryState, systems::IesResolveCommand,
};

struct InflightQuery {
    original_id: u16,
    sender: oneshot::Sender<Option<DnsPacket>>,
}

/// # Ingress and Egress System
/// The **Ingress and Egress System**, **IES** for short, is the interface between the service and any external network instances related to the DNS protocol.
/// It owns all network sockets related to DNS specific operations, parses incoming DNS packets into a [`crate::protocol::packet::DnsPacket`] wraps it in a
/// [`crate::query_state::QueryState`].
///
/// ## Area of Responsibility
/// The IES acts as the network manager and translator between the raw DNS protocol and service specific data structures.
/// It does **not** concern itself with business logic such as mapping upstream responses to upstream queries.
///
/// ## Tasks
/// The IES is made up of four tasks:
///
/// - Client Ingress Task
/// - Client Egress Task
/// - Upstream Egress Task
/// - Upstream Ingress Task
///
/// ### Client Ingress Task
/// Listens for queries from network hosts. This is the entry of a DNS query into the service.
///
/// ### Client Egress Task
/// Sends responses to queries to the respective client. The is the exit of the DNS query out of the service.
///
/// ### Upstream Egress Task
/// Sends a query to an upstream resolver (like Cloudflare 1.1.1.1 or Google 8.8.8.8).
///
/// ### Upstream Ingress Task
/// Listens to responses from the upstream resolver.
pub struct IngressEgressSystem {
    socket: Arc<UdpSocket>,
    upstream_resolver_ip: Ipv4Addr,
    ies_to_dps_tx: Sender<QueryState>,
    ies_answer_rx: Receiver<QueryState>,
    ies_upstream_resolve_rx: Receiver<HalfDuplexMessage<IesResolveCommand, Option<DnsPacket>>>,
}

impl IngressEgressSystem {
    pub async fn run(mut self) -> JoinSet<()> {
        println!("Starting IES...");

        let mut join_set = JoinSet::<()>::new();

        let downstream_socket_tx = self.socket.clone();
        let downstream_socket_rx = self.socket.clone();

        let upstream_socket = Arc::new(UdpSocket::bind("0.0.0.0:0").await.unwrap());

        let upstream_socket_tx = upstream_socket.clone();
        let upstream_socket_rx = upstream_socket.clone();

        let inflight_queries = Arc::new(DashMap::<u16, InflightQuery>::new());
        let egress_inflight_queries = inflight_queries.clone();
        let ingress_inflight_queries = inflight_queries.clone();

        if let Err(e) = upstream_socket
            .connect(SocketAddr::new(self.upstream_resolver_ip.into(), 53))
            .await
        {
            panic!(
                "Error while trying to connect to upstream resolver at {}. Error: {}",
                self.upstream_resolver_ip, e
            );
        }

        // CLIENT INGRESS TASK
        join_set.spawn(async move {
            println!("Starting incoming query task...");

            loop {
                let mut buf = [0_u8; MAX_PACKET_SIZE];
                let (_, origin) = match downstream_socket_rx.recv_from(&mut buf).await {
                    Err(e) => {
                        eprintln!("Error while receiving DGRAM: {:#?}", e);
                        continue;
                    }
                    Ok(value) => value,
                };

                println!("[IES] RECEIVED QUERY");

                // Parse bytes to DnsPacket
                let mut packet_buf = PacketBuffer::from_raw_buffer(buf);
                let packet = match DnsPacket::parse_from(&mut packet_buf) {
                    Err(_) => continue,
                    Ok(packet) => packet,
                };

                // Create QueryState
                let query_state = QueryState::new(origin, packet);

                // Send DnsPacket to DPS
                if let Err(e) = self.ies_to_dps_tx.try_send(query_state) {
                    match e {
                        tokio::sync::mpsc::error::TrySendError::Closed(_) => {
                            panic!("DPS channel closed unexpectedly.")
                        }
                        tokio::sync::mpsc::error::TrySendError::Full(_) => {
                            println!("IES-DPS channel overloaded. Dropping query")
                        }
                    }
                }
            }
        });

        // CLIENT EGRESS TASK
        // This task receives answered query states and sends them back to the querying client.
        join_set.spawn(async move {
            println!("Starting answer task...");
            loop {
                let received_query_state = match self.ies_answer_rx.recv().await {
                    None => {
                        panic!("IES RX channel to receive answered queries closed unexpectedly.")
                    }
                    Some(v) => v,
                };

                println!("[IES]  RECEIVED ANSWER -> SENDING TO CLIENT");

                // Translate the DnsPacket into bytes
                let mut buffer = PacketBuffer::<MAX_PACKET_SIZE>::new();
                let bytes = match received_query_state.get_packet().to_bytes(&mut buffer) {
                    Err(e) => {
                        eprintln!("Error while translating query to bytes: {}", e);
                        continue;
                    }
                    Ok(v) => v.to_be_bytes(),
                };

                // Send data to downstream client
                if let Err(e) = downstream_socket_tx
                    .send_to(buffer.as_slice(), received_query_state.get_origin())
                    .await
                {
                    eprintln!(
                        "Error {e} occurred while trying to send answer to client. Dropping query"
                    );
                }
            }
        });

        // UPSTREAM EGRESS TASK
        // This task handles the query forwarding to an upstream resolver.
        join_set.spawn(async move {
            println!("Starting upstream forward task...");
            loop {
                let mut message = match self.ies_upstream_resolve_rx.recv().await {
                    None => {
                        panic!("IES-SRS resolving channel closed unexpectedly.");
                    }
                    Some(v) => v,
                };

                println!("[IES - UPSTREAM EGRESS] RECEIVED FORWARD QUERY FROM SRS -> forwarding");

                // Replace query ID
                let original_id = message.payload.packet.header.id;
                let horizon_id = Self::generate_id();
                message.payload.packet.header.id = horizon_id;

                // Create InflightQuery
                let inflight_query = InflightQuery {
                    original_id,
                    sender: message.return_channel,
                };


                // Translate packet into bytes
                let mut buffer = PacketBuffer::<MAX_PACKET_SIZE>::new();
                let _bytes = match message.payload.packet.to_bytes(&mut buffer) {
                    Err(e) => {
                        eprintln!("Error while translating query for upstream resolver. Error: {}", e);
                        if inflight_query.sender.send(None).is_err() {
                            eprintln!("[IES] Could not notify SRS of above failure due failure in the return channel.");
                        }
                        continue;
                    }
                    Ok(v) => v,
                };

                let mut parse_buf = PacketBuffer::<MAX_PACKET_SIZE>::from_raw_buffer(buffer.as_slice().to_owned());
                let test_packet = match DnsPacket::parse_from(&mut parse_buf) {
                    Err(e) => {
                        eprintln!("Error while parsing the outgoing packet. Error: {:?}", e);
                        continue;
                    }
                    Ok(packet) => packet,
                };

                println!("");
                println!("---------------------");
                println!("{:#?}", message.payload.packet);
                println!("---------------------");
                println!("");

                println!("");
                println!("---------------------");
                println!("{:#?}", test_packet);
                println!("---------------------");
                println!("");

                // Register query as being inflight
                egress_inflight_queries.insert(horizon_id, inflight_query);

                // Send via upstream_socket
                if let Err(e) = upstream_socket_tx.send(buffer.as_slice()).await {
                    eprintln!(
                        "Socket error - could not forward query to upstream resolver. Error: {e}"
                    );

                    if let Some((_, removed_query)) = egress_inflight_queries.remove(&horizon_id) {
                        let send_result = removed_query.sender.send(None);
                        if send_result.is_err() {
                            eprintln!("Failed to notify SRS of above failure.");
                        }
                    }
                    continue;
                }

                // Let the task sleep until the query's timeout is elapsed - once woken up, remove the task from the `InflightQuery` map, thereby effectively dropping the query
                tokio::time::sleep(message.payload.timeout).await;
                if let Some((_, removed_query)) = egress_inflight_queries.remove(&horizon_id) {
                   let send_result = removed_query.sender.send(None);
                    if send_result.is_err() {
                        eprintln!("Failed to notify SRS that query timed out.");
                    }
                }
            }
        });

        // UPSTREAM INGRESS TASK
        // This task handles receiving data from the upstream resolver and forwards it to the SRS.
        join_set.spawn(async move {
            loop {
                // Receive data from upstream
                let mut buf = [0_u8; MAX_PACKET_SIZE];
                match upstream_socket_rx.recv(&mut buf).await {
                    Err(e) => {
                        eprintln!("Error while receiving upstream response: {e}");
                        continue;
                    }
                    Ok(size) => {
                        if size > MAX_PACKET_SIZE {
                            eprintln!("Max packet size exceeded. Dropping...");
                            continue;
                        }
                    }
                };

                println!("[IES] RECEIVED UPSTREAM ANSWER -> SENDING TO SRS");

                // Parse upstream data
                let mut packet_buf = PacketBuffer::from_raw_buffer(buf);
                let mut packet = match DnsPacket::parse_from(&mut packet_buf) {
                    Err(_) => {
                        eprintln!("Error while parsing received packet. Dropping...");
                        continue;
                    }
                    Ok(packet) => packet,
                };

                // Check if upstream response answers an inflight query and send it to the SRS if successful
                if let Some((_, inflight_query)) =
                    ingress_inflight_queries.remove(&packet.header.id)
                {
                    // Replace ID with original ID
                    packet.header.id = inflight_query.original_id;

                    if inflight_query.sender.send(Some(packet)).is_err() {
                        eprintln!(
                            "Return channel failure - could not send response to SRS. Dropping..."
                        );
                        continue;
                    }
                }
            }
        });

        join_set
    }

    pub async fn new(
        upstream_resolver_ip: Ipv4Addr,
        ies_to_dps_tx: Sender<QueryState>,
        ies_answer_rx: Receiver<QueryState>,
        ies_upstream_resolve_rx: Receiver<HalfDuplexMessage<IesResolveCommand, Option<DnsPacket>>>,
    ) -> Self {
        let socket = Arc::new(UdpSocket::bind(OWN_IP_SOCKET_ADDR).await.unwrap());
        Self {
            socket,
            upstream_resolver_ip,
            ies_to_dps_tx,
            ies_answer_rx,
            ies_upstream_resolve_rx,
        }
    }

    fn generate_id() -> u16 {
        rand::random::<u16>()
    }
}
