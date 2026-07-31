use crate::constants;

pub struct DownstreamUdpSocket;

impl DownstreamUdpSocket {
    /// Create a [`tokio::net::UdpSocket`] with the `SO_REUSEPORT` flag set.
    ///
    /// # Panics
    /// This
    pub fn create() -> tokio::net::UdpSocket {
        println!("Creating downstream UDP socket...");
        let socket = socket2::Socket::new(
            socket2::Domain::IPV4,
            socket2::Type::DGRAM,
            Some(socket2::Protocol::UDP),
        )
        .unwrap();

        socket.set_reuse_port(true).unwrap();
        socket.set_nonblocking(true).unwrap();
        socket
            .bind(&constants::DOWNSTREAM_SOCKET_ADDR.into())
            .unwrap();
        println!("Bound socket to {:?}", &constants::DOWNSTREAM_SOCKET_ADDR);

        tokio::net::UdpSocket::from_std(socket.into()).unwrap()
    }
}
