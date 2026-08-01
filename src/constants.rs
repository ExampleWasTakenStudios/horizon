use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

/// Number of tasks spawned that each create a downstream socket.
pub const DOWNSTREAM_SOCKET_TASK_COUNT: u8 = 4;

/// IP Address of the socket that listens to incoming queries from hosts on the network.
const DOWNSTREAM_IP_ADDR: Ipv4Addr = Ipv4Addr::new(0, 0, 0, 0);

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

/// IP address to the socket that connects to the upstream resolver.
///
/// This is the IP address of the socket itself, **NOT** the IP address of the recursive resolver.
const _UPSTREAM_IP_ADDR: Ipv4Addr = Ipv4Addr::new(0, 0, 0, 0);

/// The port of the socket that connects to the upstream resolver.
///
/// This is the port of the socket itself, **NOT** the port of the recursive resolver.
const _UPSTREAM_PORT: u16 = 0;

/// The [`SocketAddr`] of the socket that connects to the upstream resolver.
///
/// This is the [`SocketAddr`] of the socket itself, **NOT** the [`SocketAddr`] of the recursive resolver.
/// The [`SocketAddr`] of the recursive resolver is specified in the constructor of the [`SHS`](crate::srs::StubResolverSystem::new()).
pub const _UPSTREAM_SOCKET_ADDR: SocketAddr =
    SocketAddr::V4(SocketAddrV4::new(_UPSTREAM_IP_ADDR, _UPSTREAM_PORT));

/// The maximum size any given DNS packet may have. Any packets exceeding this packet will be ignored.
///
/// This value should be set to `512` when the service does not support EDNS(0).
/// In case of EDNS(0) support, this value should be set to `1232`.
pub const MAX_PACKET_SIZE: usize = 512;

/// The maximum amount of concurrent UDP queries the system may ever handle.
///
/// Incoming queries are dropped if this number would otherwise be exceeded.
pub const MAX_CONCURRENT_UDP_QUERIES: u16 = 5_000;
