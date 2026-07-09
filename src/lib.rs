use tokio::{self, runtime};

use crate::systems::IngressEgressSystem;

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

        // Start IES
        let ies = IngressEgressSystem::new(
            channels.ies_to_dps.tx,
            channels.ies_answer.rx,
            channels.ies_upstream_resolve.rx,
        )
        .await;

        ies.run().await;
    })
}
