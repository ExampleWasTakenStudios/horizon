use std::{
    net::{Ipv4Addr, SocketAddr, SocketAddrV4}, str::FromStr, sync::Arc,
};

use tokio::{net::UdpSocket, runtime, task::JoinSet};

use crate::{constants::{DOWNSTREAM_SOCKET_ADDR, MAX_PACKET_SIZE, UPSTREAM_SOCKET_ADDR}, query::{Query, SocketData}, srs::StubResolverSystem};

mod buffer;
mod protocol;
mod query;
mod srs;
mod constants;



pub fn entry() {
    let rt = runtime::Builder::new_multi_thread()
        .enable_all()
        .name("horizon-tokio-runtime")
        .thread_name("horizon-worker")
        .build()
        .unwrap();

    rt.block_on(async move {
        let downstream_socket = match UdpSocket::bind(DOWNSTREAM_SOCKET_ADDR).await {
            Err(e) => panic!("Error occurred while trying to bind downstream socket: {e}"),
            Ok(socket) => {
                println!(
                    "Successfully bound downstream socket to: {}",
                    socket.local_addr().unwrap()
                );
                socket
            }
        };
        let downstream_socket = Arc::new(downstream_socket);

        let upstream_socket = match UdpSocket::bind(UPSTREAM_SOCKET_ADDR).await {
            Err(e) => panic!("Error occurred while trying to bind upstream socket with address {DOWNSTREAM_SOCKET_ADDR}.\n Error: {e}"),
            Ok(socket) => {
                println!(
                    "Successfully bound upstream socket to: {}",
                    socket.local_addr().unwrap()
                );
                socket
            }
        };
        let upstream_socket = Arc::new(upstream_socket);

        let srs = Arc::new(StubResolverSystem::new(SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(1, 1, 1, 1), 53))));

        let mut active_query_join_set = JoinSet::<()>::new();

        loop {
            let mut recv_buf = [0_u8; MAX_PACKET_SIZE];
            match downstream_socket.recv_from(&mut recv_buf).await {
                Err(e) => {
                    eprintln!("Error while receiving on downstream socket: {e}");
                    continue;
                }
                Ok((length, origin)) => {
                    let downstream_socket = downstream_socket.clone();
                    let upstream_socket = upstream_socket.clone();
                    let srs = srs.clone();

                    let socket_data = SocketData { length, origin, data: recv_buf };

                    active_query_join_set.spawn(async move {
                        Query::create(
                            downstream_socket,
                            upstream_socket,
                            srs,
                            socket_data,
                        ).await;
                    });
                }
            };
        }
    });
}
