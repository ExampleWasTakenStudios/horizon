use std::net::SocketAddr;

use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream}, sync::Semaphore};
use tokio_util::{sync::CancellationToken, task::TaskTracker};

use crate::{error::{DnsError::{self, ConnectionSemaphoreClosed, TcpStreamReadError}, DnsResult}, protocol::{self, DnsMessage}};

#[derive(Debug)]
pub struct DnsTcpListener {
    semaphore: Semaphore,
    listener: TcpListener,
    tracker: TaskTracker,
    cancel_token: CancellationToken,
}

impl DnsTcpListener {
    pub fn new(semaphore: Semaphore, tracker: TaskTracker, cancel_token: CancellationToken) -> {
        let listener = socket2::Socket::new(socket2::Domain::IPV4, socket2::Type::STREAM, None).unwrap();
        listener.set_nonblocking(true).unwrap();
        listener.set_reuse_port(true).unwrap();

        let listener = TcpListener::from_std(listener.into()).unwrap();

        Self {
            semaphore,
            listener,
            tracker,
            cancel_token,
        }
    }

    pub async fn listen(&mut self) {
        loop {
            tokio::select! {
                connection = self.try_accept() => {
                    
                }

                _ = self.cancel_token.cancelled() => {
                    println("[TCP Listener] Shutting down.");
                    return;
                }
            }
        }
    }

    async fn try_accept(&mut self) -> DnsResult<TcpConnection> {
        if self.semaphore.available_permits() == 0 {
            return Err(DnsError::NoPermitAvailable);
        }

        let permit = match self.semaphore.try_acquire() {
            Ok(p) => p,
            Err(e) => {
                match e {
                    tokio::sync::TryAcquireError::Closed => {
                        return Err(DnsError::ConnectionSemaphoreClosed);
                    }
                    tokio::sync::TryAcquireError::NoPermits => {
                        return Err(DnsError::NoPermitAvailable);
                    }
                }
            },
        };

        let (stream, peer_addr) = match self.listener.accept().await {
            Ok(v) => v,
            Err(e) => {
                eprintln!("[TCP Listener] Error while attempting to accept an inbound connection: {e}");
                return Err(DnsError::ConnectionAcceptError);
            },
        };

        Ok(TcpConnection::new(stream, peer_addr))
    }
}

#[derive(Debug)]
pub struct TcpConnection {
    stream: TcpStream,
    client_message: Option<DnsMessage>,
    peer_addr: SocketAddr,
}

impl TcpConnection {
    pub fn new(stream: TcpStream, peer_addr: SocketAddr) -> Self {
        Self {
            stream,
            client_message: None,
            peer_addr,
        }
    }

    pub async fn read(&mut self) -> DnsResult<&DnsMessage> {
        if self.client_message.is_some() {
            return Ok(&self.client_message.as_ref().expect("[TCP Connection] Expected self.message to be 'some'."));
        }

        let prefix = match self.stream.read_u16().await {
            Ok(v) => v,
            Err(e) => {
                eprintln!("[TCP Connection] Error while reading DNS message prefix: {e}");
                return Err(DnsError::TcpStreamReadError);
            },
        };

        let mut buf = vec![0_u8; prefix.into()];
        match self.stream.read_exact(&mut buf).await {
            Ok(v) => v,
            Err(e) => {
                eprintln!("[TCP Connection] Error while reading DNS message: {e}");
                return Err(DnsError::TcpStreamReadError);
            },
        };

        let message = protocol::deserialize(&buf)?;
        self.client_message = Some(message);

        Ok(self.client_message.as_ref().expect("[TCP Connection] Deserializing the packet into a message worked, but 'self.message' was 'None'."))
    }

    pub async fn write(&mut self, message: DnsMessage) -> DnsResult<()> {
        let packet: Vec<u8> = Vec::new(); // TODO: serialize message to packet

        match self.stream.write_all(&packet).await {
            Ok(_) => {},
            Err(e) => {
                eprintln!("[TCP Connection] Error while writing packet to TCP stream: {e}");
                return Err(DnsError::TcpStreamWriteError);
            },
        }
        self.stream.write_all(&[0]); // EOF

        Ok(())
    }
}
