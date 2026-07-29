use crate::{
    MAX_PACKET_SIZE,
    protocol::{
        DnsHeader, DnsRecord,
        ResponseCode,
        packet::DnsPacket,
    },
    query::Query,
};

pub struct StubResolverSystem;

impl StubResolverSystem {
    pub fn new() -> Self {
        Self {}
    }

    /// Sends the query to the specified upstream resolver.
    pub async fn resolve(&self, query: &Query) -> DnsPacket {
        self.fetch_upstream(query).await
    }

    async fn fetch_upstream(&self, query: &Query) -> DnsPacket {
        if let Err(e) = query.upstream_socket.send(&query.socket_data.data).await {
            eprintln!(
                "Error occurred while trying to forward query to upstream resolver. Error: {e}"
            );
            return self.create_serv_fail_response(&query.query_packet);
        }

        let mut recv_buf = [0_u8; MAX_PACKET_SIZE];

        // Since we will receive all incoming traffic including that designated for other queries,
        // we must check if the incoming response has the ID that we'are looking for and only then process it.
        loop {
            if let Err(e) = query.upstream_socket.recv(&mut recv_buf).await {
                eprintln!(
                    "Error occurred while receiving (or trying to receive) a response from the upstream resolver. Error: {e}"
                );
                return self.create_serv_fail_response(&query.query_packet);
            }

            // Read ID from the received buffer and compare it against the ID of the query
            let response_id = match recv_buf.get(0..12) {
                None => continue,
                Some(v) => {
                    let id: u16 = (v[0] | v[1] | v[2] | v[3]) as u16;
                    id
                }
            };
            if response_id != query.query_packet.header.id {
                continue; // The ID did not match - we continue the loop and listen again
            } else {
                break; // The ID matched - we break out of the loop
            }
        }

        match DnsPacket::parse_from(recv_buf) {
            Err(_) => {
                eprintln!("Error occurred while trying to parse upstream response.");
                self.create_serv_fail_response(&query.query_packet)
            }
            Ok(packet) => packet,
        }
    }

    fn create_upstream_success_response(
        &self,
        query_packet: &DnsPacket,
        upstream_packet: &DnsPacket,
    ) -> DnsPacket {
        todo!();
    }

    fn create_cached_success_response(
        &self,
        query_packet: &DnsPacket,
        rr_set: Vec<DnsRecord>,
    ) -> DnsPacket {
        let header = DnsHeader {
            response_code: ResponseCode::NOERROR,
            is_response: true,
            is_truncated: false,
            is_recursion_avail: true,
            question_count: 0,
            answer_count: rr_set.len() as u16,
            authoritative_count: 0,
            additional_count: 0,
            ..query_packet.header
        };



        DnsPacket {
            header,
            questions: Vec::new(),
            answers: rr_set.clone(),
            authoritatives: Vec::new(),
            additionals: Vec::new(),
        }
    }

    fn create_serv_fail_response(&self, query_packet: &DnsPacket) -> DnsPacket {
        let header = DnsHeader {
            is_response: true,
            response_code: ResponseCode::SERVFAIL,
            is_truncated: false,
            is_recursion_avail: true,
            question_count: 0,
            answer_count: 0,
            authoritative_count: 0,
            additional_count: 0,
            ..query_packet.header
        };

        DnsPacket {
            header,
            questions: Vec::new(),
            answers: Vec::new(),
            authoritatives: Vec::new(),
            additionals: Vec::new(),
        }
    }
}
