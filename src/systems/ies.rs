use std::{net::SocketAddr, sync::Arc, thread::JoinHandle};

use tokio::{
    net::UdpSocket,
    sync::mpsc::{Receiver, Sender},
};

use crate::{
    IP_ADDR, MAX_PACKET_SIZE, buffer::PacketBuffer, isc::HalfDuplexMessage,
    protocol::packet::DnsPacket, query_state::QueryState, systems::IesCommand,
};

pub struct IngressEgressSystem {
    socket: Arc<UdpSocket>,
    ies_to_dps_tx: Sender<QueryState>,
    ies_answer_rx: Receiver<QueryState>,
    ies_upstream_resolve_rx: Receiver<HalfDuplexMessage<IesCommand, Option<DnsPacket>>>,
}

impl IngressEgressSystem {
    pub async fn run(self) {
        println!("Starting IES...");

        let socket_ingress_clone = self.socket.clone();
        let socket_egress_clone = self.socket.clone();

        let ies_to_dps_tx_clone = self.ies_to_dps_tx.clone();
        let mut ies_answer_rx_clone = self.ies_answer_rx;
        let mut ies_upstream_resolve_rx_clone = self.ies_upstream_resolve_rx;

        // This task handles all incoming queries.
        tokio::spawn(async move {
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
                ies_to_dps_tx_clone.send(query_state).await;
            }
        });

        // This task receives answered query states and sends them back to the querying client.
        tokio::spawn(async move {
            println!("Starting answer task...");
            loop {
                let received_query_state = match ies_answer_rx_clone.recv().await {
                    None => {
                        panic!("IES RX channel to receive answered queries closed unexpectedly.")
                    }
                    Some(v) => v,
                };

                // Translate the DnsPacket into bytes
                let bytes = match received_query_state.get_packet().to_raw_bytes() {
                    None => {
                        eprintln!("Error while translating query to bytes.");
                        return;
                    }
                    Some(v) => v,
                };

                // Send data to downstream client
                socket_egress_clone
                    .send_to(&bytes, received_query_state.get_origin())
                    .await;
            }
        });

        // This task handles the query forwarding to an upstream resolver and returns it to the SRS.
        tokio::spawn(async move {
            println!("Starting upstream forward task...");
            loop {
                let message = match ies_upstream_resolve_rx_clone.recv().await {
                    None => {
                        panic!("IES-SRS resolving channel closed unexpectedly.");
                    }
                    Some(v) => v,
                };

                // Translate packet into bytes
                let bytes = match message.payload.packet.to_raw_bytes() {
                    None => {
                        eprintln!("Error while translating query for upstream resolver.");
                        return;
                    }
                    Some(v) => v,
                };

                // Spawn socket
                tokio::spawn(async move {
                    let mut receive_buf = [0_u8; MAX_PACKET_SIZE];
                    let socket = match tokio::net::UdpSocket::bind("0.0.0.0:0").await {
                        Err(e) => {
                            eprintln!("Error while trying to bind resolving socket. Error: {e}");
                            message.return_channel.send(None);
                            return;
                        }
                        Ok(v) => v,
                    };

                    if let Err(e) = socket.connect(message.payload.resolver_address).await {
                        eprintln!(
                            "Error while trying to send query to upstream resolver. Error: {e}"
                        );
                    };
                    if let Err(e) = socket.try_send(&bytes) {
                        eprintln!(
                            "Error while trying to send query to upstream resolver. Error: {e}"
                        );
                        message.return_channel.send(None);
                        return;
                    };

                    match socket.recv(&mut receive_buf).await {
                        Err(e) => {
                            eprintln!(
                                "Error while receiving data from upstream resolver. Error. {e}"
                            );
                            message.return_channel.send(None);
                            return;
                        }
                        Ok(v) => v,
                    };

                    // Parse received data into DnsPacket
                    let mut packet_buf = PacketBuffer::from_raw_buffer(receive_buf);
                    let received_packet = match DnsPacket::parse_from(&mut packet_buf) {
                        Err(_) => {
                            eprintln!("Error while parsing received upstream data.");
                            message.return_channel.send(None);
                            return;
                        }
                        Ok(v) => v,
                    };

                    // Send data back to SRS
                    message.return_channel.send(Some(received_packet));
                });
            }
        });
    }

    pub async fn new(
        ies_to_dps_tx: Sender<QueryState>,
        ies_answer_rx: Receiver<QueryState>,
        ies_upstream_resolve_rx: Receiver<HalfDuplexMessage<IesCommand, Option<DnsPacket>>>,
    ) -> Self {
        let socket = Arc::new(UdpSocket::bind(IP_ADDR).await.unwrap());
        Self {
            socket,
            ies_to_dps_tx,
            ies_answer_rx,
            ies_upstream_resolve_rx,
        }
    }
}
