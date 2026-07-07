use tokio::{self, runtime};

use crate::{
    isc::init_msg_channels,
    systems::{IesChannels, IngressEgressSystem},
};

mod buffer;
mod isc;
mod protocol;
mod query_state;
mod systems;

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

        // Start IES
        IngressEgressSystem::run(IesChannels {
            ies_ingress_rx: channels.ies_ingress_channel.rx,
            ies_to_dps_tx: channels.ies_to_dps.tx,
            ies_egress_tx: channels.ies_egress_channel.tx,
            ies_resolving_rx: channels.ies_resolving_channel.rx,
            ies_answer_rx: channels.ies_answer_channel.rx,
        })
        .await;

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
