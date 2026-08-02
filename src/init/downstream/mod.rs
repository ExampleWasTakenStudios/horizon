use std::sync::Arc;

use tokio::{sync::Semaphore, task::JoinSet};

use crate::{
    constants,
    network::{DnsTcpListener, DnsUdpSocket},
};

pub(super) mod tcp;
pub(super) mod udp;

pub struct DownstreamAppState {
    pub udp_sockets: Vec<Arc<DnsUdpSocket>>,
    pub tcp_listeners: Vec<Arc<DnsTcpListener>>,
    pub udp_join_set: JoinSet<()>,
    pub tcp_join_set: JoinSet<()>,
    pub semaphore: Arc<Semaphore>,
}

pub(super) fn init() -> DownstreamAppState {
    DownstreamAppState {
        udp_sockets: udp::init(),
        tcp_listeners: tcp::init(),
        udp_join_set: JoinSet::new(),
        tcp_join_set: JoinSet::new(),
        semaphore: Arc::new(Semaphore::new(constants::MAX_CONCURRENT_ACTIVE_QUERIES)),
    }
}
