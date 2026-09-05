use crate::{error::DnsResult, protocol::DnsMessage, server::TcpConnection};

#[derive(Debug)]
pub enum ClientConnection {
    Tcp(TcpConnection),
}

impl ClientConnection {
    pub async fn read(&mut self) -> DnsResult<&DnsMessage> {
        match self {
            ClientConnection::Tcp(conn) => conn.read().await
        }
    }

    pub async fn write(&mut self, message: DnsMessage) -> DnsResult<()> {
        match self {
            ClientConnection::Tcp(conn) => conn.write(message).await
        }
    }
}

#[derive(Debug)]
pub struct NetworkHandler {

}

impl NetworkHandler {
    pub async fn handle(connection: ClientConnection) {
        let message = match connection.read().await {
            Ok(m) => m,
            Err(e) => {},
        }
    }
}
