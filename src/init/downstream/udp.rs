use crate::{constants, network::downstream::udp::DownstreamUdpSocket};
use std::sync::Arc;

pub(super) fn init() -> Vec<Arc<DownstreamUdpSocket>> {
    let mut sockets = Vec::<Arc<DownstreamUdpSocket>>::with_capacity(
        constants::DOWNSTREAM_SOCKET_TASK_COUNT as usize,
    );

    for _ in 0..constants::DOWNSTREAM_SOCKET_TASK_COUNT {
        sockets.push(Arc::new(DownstreamUdpSocket::new()));
    }

    sockets
}
