use std::net::Ipv4Addr;

use crate::systems::{DecisionPipelineSystem, IngressEgressSystem};
use tokio::runtime;

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
        let channels = isc::init_msg_channels();

        // Init IES
        let ies = IngressEgressSystem::new(
            Ipv4Addr::new(1, 1, 1, 1),
            channels.ies_to_dps.tx,
            channels.ies_answer.rx,
            channels.ies_upstream_resolve.rx,
        )
        .await;

        // Init DPS
        let dps = DecisionPipelineSystem::new(channels.ies_to_dps.rx, channels.dps_to_srs.tx);

        let ies_join_set = ies.run().await;
        let dps_join_set = dps.run().await;

        ies_join_set.join_all().await;
        dps_join_set.join_all().await;
    })
}
