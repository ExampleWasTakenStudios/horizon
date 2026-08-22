use tokio::net::UdpSocket;
use tokio_util::task::TaskTracker;

use crate::{constants, query};

#[derive(Debug)]
pub struct UdpListener {
    socket: UdpSocket,
    task_tracker: TaskTracker,
}

impl UdpListener {
    pub fn new(socket: UdpSocket, task_tracker: TaskTracker) -> Self {
        Self {
            socket,
            task_tracker,
        }
    }

    pub async fn listen(&self) {
        let mut buf = [0; constants::MAX_DGRAM_SIZE];

        loop {
            let (length, peer_addr) = match self.socket.recv_from(&mut buf).await {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("Error while receiving datagram: {e}");
                    continue;
                }
            };

            self.task_tracker.spawn(async move {
                let mut packet = Vec::with_capacity(length);
                packet.extend_from_slice(&buf[0..length]); // Only pass the bytes actually containing data to the vector. Empty bytes are discarded.

                query::handle(packet, peer_addr);
            });
        }
    }
}
