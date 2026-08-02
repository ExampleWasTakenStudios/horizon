mod firewall;
mod trans_proto;

pub use firewall::*;
pub use trans_proto::*;

use crate::constants;

pub fn create_udp_socket() -> tokio::net::UdpSocket {
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
    println!("Bound downstream UDP socket to {:?}", &constants::DOWNSTREAM_SOCKET_ADDR);

    tokio::net::UdpSocket::from_std(socket.into()).unwrap()
}

pub fn create_tcp_listener() -> tokio::net::TcpListener {
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
    socket.listen(constants::DOWNSTREAM_TCP_BACKLOG).unwrap();

    println!(
        "Bound downstream TCP socket to {:?} with backlog of {}",
        constants::DOWNSTREAM_SOCKET_ADDR,
        constants::DOWNSTREAM_TCP_BACKLOG
    );

    tokio::net::TcpListener::from_std(socket.into()).unwrap()
}
