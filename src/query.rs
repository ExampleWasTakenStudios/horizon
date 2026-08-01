use std::net::SocketAddr;

use tokio::sync::{OwnedSemaphorePermit};

use crate::network::TransmissionProtocol;

#[derive(Debug)]
pub struct Query {
    // The OwnedSemaphorePermit ensures a bounded number of concurrent queries.
    // The permit is held for the lifetime of query processing and dropped when processing completes.
    // In other words, this permit is used to ensure the maximum number of concurrent queries.
    // It acts sort of as an ID that acts as prove for your legal status in a country.
    semaphore_permit: OwnedSemaphorePermit,
    trans_proto: TransmissionProtocol,
    origin: SocketAddr,
    buf: Vec<u8>,
}

impl Query {
    pub fn new(
        semaphore_permit: OwnedSemaphorePermit,
        trans_proto: TransmissionProtocol,
        origin: SocketAddr,
        buf: Vec<u8>,
    ) -> Query {
        Query {
            semaphore_permit,
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
