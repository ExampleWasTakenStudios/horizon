use std::sync::Arc;

use tokio::{
    net::{TcpListener, UdpSocket},
    sync::Semaphore,
};

use crate::constants;

pub(super) mod tcp;
pub(super) mod udp;

pub struct DownstreamAppState {
    pub udp_sockets: Vec<Arc<UdpSocket>>,
    pub tcp_listeners: Vec<Arc<TcpListener>>,
    pub semaphore: Arc<Semaphore>,
}

pub(super) fn init() -> DownstreamAppState {
    DownstreamAppState {
        udp_sockets: udp::init(),
        tcp_listeners: tcp::init(),
        semaphore: Arc::new(Semaphore::new(
            constants::MAX_CONCURRENT_ACTIVE_QUERIES,
        )),
    }
}
