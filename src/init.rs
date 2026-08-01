use crate::{
    constants,
    network::{DownstreamTcpListener, DownstreamUdpSocket, Firewall},
};
use std::sync::Arc;
use tokio::{sync::Semaphore, task::JoinSet};

/// Represents the core state of the application.
pub struct ApplicationState {
    /// The global [`JoinSet<()>`] that owns all parent tasks.
    pub global_join_set: JoinSet<()>,
}

/// Initialize application state.
///
/// # Panics
/// This method will panic when it encounters an error to prevent invalid application state.
pub fn init() -> ApplicationState {
    println!("Initializing...");

    let mut global_join_set = JoinSet::<()>::new();
    let udp_query_semaphore = Arc::new(Semaphore::new(constants::MAX_CONCURRENT_UDP_QUERIES));
    let tcp_query_semaphore = Arc::new(Semaphore::new(constants::MAX_CONCURRENT_TCP_QUERIES));

    init_downstream_udp_sockets(&mut global_join_set, udp_query_semaphore);
    init_downstream_tcp_listeners(&mut global_join_set, tcp_query_semaphore);

    ApplicationState { global_join_set }
}

pub fn init_tokio_runtime() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .name("horizon-runtime")
        .thread_name("horizon-worker")
        .build()
        .unwrap()
}

fn init_downstream_udp_sockets(join_set: &mut JoinSet<()>, semaphore: Arc<Semaphore>) {
    println!(
        "Spawning {} downstream UDP tasks...",
        constants::DOWNSTREAM_SOCKET_TASK_COUNT
    );

    for i in 0..constants::DOWNSTREAM_SOCKET_TASK_COUNT {
        let socket = Arc::new(DownstreamUdpSocket::create());
        let recv_task_socket = socket.clone();

        let semaphore_clone = semaphore.clone();

        join_set.spawn(async move {
            loop {
                let mut buf = [0_u8; constants::MAX_PACKET_SIZE];
                let (length, origin) = match recv_task_socket.recv_from(&mut buf).await {
                    Err(e) => {
                        eprintln!("error while receiving downstream traffic: {e}");
                        continue;
                    }
                    Ok(v) => v,
                };

                if !Firewall::verify_query(&buf, length) {
                    eprintln!("  warning: received invalid DGRAM from {}", origin.ip());
                    continue;
                }

                let permit = match semaphore_clone.clone().try_acquire_owned() {
                    Err(e) => {
                        match e {
                            tokio::sync::TryAcquireError::Closed => {
                                panic!("UDP Query Semaphore is closed. No new permits can be offered. Unrecoverable state.");
                            }
                            tokio::sync::TryAcquireError::NoPermits => {
                                eprintln!("  warning: max. number of concurrent UDP Queries reached. Dropping query...");
                                continue;
                            }
                        }
                    }
                    Ok(permit) => permit,
                };

                DownstreamUdpSocket::on_recv(permit, recv_task_socket.clone(), length, origin, buf);
            }
        });
        println!(
            "Created downstream UDP socket task {} of {}",
            i + 1,
            constants::DOWNSTREAM_SOCKET_TASK_COUNT
        );
    }
}

fn init_downstream_tcp_listeners(join_set: &mut JoinSet<()>, semaphore: Arc<Semaphore>) {
    println!(
        "Spawning {} downstream TCP tasks...",
        constants::DOWNSTREAM_SOCKET_TASK_COUNT
    );

    for i in 0..constants::DOWNSTREAM_SOCKET_TASK_COUNT {
        let listener = Arc::new(DownstreamTcpListener::create());
        let recv_task_listener = listener.clone();

        let semaphore_clone = semaphore.clone();

        join_set.spawn(async move {
            loop {
                let (stream, origin) = match recv_task_listener.accept().await {
                    Err(e) => {
                        eprintln!("error while accepting TCP connection: {e}");
                        continue;
                    }
                    Ok(v) => v,
                };

                let permit = match semaphore_clone.clone().try_acquire_owned() {
                        Err(e) => {
                            match e {
                                tokio::sync::TryAcquireError::Closed => {
                                    panic!("TCP Query Semaphore is closed. No new permits can be offered. Unrecoverable state...");
                                }
                                tokio::sync::TryAcquireError::NoPermits => {
                                    eprintln!("   warning: max. number of concurrent TCP queries reached. Dropping query...");
                                    let _ = stream.set_zero_linger(); // We do this to force the socket to be closed immediately.
                                    continue;
                                }
                            }
                        }
                        Ok(v) => v,
                    };

                    DownstreamTcpListener::on_recv(permit, stream, origin);
            }
        });
        println!(
            "Created downstream TCP listener task {} of {}",
            i + 1,
            constants::DOWNSTREAM_SOCKET_TASK_COUNT
        );
    }
}
