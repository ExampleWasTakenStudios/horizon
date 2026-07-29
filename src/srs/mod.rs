use std::sync::Arc;

use tokio::{
    net::{UdpSocket, unix::SocketAddr},
    task::JoinHandle,
};

use crate::{
    MAX_PACKET_SIZE,
    cache::{cache_entry::RRSet, ticket_guard::CacheTicketGuard},
    protocol::{
        DnsHeader, DnsRecord,
        ResponseCode::{self, NOERROR},
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
        let question = match query.query_packet.questions.first() {
            None => {
                eprint!("Query doese not contain a question.");
                return self.create_serv_fail_response(&query.query_packet);
            }
            Some(question) => question,
        };

        match query.cache.register_ticket(question.clone()) {
            Err(mut sender) => {
                println!("Found an already registered ticket. Subscribing to sender instead.");
                match sender.recv().await {
                    Err(e) => {
                        eprintln!("Error on the sender: {e}");
                        return self.create_serv_fail_response(&query.query_packet);
                    }
                    Ok(rr_set) => {
                        return self.create_cached_success_response(&query.query_packet, rr_set);
                    }
                }
            }
            Ok(ticket_guard) => {
                let upstream_packet = self.fetch_upstream(query, &ticket_guard).await;
                query
                    .cache
                    .commit(question.clone(), upstream_packet.answers.clone());
                return upstream_packet;
            }
        }
    }

    async fn fetch_upstream(&self, query: &Query, ticket_guard: &CacheTicketGuard) -> DnsPacket {
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

            let response_id = match recv_buf.get(0..12) {
                None => continue,
                Some(v) => {
                    let id: u16 = (v[0] | v[1] | v[2] | v[3]) as u16;
                    id
                }
            };
            if response_id != query.query_packet.header.id {
                continue;
            } else {
                break;
            }
        }

        match DnsPacket::parse_from(recv_buf) {
            Err(e) => {
                eprintln!("Error occurred while trying to parse upstream response.");
                return self.create_serv_fail_response(&query.query_packet);
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

        let packet = DnsPacket {
            header,
            questions: Vec::new(),
            answers: rr_set.clone(),
            authoritatives: Vec::new(),
            additionals: Vec::new(),
        };

        packet
    }

    fn create_serv_fail_response(&self, query_packet: &DnsPacket) -> DnsPacket {
        todo!("creating server failure responses is not yet implemented.");
    }
}
