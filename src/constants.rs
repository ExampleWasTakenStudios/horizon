use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

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

/// IP address to the socket that connects to the upstream resolver.
///
/// This is the IP address of the socket itself, **NOT** the IP address of the recursive resolver.
const UPSTREAM_IP_ADDR: Ipv4Addr = Ipv4Addr::new(0,0,0,0);

/// The port of the socket that connects to the upstream resolver.
///
/// This is the port of the socket itself, **NOT** the port of the recursive resolver.
const UPSTREAM_PORT: u16 = 0;

/// The [`SocketAddr`] of the socket that connects to the upstream resolver.
///
/// This is the [`SocketAddr`] of the socket itself, **NOT** the [`SocketAddr`] of the recursive resolver.
/// The [`SocketAddr`] of the recursive resolver is specified in the constructor of the [`SHS`](crate::srs::StubResolverSystem::new()).
pub const UPSTREAM_SOCKET_ADDR: SocketAddr =
    SocketAddr::V4(SocketAddrV4::new(UPSTREAM_IP_ADDR, UPSTREAM_PORT));

/// The maximum size any given DNS packet may have. Any packets exceeding this packet will be ignored.
///
/// This value should be set to `512` when the service does not support EDNS(0).
/// In case of EDNS(0) support, this value should be set to `1232`.
pub const MAX_PACKET_SIZE: usize = 512;
