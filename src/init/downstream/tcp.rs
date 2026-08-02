use std::sync::Arc;

use tokio::net::TcpListener;

use crate::constants;

pub(super) fn init() -> Vec<Arc<TcpListener>> {
    let mut listeners =
        Vec::<Arc<TcpListener>>::with_capacity(constants::DOWNSTREAM_SOCKET_TASK_COUNT as usize);

    for _ in 0..constants::DOWNSTREAM_SOCKET_TASK_COUNT {
        listeners.push(Arc::new(crate::network::create_tcp_listener()));
    }

    listeners
}
