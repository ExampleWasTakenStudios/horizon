use std::{
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    sync::Arc,
};

use tokio::{net::UdpSocket, runtime, task::JoinSet};

use crate::{query::{Query, SocketData}, srs::StubResolverSystem};

mod buffer;
mod protocol;
mod query;
mod srs;

const DOWNSTREAM_IP_ADDR: Ipv4Addr = Ipv4Addr::new(0, 0, 0, 0);
const DOWNSTREAM_PORT: u16 = 1234;
pub const DOWNSTREAM_SOCKET_ADDR: SocketAddr =
    SocketAddr::V4(SocketAddrV4::new(DOWNSTREAM_IP_ADDR, DOWNSTREAM_PORT));

const UPSTREAM_IP_ADDR: Ipv4Addr = Ipv4Addr::new(1, 1, 1, 1);
const UPSTREAM_PORT: u16 = 53;
pub const UPSTREAM_SOCKET_ADDR: SocketAddr =
    SocketAddr::V4(SocketAddrV4::new(UPSTREAM_IP_ADDR, UPSTREAM_PORT));

pub const MAX_PACKET_SIZE: usize = 512;

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

        let upstream_socket = match UdpSocket::bind(DOWNSTREAM_SOCKET_ADDR).await {
            Err(e) => panic!("Error occurred while trying to bind upstream socket: {e}"),
            Ok(socket) => {
                println!(
                    "Successfully bound upstream socket to: {}",
                    socket.local_addr().unwrap()
                );
                socket
            }
        };
        let upstream_socket = Arc::new(upstream_socket);

        let srs = Arc::new(StubResolverSystem::new());

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
