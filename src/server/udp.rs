use tokio::net::UdpSocket;
use tokio_util::{sync::CancellationToken, task::TaskTracker};

use crate::{constants, query};

#[derive(Debug)]
pub struct UdpListener {
    socket: UdpSocket,
    tracker: TaskTracker,
    cancel_token: CancellationToken,
}

impl UdpListener {
    pub fn new(socket: UdpSocket, tracker: TaskTracker, cancel_token: CancellationToken) -> Self {
        Self {
            socket,
            tracker,
            cancel_token,
        }
    }

    pub async fn listen(&self, id: usize) {
        tokio::select! {
            _ = self.cancel_token.cancelled() => {
                println!("[UDP Listener {}] Shutting down gracefully...", id);

                #[allow(clippy::needless_return, reason = "Allowed for readability.")]
                return;
            }

            _ = self.listener_loop(id) => {}
        }
    }

    async fn listener_loop(&self, id: usize) {
        let mut buf = [0; constants::MAX_DGRAM_SIZE];
        println!("[UDP Listener {}] Listening on {}", id, self.socket.local_addr().unwrap());

        loop {
            let (length, peer_addr) = match self.socket.recv_from(&mut buf).await {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("[UDP Listener {}] Error while receiving datagram: {e}", id);
                    continue;
                }
            };

            self.tracker.spawn(async move {
                let mut packet = Vec::with_capacity(length);
                packet.extend_from_slice(&buf[0..length]); // Only pass the bytes actually containing data to the vector. Empty bytes are discarded.

                query::handle(packet, peer_addr);
            });
        }
    }
}
