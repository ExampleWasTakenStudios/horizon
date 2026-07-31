use crate::constants;

pub struct DownstreamTcpListener;

impl DownstreamTcpListener {
    pub fn create() -> tokio::net::TcpListener {
        let socket = socket2::Socket::new(
            socket2::Domain::IPV4,
            socket2::Type::STREAM,
            Some(socket2::Protocol::TCP),
        )
        .unwrap();

        socket.set_nonblocking(true).unwrap();
        socket.set_reuse_port(true).unwrap();

        socket
            .bind(&constants::DOWNSTREAM_SOCKET_ADDR.into())
            .unwrap();

        tokio::net::TcpListener::from_std(socket.into()).unwrap()
    }
}
