use crate::{
    constants,
    network::{Firewall, TransmissionProtocol},
    query::Query,
};
use std::{net::SocketAddr, time::Duration};
use tokio::{io::AsyncReadExt, net::TcpStream, sync::OwnedSemaphorePermit, time::timeout};

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

    pub fn on_recv(
        semaphore_permit: OwnedSemaphorePermit,
        mut stream: TcpStream,
        origin: SocketAddr,
    ) {
        // Create a new task to handle the query and immediately release the receiving task back to the runtime
        tokio::spawn(async move {
            let dns_buf = match DownstreamTcpListener::read_packet(&mut stream).await {
                None => return,
                Some(dns_buf) => dns_buf,
            };

            if !Firewall::verify_query(&dns_buf, dns_buf.len()) {
                eprintln!(
                    "  warning: received invalid TCP stream from {}",
                    origin.ip()
                );
                return;
            }

            let query = Query::new(
                semaphore_permit,
                TransmissionProtocol::Tcp(stream),
                origin,
                dns_buf,
            );
            query.process().await;
        });
    }

    /// Read an entire DNS packet from a [`TcpStream`].
    ///
    /// This method also enforces that the packet be sent within 2 seconds to prevent slowloris attacks.
    async fn read_packet(stream: &mut TcpStream) -> Option<Vec<u8>> {
        // Read the length prefix that TCP DNS messages carry as defined in
        // RFC 1035 Section 4.2.2 <https://datatracker.ietf.org/doc/html/rfc1035#section-4.2.2>
        let mut prefix_buf = [0_u8; 2];

        // The length of the DNS packet announced by `prefix_buf`
        let mut length: usize = 0;

        // This is the buffer that will contain the actual DNS data
        let mut dns_buf = Vec::<u8>::new();

        if timeout(Duration::from_secs(2), async {
            if let Err(e) = stream.read_exact(&mut prefix_buf).await {
                eprintln!("  error while reading TCP length prefix: {e}");
                return;
            };

            length = ((prefix_buf[0] as usize) << 8) | prefix_buf[1] as usize;
            dns_buf = vec![0; length];

            if let Err(e) = stream.read_exact(&mut dns_buf).await {
                eprintln!("  error while reading TCP DNS buffer: {e}");
            }
        })
        .await
        .is_err()
        {
            eprintln!(
                "  error: full TCP DNS packet was not received in the legal time frame (2 seconds)"
            );
            return None;
        }

        Some(dns_buf)
    }
}
