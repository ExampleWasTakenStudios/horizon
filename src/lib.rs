use std::net::Ipv4Addr;

use crate::systems::{DecisionPipelineSystem, IngressEgressSystem, StubResolverSystem};
use tokio::runtime;

mod buffer;
mod isc;
mod protocol;
mod query_state;
mod systems;

pub const OWN_IP_SOCKET_ADDR: &str = "0.0.0.0:1234";
pub const MAX_PACKET_SIZE: usize = 512;

pub const UPSTREAM_IP_ADDR: Ipv4Addr = Ipv4Addr::from_octets([1, 1, 1, 1]);

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
            UPSTREAM_IP_ADDR,
            channels.ies_to_dps.tx.clone(),
            channels.ies_answer.rx,
            channels.ies_upstream_resolve.rx,
        )
        .await;

        // Init DPS
        let dps =
            DecisionPipelineSystem::new(channels.ies_to_dps.rx, channels.dps_to_srs.tx.clone());

        // Init SRS
        let srs = StubResolverSystem::new(
            channels.dps_to_srs.rx,
            channels.ies_upstream_resolve.tx.clone(),
            channels.ies_answer.tx.clone(),
        );

        let ies_join_set = ies.run().await;
        let dps_join_set = dps.run().await;
        let srs_join_set = srs.run().await;

        println!("READY");

        ies_join_set.join_all().await;
        dps_join_set.join_all().await;
        srs_join_set.join_all().await;
    })
}
