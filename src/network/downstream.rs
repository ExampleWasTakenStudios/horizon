use std::{net::SocketAddr, sync::Arc, time::Duration};

use tokio::{
    io::AsyncReadExt,
    net::{TcpListener, TcpStream, UdpSocket},
    sync::Semaphore,
    time::timeout,
};

use crate::{
    constants,
    network::{Firewall, TransmissionProtocol},
    query::Query,
};

#[derive(Debug)]
pub struct DnsUdpSocket {
    socket: Arc<UdpSocket>,
}

impl DnsUdpSocket {
    pub fn new() -> Self {
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
        println!(
            "Bound downstream UDP socket to {:?}",
            &constants::DOWNSTREAM_SOCKET_ADDR
        );

        Self {
            socket: Arc::new(UdpSocket::from_std(socket.into()).unwrap()),
        }
    }

    /// Continuously listens to the socket and processes incoming queries.
    pub async fn listen(&self) -> DnsUdpPacket {
        loop {
            let mut buf = vec![0; constants::MAX_PACKET_SIZE];
            let (length, origin) = match self.socket.recv_from(&mut buf).await {
                Err(e) => {
                    eprintln!("  error while receiving downstream UDP traffic: {e}");
                    continue;
                }
                Ok(v) => v,
            };
            buf.truncate(length);

            return DnsUdpPacket::new(self.socket.clone(), origin, buf);
        }
    }
}

#[derive(Debug)]
pub struct DnsUdpPacket {
    socket: Arc<UdpSocket>,
    peer_addr: SocketAddr,
    buf: Vec<u8>,
}

impl DnsUdpPacket {
    pub fn new(socket: Arc<UdpSocket>, peer_addr: SocketAddr, buf: Vec<u8>) -> Self {
        Self {
            socket,
            peer_addr,
            buf,
        }
    }

    pub async fn process(self, semaphore: Arc<Semaphore>) {
        if !Firewall::verify_query(&self.buf) {
            eprintln!("  warning: firewall rejected DGRAM from {}", &self.peer_addr.ip());
            return;
        }

        let permit = match semaphore.clone().try_acquire_owned() {
            Err(e) => match e {
                tokio::sync::TryAcquireError::Closed => {
                    panic!(
                        "Query Semaphore is closed. No new permits can be offered. Unrecoverable state."
                    );
                }
                tokio::sync::TryAcquireError::NoPermits => {
                    eprintln!(
                        "  warning: max. number of concurrent queries reached. Dropping query..."
                    );
                    return;
                }
            },
            Ok(permit) => permit,
        };

        let query = Query::new(
            permit,
            TransmissionProtocol::Udp(self.socket.clone()),
            self.peer_addr,
            self.buf,
        );
        query.process().await;
    }
}

#[derive(Debug)]
pub struct DnsTcpListener {
    listener: TcpListener,
}

impl DnsTcpListener {
    pub fn new() -> Self {
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

        Self {
            listener: tokio::net::TcpListener::from_std(socket.into()).unwrap(),
        }
    }

    /// Continuously waits for a connection on the socket. Once a connection has been established, it returns a [`DnsTcpStream`].
    pub async fn accept(&self) -> DnsTcpStream {
        loop {
            if let Ok((stream, addr)) = self.listener.accept().await {
                return DnsTcpStream::new(stream, addr);
            }
        }
    }
}

#[derive(Debug)]
pub struct DnsTcpStream {
    stream: TcpStream,
    peer_addr: SocketAddr,
}

impl DnsTcpStream {
    pub fn new(stream: TcpStream, peer_addr: SocketAddr) -> Self {
        Self { stream, peer_addr }
    }

    /// Reads exactly ONE DNS packet and processes it.
    ///
    /// # *Important*
    /// Query Pipelining is **NOT** not yet implemented! Therefore, this method will read exactly ***ONE*** DNS packet and close the stream once a response has been sent.
    /// Implementation is however planned to be implemented at a later date.
    pub async fn read_and_process(mut self, semaphore: Arc<Semaphore>) {
        let permit = match semaphore.clone().try_acquire_owned() {
            Err(e) => match e {
                tokio::sync::TryAcquireError::Closed => {
                    panic!(
                        "Query Semaphore is closed. No new permits can be offered. Unrecoverable state."
                    );
                }
                tokio::sync::TryAcquireError::NoPermits => {
                    eprintln!(
                        "  warning: max. number of concurrent queries reached. Dropping query..."
                    );
                    return;
                }
            },
            Ok(permit) => permit,
        };

        let dns_buf = match self.read_packet().await {
            None => return,
            Some(v) => v,
        };

        if !Firewall::verify_query(&dns_buf) {
            eprintln!(
                "  warning: received invalid TCP stream from {}",
                self.peer_addr.ip()
            );
            return;
        }

        let query = Query::new(
            permit,
            TransmissionProtocol::Tcp(self.stream),
            self.peer_addr,
            dns_buf,
        );
        query.process().await;
    }

    /// Read an entire DNS packet from a [`TcpStream`].
    ///
    /// This method also enforces that the packet be sent within 2 seconds to prevent slowloris attacks.
    ///
    /// # Return
    /// The returned vector has the exact size of the number of bytes read.
    async fn read_packet(&mut self) -> Option<Vec<u8>> {
        // Read the length prefix that TCP DNS messages carry as defined in
        // RFC 1035 Section 4.2.2 <https://datatracker.ietf.org/doc/html/rfc1035#section-4.2.2>
        let mut prefix_buf = [0_u8; 2];

        // The length of the DNS packet announced by `prefix_buf`
        let mut length: usize = 0;

        // This is the buffer that will contain the actual DNS data
        let mut dns_buf = Vec::<u8>::new();

        if timeout(Duration::from_secs(2), async {
            if let Err(e) = self.stream.read_exact(&mut prefix_buf).await {
                eprintln!("  error while reading TCP length prefix: {e}");
                return;
            };

            length = ((prefix_buf[0] as usize) << 8) | prefix_buf[1] as usize;
            dns_buf = vec![0; length];

            if let Err(e) = self.stream.read_exact(&mut dns_buf).await {
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
