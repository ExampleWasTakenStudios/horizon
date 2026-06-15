use std::{net::SocketAddr, sync::Arc, time::SystemTime};

use tokio::{net::UdpSocket, sync::Mutex};

use crate::{buffer::PacketBuffer, protocol::{DnsHeader, ResponseCode, packet::DnsPacket}, query_state::QueryState};

struct IngressEgressSystem {
    socket: UdpSocket;
}

impl IngressEgressSystem {
    pub async fn enter(raw_data: [u8; 1232], origin: SocketAddr) {
        let mut buffer = PacketBuffer::<1232>::from_raw_buffer(raw_data);

        let parse_result = DnsPacket::parse_from(&mut buffer);
        let packet = match parse_result {
            Err(e) => {
                return; // We don't have an ID that we can set in the response, hence we drop the query.
            },
            Ok(packet) => packet
        };

        let mut query_state = QueryState::new(packet, origin, SystemTime::now())
    }

    pub async fn exit(&self, packet: DnsPacket, destination: SocketAddr) {
        let raw_data: Arc<Mutex<Vec<u8>>> = Arc::new(Mutex::new(Vec::new()));
        let decode_handle = tokio::task::spawn_blocking(move || {
            let mut raw_data_clone = raw_data.clone();
            let raw_data = match raw_data_clone.lock() {
                Err(e) => {
                    eprintln!("Error while trying to accquire lock on raw data for encoding: {e}");
                    return;
                }
                Ok(raw_data) => raw_data,
            }

            raw_data = IngressEgressSystem::encode(packet)
        })
    }

    fn encode(packet: DnsPacket) -> Vec<u8> {
        // TODO: implement
    }
}
