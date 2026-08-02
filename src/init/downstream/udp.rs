use crate::{constants, network::DnsUdpSocket};
use std::sync::Arc;

pub(super) fn init() -> Vec<Arc<DnsUdpSocket>> {
    let mut sockets =
        Vec::<Arc<DnsUdpSocket>>::with_capacity(constants::DOWNSTREAM_SOCKET_TASK_COUNT as usize);

    for _ in 0..constants::DOWNSTREAM_SOCKET_TASK_COUNT {
        sockets.push(Arc::new(crate::network::DnsUdpSocket::new()));
    }

    sockets
}
