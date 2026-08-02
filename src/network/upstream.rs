use std::{collections::VecDeque, ops::Deref, sync::Arc, sync::Mutex};

use tokio::net::UdpSocket;

use crate::constants;

pub struct UpstreamUdpSocketPool {
    pool: Arc<Mutex<VecDeque<Arc<UdpSocket>>>>,
}

impl UpstreamUdpSocketPool {
    pub fn new() -> Self {
        let mut pool = VecDeque::with_capacity(constants::UPSTREAM_UDP_SOCKET_POOL_SIZE as usize);

        println!("Initializing Upstream UDP Socket Pool...");

        // TODO: this initialization may be multi-threaded to reduce long initialization times
        for _ in 0..constants::UPSTREAM_UDP_SOCKET_POOL_SIZE {
            let socket = std::net::UdpSocket::bind("0.0.0.0:0").unwrap();
            let _ = socket.set_nonblocking(true);
            let socket = UdpSocket::from_std(socket).unwrap();

            pool.push_back(Arc::new(socket));
        }

        println!("Initialized Upstream UDP Socket Pool");

        Self {
            pool: Arc::new(Mutex::new(pool)),
        }
    }

    pub fn request_socket(&self) -> Option<UpstreamUdpSocketGuard> {
        match self.pool.lock() {
            Err(_) => None,
            Ok(mut pool) => pool
                .pop_front()
                .map(|socket| UpstreamUdpSocketGuard::new(socket, self.pool.clone())),
        }
    }
}

pub struct UpstreamUdpSocketGuard {
    socket: Arc<UdpSocket>,
    pool: Arc<Mutex<VecDeque<Arc<UdpSocket>>>>,
}

impl UpstreamUdpSocketGuard {
    pub fn new(
        socket: Arc<UdpSocket>,
        free_socket_queue: Arc<Mutex<VecDeque<Arc<UdpSocket>>>>,
    ) -> Self {
        Self {
            socket,
            pool: free_socket_queue,
        }
    }
}

impl Deref for UpstreamUdpSocketGuard {
    type Target = UdpSocket;

    fn deref(&self) -> &Self::Target {
        &self.socket
    }
}

impl Drop for UpstreamUdpSocketGuard {
    fn drop(&mut self) {
        match self.pool.lock() {
            // We're forcing the socket back into the pool even if the dropping thread panicked.
            Err(e) => {
                e.into_inner().push_back(self.socket.clone());
            }
            Ok(mut pool) => pool.push_back(self.socket.clone()),
        }
    }
}
