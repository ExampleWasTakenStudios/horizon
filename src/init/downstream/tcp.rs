use std::sync::Arc;

use crate::{constants, network::DownstreamTcpListener};

pub(super) fn init() -> Vec<Arc<DownstreamTcpListener>> {
    let mut listeners = Vec::<Arc<DownstreamTcpListener>>::with_capacity(
        constants::DOWNSTREAM_SOCKET_TASK_COUNT as usize,
    );

    for _ in 0..constants::DOWNSTREAM_SOCKET_TASK_COUNT {
        listeners.push(Arc::new(crate::network::DownstreamTcpListener::new()));
    }

    listeners
}
