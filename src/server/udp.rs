use std::sync::Arc;

use tokio::{
    net::UdpSocket,
    task::JoinSet,
};

use crate::{constants, worker::WorkerPool};

#[derive(Debug)]
pub struct Listener {
    socket: UdpSocket,
    worker_pool: WorkerPool,
    active_queries: JoinSet<()>,
}

impl Listener {
    /// Run the server
    ///
    /// Listen for inbound datagrams and feed them into the worker pool.
    /// The worker pool attempts to provide a free worker to handle the datagram. If no worker is available the datagram is dropped.
    async fn run(&self, socket: UdpSocket, worker_pool: WorkerPool) {
        loop {
            let mut buf = [0_u8; constants::MAX_PACKET_SIZE];

            // Waiting for a datagram to arrive at the socket
            let (length, peer_addr) = match socket.recv_from(&mut buf).await {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("Error while receiving datagram: {e}");
                    continue;
                }
            };

            // Requesting a free worker from the worker pool. If no worker is available, we drop the datagram by continuing the loop
            todo!("request a free worker from the worker pool");
        }
    }
}
