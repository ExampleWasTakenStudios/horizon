use core::panic;
use std::{
    net::{Ipv4Addr, SocketAddr}, sync::Arc,
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
    IP_ADDR, MAX_PACKET_SIZE, buffer::PacketBuffer, isc::HalfDuplexMessage,
    protocol::packet::DnsPacket, query_state::QueryState, systems::IesResolveCommand,
};

struct InflightQuery {
    original_id: u16,
    sender: oneshot::Sender<Option<DnsPacket>>,
}

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

        let socket_ingress_clone = self.socket.clone();
        let socket_egress_clone = self.socket.clone();

        let upstream_socket = Arc::new(UdpSocket::bind("0.0.0.0:0").await.unwrap());

        let upstream_socket_tx = upstream_socket.clone();
        let upstream_socket_rx = upstream_socket.clone();

        let inflight_queries = Arc::new(DashMap::<u16, InflightQuery>::new());
        let egress_inflight_queries = inflight_queries.clone();
        let ingress_inflight_queries = inflight_queries.clone();

        if let Err(e) = upstream_socket
            .connect(SocketAddr::new(self.upstream_resolver_ip.into(), 53))
            .await {
            panic!("Error while trying to connect to upstream resolver at {}. Error: {}", self.upstream_resolver_ip, e);
        }

        // This task handles all incoming queries.
        join_set.spawn(async move {
            println!("Starting incoming query task...");

            loop {
                let mut buf = [0_u8; MAX_PACKET_SIZE];
                let (_, origin) = match socket_ingress_clone.recv_from(&mut buf).await {
                    Err(e) => {
                        eprintln!("Error while receiving DGRAM: {:#?}", e);
                        continue;
                    }
                    Ok(value) => value,
                };

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

                // Translate the DnsPacket into bytes
                let bytes = match received_query_state.get_packet().to_raw_bytes() {
                    None => {
                        eprintln!("Error while translating query to bytes.");
                        continue;
                    }
                    Some(v) => v,
                };

                // Send data to downstream client
                if let Err(e) = socket_egress_clone
                    .send_to(&bytes, received_query_state.get_origin())
                    .await
                {
                    eprintln!(
                        "Error {e} occurred while trying to send answer to client. Dropping query"
                    );
                }
            }
        });

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

                // Translate packet into bytes
                let bytes = match message.payload.packet.to_raw_bytes() {
                    None => {
                        eprintln!("Error while translating query for upstream resolver.");
                        continue;
                    }
                    Some(v) => v,
                };

                // Replace query ID
                let original_id = message.payload.packet.header.id;
                let new_id = Self::generate_id();
                message.payload.packet.header.id = new_id;

                // Store ID mapping
                let inflight_query = InflightQuery {
                    original_id,
                    sender: message.return_channel,
                };

                egress_inflight_queries.insert(new_id, inflight_query);

                // Send via upstream_socket and forget
                if let Err(e) = upstream_socket_tx.send(&bytes).await {
                    eprintln!(
                        "Socket error - could not forward query to upstream resolver. Error: {e}"
                    );

                    match egress_inflight_queries.remove(&new_id) {
                        None => (),
                        Some((_, removed_query)) => {
                            let send_result = removed_query.sender.send(None);
                            if send_result.is_err() {
                                eprintln!("Failed to notify SRS.");
                            }
                        }
                    }
                    continue;
                }

                tokio::time::sleep(message.payload.timeout).await;
                egress_inflight_queries.remove(&new_id);
            }
        });

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
                    },
                };

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
                match ingress_inflight_queries.remove(&packet.header.id) {
                    Some((_, inflight_query)) => {

                        // Replace ID with original ID
                        packet.header.id = inflight_query.original_id;

                        if inflight_query.sender.send(Some(packet)).is_err() {
                            eprintln!("Return channel failure - could not send response to SRS. Dropping...");
                            continue;
                        }
                    }
                    None => {
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
        let socket = Arc::new(UdpSocket::bind(IP_ADDR).await.unwrap());
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
