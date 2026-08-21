use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

/// Number of tasks spawned that each create a downstream socket.
pub const DOWNSTREAM_SOCKET_TASK_COUNT: u8 = 4;

/// Maximum number of concurrent active UDP queries.
pub const DOWNSTREAM_UDP_WORKER_POOL_SIZE: usize = 300;

/// Specifies how many sockets are created for the Upstream UDP Socket Pool.
pub const UPSTREAM_UDP_SOCKET_POOL_SIZE: u16 = 2_000;

/// Specifies how many sockets are created for the Upstream TCP Socket Pool.
pub const UPSTREAM_TCP_SOCKET_POOL_SIZE: u16 = 500;

/// IP Address of the socket that listens to incoming queries from hosts on the network.
const DOWNSTREAM_IP_ADDR: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);

/// Port of th socket that listens to incoming queries from hosts on the network.
/// This should be set to port 53 in production environments. (Elevated privilege required!)
const DOWNSTREAM_PORT: u16 = 1234;

/// [`SocketAddr`] of the socket that listens to incoming queries from hosts on the network.
///
/// See [`self`]
pub const DOWNSTREAM_SOCKET_ADDR: SocketAddr =
    SocketAddr::V4(SocketAddrV4::new(DOWNSTREAM_IP_ADDR, DOWNSTREAM_PORT));

/// Specifies how many unaccepted handshakes the OS should queue in memory before rejecting new clients.
pub const DOWNSTREAM_TCP_BACKLOG: i32 = 1024;

/// The maximum size any given DNS packet that is received over UDP may have. Any packets exceeding this packet will be ignored.
///
/// This value should be set to `512` when the service does not support EDNS(0).
/// In case of EDNS(0) support, this value should be set to `1232`.
/// This value does not apply to DNS messages received over TCP.
pub const MAX_DGRAM_SIZE: usize = 512;

/// The maximum amount of concurrent UDP queries the system may ever handle.
///
/// Incoming queries are dropped if this number would otherwise be exceeded.
pub const MAX_CONCURRENT_ACTIVE_QUERIES: usize = 6_000;
