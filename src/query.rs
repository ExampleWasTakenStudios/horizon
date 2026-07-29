use std::{net::SocketAddr, sync::Arc, time::SystemTime};

use tokio::net::UdpSocket;

use crate::{
    MAX_PACKET_SIZE,
    buffer::PacketBuffer,
    cache::Cache,
    protocol::{DnsHeader, DnsRecord, ResponseCode, packet::DnsPacket},
    query,
};

/// A query that is currently handled by the service.
///
/// This is the overall-wrapping struct for any query currently within the service.
pub struct Query {
    downstream_socket: Arc<UdpSocket>,
    upstream_socket: Arc<UdpSocket>,
    timestamp: SystemTime,
    cache: Arc<Cache>,
    // shs: RwLock<Arc<SinkHoleSystem>>
    // lza: RwLock<Arc<LocalZoneAuthority>>
    socket_data: (usize, SocketAddr, [u8; MAX_PACKET_SIZE]),
}

impl Query {
    /// Create a new query. Calling this method creates a new query which is automatically handled by the service.
    /// Callers should not have to call any other methods to initiate the resolving of the query.
    pub async fn create(
        downstream_socket: Arc<UdpSocket>,
        upstream_socket: Arc<UdpSocket>,
        cache: Arc<Cache>,
        // shs: RwLock<Arc<SinkHoleSystem>>
        // lza: RwLock<Arc<LocalZoneAuthority>>
        socket_data: (usize, SocketAddr, [u8; MAX_PACKET_SIZE]),
    ) {
        let query = Query {
            downstream_socket,
            upstream_socket,
            timestamp: SystemTime::now(),
            cache,
            // shs,
            // lza,
            socket_data,
        };

        query.run().await;
    }

    async fn run(&self) {
        let (_length, _socket_addr, buf) = self.socket_data;
        let packet = match DnsPacket::parse_from(buf) {
            Err(_) => return,
            Ok(v) => v,
        };

        if !Query::validate_query(&packet) {
            return; // Drop the query since it is invalid
        }

        // Interrogate cache
        let question = match packet.questions.first() {
            None => {
                eprintln!(
                    "[Query] Packet contained no queries. Note, this should be an impossible state. The packet should be validated to be a valid query before being used. See `Query::validate_query()`"
                );
                return;
            }
            Some(v) => v,
        };

        if let Some(rr_set) = self.cache.check_for(question).await {
            let answer_packet = self.create_answer(&packet, rr_set);

            let buf = answer_packet.to_raw_bytes().unwrap();
            let _ = self.downstream_socket.send_to(&buf, _socket_addr).await;
            return;
        }


        // Interrogate LZA

        // Interrogate SHS

        // Command lookup through SHS
    }

    fn create_answer(&self, query_packet: &DnsPacket, answer_rr: Vec<DnsRecord>) -> DnsPacket {
        let header = DnsHeader {
            is_response: true,
            is_truncated: false,
            is_recursion_avail: true,
            response_code: ResponseCode::NOERROR,
            answer_count: answer_rr.len() as u16,
            authoritative_count: 0,
            additional_count: 0,
            ..query_packet.header
        };

        DnsPacket {
            header,
            questions: Vec::new(),
            answers: answer_rr,
            authoritatives: Vec::new(),
            additionals: Vec::new(),
        }
    }

    /// Validate if the packet contains a valid query
    fn validate_query(packet: &DnsPacket) -> bool {
        // The following if statements are separate for readability
        if packet.questions.len() != 1 {
            return false;
        }
        if packet.header.is_response {
            return false;
        }

        true
    }
}
