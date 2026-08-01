use std::net::SocketAddr;

use tokio::{io::AsyncReadExt, net::TcpStream};

use crate::{constants, query::Query};

pub struct DownstreamTcpListener;

impl DownstreamTcpListener {
    /// Create a [`tokio::net::UdpSocket`] with the `SO_REUSEPORT` flag set.
    ///
    /// # Panics
    /// This method panics whenever the listener cannot be created.
    /// This is intentional as an application without a downstream TCP
    /// listener is not desirable and we, thus, rather exit the application.
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
        socket.listen(constants::DOWNSTREAM_TCP_BACKLOG).unwrap();

        println!(
            "Bound downstream TCP socket to {:?} with backlog of {}",
            constants::DOWNSTREAM_SOCKET_ADDR,
            constants::DOWNSTREAM_TCP_BACKLOG
        );

        tokio::net::TcpListener::from_std(socket.into()).unwrap()
    }

    pub async fn on_recv(mut stream: TcpStream, origin: SocketAddr) {
        // TODO: rate limiting and attack mitigation

        // Create a new task to handle the query and immediately release the receiving task back to the runtime
        tokio::spawn(async move {
            // Read the length prefix that TCP DNS messages carry as defined in
            // RFC 1035 Section 4.2.2 <https://datatracker.ietf.org/doc/html/rfc1035#section-4.2.2>
            let mut prefix_buf = [0_u8, 2];
            if let Err(e) = stream.read_exact(&mut prefix_buf).await {
                eprintln!("  error while reading TCP length prefix: {e}");
                return;
            };
            let length: u16 = (prefix_buf[0] << 8) as u16 | prefix_buf[1] as u16;

            // This is the buffer that will contain the actual DNS data
            let mut dns_buf = Vec::<u8>::with_capacity(length as usize);
            if let Err(e) = stream.read_exact(&mut dns_buf).await {
                eprintln!("  error while reading TCP DNS buffer: {e}");
                return;
            }

            let query = Query::new(super::TransmissionProtocol::Tcp, origin, dns_buf);
            query.process().await;
        });
    }
}
