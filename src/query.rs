use std::{net::SocketAddr, sync::Arc, time::SystemTime};

use tokio::net::UdpSocket;

use crate::{
    constants::MAX_PACKET_SIZE,
    protocol::{DnsHeader, DnsRecord, ResponseCode, packet::DnsPacket},
    srs::StubResolverSystem,
};

/// A query that is currently handled by the service.
///
/// This is the overall-wrapping struct for any query currently within the service.
pub struct Query {
    pub query_packet: DnsPacket,
    pub downstream_socket: Arc<UdpSocket>,
    pub upstream_socket: Arc<UdpSocket>,
    pub timestamp: SystemTime,
    // pub cache: Arc<Cache>,
    // pub lza: RwLock<Arc<LocalZoneAuthority>>
    // pub shs: RwLock<Arc<SinkHoleSystem>>
    pub srs: Arc<StubResolverSystem>,
    pub socket_data: SocketData,
}

impl Query {
    /// Create a new query. Calling this method creates a new query which is automatically handled by the service.
    /// Callers should not have to call any other methods to initiate the resolving of the query.
    pub async fn create(
        downstream_socket: Arc<UdpSocket>,
        upstream_socket: Arc<UdpSocket>,
        // cache: Arc<Cache>,
        // shs: RwLock<Arc<SinkHoleSystem>>
        // lza: RwLock<Arc<LocalZoneAuthority>>
        srs: Arc<StubResolverSystem>,
        socket_data: SocketData,
    ) {
        let query_packet = match DnsPacket::parse_from(socket_data.data) {
            Err(_) => return,
            Ok(v) => v,
        };

        let query = Query {
            query_packet,
            downstream_socket,
            upstream_socket,
            timestamp: SystemTime::now(),
            // cache,
            // shs,
            // lza,
            srs,
            socket_data,
        };

        query.run().await;
    }

    async fn run(&self) {
        if !Query::validate_query(&self.query_packet) {
            return; // Drop the query since it is invalid
        }

        // Interrogate cache
        // TODO: implement caching

        // Interrogate LZA

        // Interrogate SHS

        // Command lookup through SHS
        let response_packet = self.command_srs().await;

        let buffer = [0_u8; MAX_PACKET_SIZE];
        if response_packet.to_bytes(buffer).is_ok() {
            let _ = self.downstream_socket.send(&buffer).await;
        }
    }

    async fn command_srs(&self) -> DnsPacket {
        self.srs.resolve(self).await
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

/// Data received from a socket.
pub struct SocketData {
    /// Length of the received data
    pub length: usize,
    /// The [`SocketAddr`] from which the data was received
    pub origin: SocketAddr,
    /// The raw byte array that was received
    pub data: [u8; MAX_PACKET_SIZE],
}
