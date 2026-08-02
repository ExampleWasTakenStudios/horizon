use std::sync::Arc;

use crate::{constants, network::DnsTcpListener};

pub(super) fn init() -> Vec<Arc<DnsTcpListener>> {
    let mut listeners =
        Vec::<Arc<DnsTcpListener>>::with_capacity(constants::DOWNSTREAM_SOCKET_TASK_COUNT as usize);

    for _ in 0..constants::DOWNSTREAM_SOCKET_TASK_COUNT {
        listeners.push(Arc::new(crate::network::DnsTcpListener::new()));
    }

    listeners
}
