use std::{net::SocketAddr, sync::Arc, time::SystemTime};

use tokio::net::UdpSocket;

use crate::MAX_PACKET_SIZE;

/// A query that is currently handled by the service.
///
/// This is the overall-wrapping struct for any query currently within the service.
pub struct Query {
    downstream_socket: Arc<UdpSocket>,
    upstream_socket: Arc<UdpSocket>,
    timestamp: SystemTime,
    // cache: Arc<Cache>,
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
        // cache: Arc<Cache>,
        // shs: RwLock<Arc<SinkHoleSystem>>
        // lza: RwLock<Arc<LocalZoneAuthority>>
        socket_data: (usize, SocketAddr, [u8; MAX_PACKET_SIZE]),
    ) {
        let query = Query {
            downstream_socket,
            upstream_socket,
            timestamp: SystemTime::now(),
            // cache,
            // shs,
            // lza,
            socket_data,
        };

        query.run().await;
    }

    async fn run(&self) {}
}
