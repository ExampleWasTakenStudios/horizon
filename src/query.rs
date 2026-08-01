use std::net::SocketAddr;

use crate::{constants, network::TransmissionProtocol};

#[derive(Debug, Clone)]
pub struct Query {
    trans_proto: TransmissionProtocol,
    origin: SocketAddr,
    buf: [u8; constants::MAX_PACKET_SIZE],
}

impl Query {
    pub fn new(
        trans_proto: TransmissionProtocol,
        origin: SocketAddr,
        buf: [u8; constants::MAX_PACKET_SIZE],
    ) -> Query {
        Query {
            trans_proto,
            origin,
            buf,
        }
    }

    /// Process the query. This is the starting point of a query in the system.
    pub fn process(&self) {
        todo!("query processing");
    }
}
