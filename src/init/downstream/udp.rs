use tokio::net::UdpSocket;

use crate::constants;
use std::sync::Arc;

pub(super) fn init() -> Vec<Arc<UdpSocket>> {
    let mut sockets =
        Vec::<Arc<UdpSocket>>::with_capacity(constants::DOWNSTREAM_SOCKET_TASK_COUNT as usize);

    for _ in 0..constants::DOWNSTREAM_SOCKET_TASK_COUNT {
        sockets.push(Arc::new(crate::network::create_udp_socket()));
    }

    sockets
}
