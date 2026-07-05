use std::net::SocketAddr;

use tokio::sync::mpsc::{Receiver, Sender};

use crate::{
    MAX_PACKET_SIZE, buffer::PacketBuffer, isc::HalfDuplexMessage, protocol::packet::DnsPacket,
    query_state::QueryState,
};

/// Data structure used to send data between the socket manager and the IES.
pub struct NetworkTrafficMessage {
    origin: SocketAddr,
    data: [u8; MAX_PACKET_SIZE],
}

pub struct IngressEgressSystem;

impl IngressEgressSystem {
    pub async fn run(
        mut incoming_rx: Receiver<NetworkTrafficMessage>,
        dps_tx: Sender<QueryState>,
        mut srs_resolving_rx: Receiver<HalfDuplexMessage<DnsPacket, Option<DnsPacket>>>,
        mut answer_rx: Receiver<QueryState>,
        outgoing_tx: Sender<NetworkTrafficMessage>,
    ) {
        // This task handles all incoming queries.
        tokio::spawn(async move {
            loop {
                let incoming_message = match incoming_rx.recv().await {
                    None => {
                        panic!("IES RX channel to receive incoming traffic closed unexpectedly.")
                    }
                    Some(v) => v,
                };

                // Parse bytes to DnsPacket
                let mut packet_buf = PacketBuffer::from_raw_buffer(incoming_message.data);
                let packet = match DnsPacket::parse_from(&mut packet_buf) {
                    Err(_) => continue,
                    Ok(packet) => packet,
                };

                // Create QueryState
                let query_state = QueryState::new(incoming_message.origin, packet);

                // Send DnsPacket to DPS
                dps_tx.send(query_state).await;
            }
        });

        // This task receives answered query states and sends them back to the querying client.
        tokio::spawn(async move {
            loop {
                let received_query_state = match answer_rx.recv().await {
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

                // Send data to socket
                outgoing_tx
                    .send(NetworkTrafficMessage {
                        origin: *received_query_state.get_origin(),
                        data: bytes,
                    })
                    .await;
            }
        });

        // This task handles the query forwarding to an upstream resolver and returns it to the SRS.
        tokio::spawn(async move {
            let message = match srs_resolving_rx.recv().await {
                None => {
                    panic!("IES-SRS resolving channel closed unexpectedly.");
                }
                Some(v) => v,
            };

            // Translate packet into bytes
            let bytes = match message.payload.to_raw_bytes() {
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

                socket.connect("1.1.1.1:53").await;
                match socket.try_send(&bytes) {
                    Err(e) => {
                        eprintln!("Error while trying to send query to upstream resolver. Error: {e}");
                        message.return_channel.send(None);
                        return;
                    },
                    _ => ()
                };

                match socket.recv(&mut receive_buf).await {
                    Err(e) => {
                        eprintln!("Error while receiving data from upstream resolver. Error. {e}");
                        message.return_channel.send(None);
                        return;
                    },
                    Ok(v) => v,
                };

                // Parse received data into DnsPacket
                let mut packet_buf = PacketBuffer::from_raw_buffer(receive_buf);
                let received_packet = match DnsPacket::parse_from(&mut packet_buf) {
                    Err(_) => {
                        eprintln!("Error while parsing received upstream data.");
                        message.return_channel.send(None);
                        return;
                    },
                    Ok(v) => v,
                };

                // Send data back to SRS
                message.return_channel.send(Some(received_packet));
            });
        });
    }
}
