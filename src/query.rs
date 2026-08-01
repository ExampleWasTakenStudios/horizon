use std::net::SocketAddr;

use crate::network::TransmissionProtocol;

#[derive(Debug, Clone)]
pub struct Query {
    trans_proto: TransmissionProtocol,
    origin: SocketAddr,
    buf: Vec<u8>,
}

impl Query {
    pub fn new(trans_proto: TransmissionProtocol, origin: SocketAddr, buf: Vec<u8>) -> Query {
        Query {
            trans_proto,
            origin,
            buf,
        }
    }

    /// Process the query. This is the starting point of a query in the system.
    pub async fn process(&self) {
        todo!("query processing");
    }
}
