use std::sync::Arc;

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
    pub async fn run(mut self) {
        println!("Starting IES...");

        let socket_ingress_clone = self.socket.clone();
        let socket_egress_clone = self.socket.clone();

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
        tokio::spawn(async move {
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

        // This task handles the query forwarding to an upstream resolver and returns it to the SRS.
        tokio::spawn(async move {
            println!("Starting upstream forward task...");
            loop {
                let message = match self.ies_upstream_resolve_rx.recv().await {
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

                // Spawn socket
                tokio::spawn(async move {
                    let mut receive_buf = [0_u8; MAX_PACKET_SIZE];
                    let socket = match tokio::net::UdpSocket::bind("0.0.0.0:0").await {
                        Err(e) => {
                            eprintln!("Error while trying to bind resolving socket. Error: {e}");
                            if message.return_channel.send(None).is_err() {
                                eprintln!(
                                    "Failed to bind to upstream socket but could not notify SRS through return channel."
                                );
                            }
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
                        if message.return_channel.send(None).is_err() {
                            eprintln!(
                                "Failed to notify SRS of socket failure through return channel."
                            );
                        };
                        return;
                    };

                    match socket.recv(&mut receive_buf).await {
                        Err(e) => {
                            eprintln!(
                                "Error while receiving data from upstream resolver. Error. {e}"
                            );
                            if message.return_channel.send(None).is_err() {
                                eprintln!(
                                    "Failed to notify SRS of socket receiving failure through return channel."
                                );
                            };
                            return;
                        }
                        Ok(v) => v,
                    };

                    // Parse received data into DnsPacket
                    let mut packet_buf = PacketBuffer::from_raw_buffer(receive_buf);
                    let received_packet = match DnsPacket::parse_from(&mut packet_buf) {
                        Err(_) => {
                            eprintln!("Error while parsing received upstream data.");
                            if message.return_channel.send(None).is_err() {
                                eprintln!(
                                    "Failed to notify SRS of parsing error through return channel."
                                );
                            };
                            return;
                        }
                        Ok(v) => v,
                    };

                    // Send data back to SRS
                    if message.return_channel.send(Some(received_packet)).is_err() {
                        eprintln!("Failed send received query back to SRS through return channel.");
                    };
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
