mod constants;
mod init;
mod network;
mod query;

use std::time::Duration;

use tokio::{io::AsyncReadExt as _, net::TcpStream, time::timeout};

use crate::{
    init::{AppState, init_tokio_runtime},
    network::{DnsTcpListener, DnsUdpSocket, Firewall, TransmissionProtocol},
    query::Query,
};

pub fn entry() {
    let runtime = init_tokio_runtime();
    println!("Successfully created tokio runtime.");

    runtime.block_on(async move {
        let mut app_state = init::init();
        println!("Successfully initialized");

        run_downstream(&mut app_state);
        println!(" ");
        println!("RUNNING...");
        println!(" ");

        await_join_sets(&mut app_state).await;
    });
}

fn run_downstream(app_state: &mut AppState) {
    run_udp(app_state);
    run_tcp(app_state);

    fn run_udp(app_state: &mut AppState) {
        for downstream_socket in &app_state.downstream.udp_sockets {
            let semaphore = app_state.downstream.semaphore.clone();
            let downstream_socket = downstream_socket.clone();

            app_state.downstream.udp_join_set.spawn(async move {
                downstream_socket.listen_and_process(semaphore).await;
            });
        }
    }

    fn run_tcp(app_state: &mut AppState) {
        for downstream_listener in &app_state.downstream.tcp_listeners {
            let listener = downstream_listener.clone();
            let semaphore = app_state.downstream.semaphore.clone();

            app_state.downstream.tcp_join_set.spawn(async move {
                loop {
                    let stream = listener.accept().await;
                    stream.read_and_process(semaphore.clone()).await;
                }
            });
        }
    }
}

async fn await_join_sets(app_state: &mut AppState) {
    // Catch any panics in tasks hosting a downstream UDP socket and spawn a new one.
    while let Some(Err(_)) = app_state.downstream.udp_join_set.join_next().await {
        println!("Downstream UDP task panicked. Spawning a new one...");
        let semaphore = app_state.downstream.semaphore.clone();

        app_state.downstream.udp_join_set.spawn(async move {
            let socket = DnsUdpSocket::new();
            socket.listen_and_process(semaphore).await;
        });
    }

    // Catch any panics in TCP query handling tasks
    while let Some(Err(_)) = app_state.downstream.tcp_join_set.join_next().await {
        println!("Downstream TCP task panicked. Spawning a new one...");
        let semaphore = app_state.downstream.semaphore.clone();

        app_state.downstream.tcp_join_set.spawn(async move {
            let listener = DnsTcpListener::new();
            let stream = listener.accept().await;
            stream.read_and_process(semaphore).await;
        });
    }
}
