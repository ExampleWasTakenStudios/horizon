use tokio::task::JoinSet;

use crate::{
    constants,
    network::{DownstreamTcpListener, DownstreamUdpSocket},
};

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
    init_downstream_udp_sockets(&mut global_join_set);
    init_downstream_tcp_listeners(&mut global_join_set);

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

fn init_downstream_udp_sockets(join_set: &mut JoinSet<()>) {
    println!(
        "Spawning {} downstream UDP tasks...",
        constants::DOWNSTREAM_SOCKET_TASK_COUNT
    );
    for i in 0..constants::DOWNSTREAM_SOCKET_TASK_COUNT {
        let socket = DownstreamUdpSocket::create();

        join_set.spawn(async move {
            loop {
                let mut buf = [0_u8; constants::MAX_PACKET_SIZE];
                let (length, origin) = match socket.recv_from(&mut buf).await {
                    Err(e) => {
                        eprintln!("error while receiving downstream traffic: {e}");
                        continue;
                    }
                    Ok(v) => v,
                };

                DownstreamUdpSocket::on_recv(length, origin, buf);
            }
        });
        println!(
            "Created downstream UDP socket task {} of {}",
            i + 1,
            constants::DOWNSTREAM_SOCKET_TASK_COUNT
        );
    }
}

fn init_downstream_tcp_listeners(join_set: &mut JoinSet<()>) {
    println!(
        "Spawning {} downstream TCP tasks...",
        constants::DOWNSTREAM_SOCKET_TASK_COUNT
    );

    for i in 0..constants::DOWNSTREAM_SOCKET_TASK_COUNT {
        let listener = DownstreamTcpListener::create();

        join_set.spawn(async move {
            loop {
                let (stream, origin) = match listener.accept().await {
                    Err(e) => {
                        eprintln!("error while accepting TCP connection: {e}");
                        continue;
                    }
                    Ok(v) => v,
                };

                DownstreamTcpListener::on_recv(stream, origin);
            }
        });
        println!(
            "Created downstream TCP listener task {} of {}",
            i + 1,
            constants::DOWNSTREAM_SOCKET_TASK_COUNT
        );
    }
}
