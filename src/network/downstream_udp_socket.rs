use crate::{constants, network::TransmissionProtocol, query::Query};
use std::{net::SocketAddr, sync::Arc};
use tokio::{net::UdpSocket, sync::OwnedSemaphorePermit};

pub struct DownstreamUdpSocket;

impl DownstreamUdpSocket {
    /// Create a [`tokio::net::UdpSocket`] with the `SO_REUSEPORT` flag set.
    ///
    /// # Panics
    /// This method panics whenever the listener cannot be created.
    /// This is intentional as an application without a downstream UDP
    /// socket is not desirable and we, thus, rather exit the application.
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

    /// Handles incoming UDP datagrams
    pub fn on_recv(
        semaphore_permit: OwnedSemaphorePermit,
        downstream_socket: Arc<UdpSocket>,
        length: usize,
        origin: SocketAddr,
        buf: Vec<u8>,
    ) {
        // Create new task to handle the query and immediately release the receiving task back to the runtime.
        tokio::spawn(async move {
            // If the length exceeds the maximum supported packet size we drop the packet.
            if length > constants::MAX_PACKET_SIZE {
                eprintln!(
                    "  error: received packet exceeded maximum supported size. expected {}; got {}",
                    constants::MAX_PACKET_SIZE,
                    length
                );
                return; // Dropping the packet
            }

            let query = Query::new(
                semaphore_permit,
                TransmissionProtocol::Udp(downstream_socket),
                origin,
                buf,
            );
            query.process().await;
        });
    }
}
