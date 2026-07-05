use tokio::{self, runtime};

use crate::isc::init_msg_channels;

mod buffer;
mod protocol;
mod query_state;
mod systems;
mod isc;

pub const IP_ADDR: &str = "0.0.0.0:1234";
pub const MAX_PACKET_SIZE: usize = 1232;

pub fn entry() {
    let rt = runtime::Builder::new_multi_thread()
        .enable_all()
        .name("horizon-tokio-runtime")
        .thread_name("horizon-worker")
        .build()
        .unwrap();

    rt.block_on(async {
        // Initialize communication channels
        let channels = init_msg_channels();

        // Configure main socket
        let socket = tokio::net::UdpSocket::bind(IP_ADDR).await.unwrap();

        loop {
            let mut buf = [0_u8; MAX_PACKET_SIZE];
            let (_, origin) = match socket.recv_from(&mut buf).await {
                Err(e) => {
                    eprintln!("Error while receiving DGRAM: {:#?}", e);
                    continue;
                }
                Ok(value) => value,
            };

            
        }
    })
}
