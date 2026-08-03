/// This module is mostly for organizational and structural reasons. E.g. to easily collapse the UDP section of the [`crate::network::upstream`] module.
pub mod udp {
    use crate::constants;
    use std::{
        collections::VecDeque,
        ops::{Deref, DerefMut},
        sync::{Arc, Mutex},
    };
    use tokio::net::UdpSocket;

    pub struct UpstreamUdpSocketPool {
        pool: Arc<Mutex<VecDeque<Arc<UdpSocket>>>>,
    }

    impl UpstreamUdpSocketPool {
        pub fn new() -> Self {
            let mut pool =
                VecDeque::with_capacity(constants::UPSTREAM_UDP_SOCKET_POOL_SIZE as usize);

            println!("Initializing upstream UDP socket pool...");

            // TODO: this initialization may be multi-threaded to reduce long initialization times
            for _ in 0..constants::UPSTREAM_UDP_SOCKET_POOL_SIZE {
                let socket = std::net::UdpSocket::bind("0.0.0.0:0").unwrap();
                socket.set_nonblocking(true).unwrap();
                let socket = UdpSocket::from_std(socket).unwrap();

                pool.push_back(Arc::new(socket));
            }

            println!("Initialized upstream UDP socket pool");

            Self {
                pool: Arc::new(Mutex::new(pool)),
            }
        }

        pub fn request_socket(&self) -> Option<UpstreamUdpSocketGuard> {
            let mut pool = match self.pool.lock() {
                Err(e) => {
                    eprintln!(
                        "  warning: upstream UDP socket pool was poisoned. forcing recovery."
                    );
                    e.into_inner()
                }
                Ok(pool) => pool,
            };

            pool.pop_front()
                .map(|socket| UpstreamUdpSocketGuard::new(socket, self.pool.clone()))
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
}

/// This module is mostly for organizational and structural reasons. E.g. to easily collapse the UDP section of the [`crate::network::upstream`] module.
pub mod tcp {
    use std::{
        net::SocketAddr,
        ops::{Deref, DerefMut},
        sync::Arc,
        time::Duration,
    };
    use tokio::{
        net::TcpStream,
        sync::{OwnedSemaphorePermit, Semaphore},
        time::timeout,
    };

    use crate::constants;

    pub struct UpstreamTcpStreamPool {
        semaphore: Arc<Semaphore>,
    }

    impl UpstreamTcpStreamPool {
        pub fn new() -> Self {
            Self {
                semaphore: Arc::new(Semaphore::new(
                    constants::UPSTREAM_TCP_SOCKET_POOL_SIZE as usize,
                )),
            }
        }

        pub async fn request_stream(&self, dest: SocketAddr) -> Option<UpstreamTcpSocketGuard> {
            let permit = match self.semaphore.clone().try_acquire_owned() {
                Err(e) => match e {
                    tokio::sync::TryAcquireError::Closed => {
                        eprintln!(
                            "   error: Upstream TCP stream pool semaphore is closed. No new permits can be offered. Unrecoverable state."
                        );
                        return None;
                    }
                    tokio::sync::TryAcquireError::NoPermits => {
                        return None;
                    }
                },
                Ok(permit) => permit,
            };

            Some(UpstreamTcpSocketGuard::new(
                UpstreamTcpStreamPool::create_stream(&dest).await?,
                permit,
            ))
        }

        async fn create_stream(dest: &SocketAddr) -> Option<TcpStream> {
            timeout(Duration::from_secs(9), async move {
                TcpStream::connect(dest).await
            })
            .await
            .ok()?
            .ok()
        }
    }

    pub struct UpstreamTcpSocketGuard {
        stream: TcpStream,
        permit: OwnedSemaphorePermit,
    }

    impl UpstreamTcpSocketGuard {
        pub fn new(stream: TcpStream, permit: OwnedSemaphorePermit) -> Self {
            Self { stream, permit }
        }
    }

    impl Deref for UpstreamTcpSocketGuard {
        type Target = TcpStream;

        fn deref(&self) -> &Self::Target {
            &self.stream
        }
    }

    impl DerefMut for UpstreamTcpSocketGuard {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.stream
        }
    }
}
