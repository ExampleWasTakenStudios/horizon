use crate::network::UpstreamUdpSocketPool;

pub struct UpstreamAppState {
    pub udp_socket_pool: UpstreamUdpSocketPool,
}

pub(super) fn init() -> UpstreamAppState {
    UpstreamAppState {
        udp_socket_pool: UpstreamUdpSocketPool::new(),
    }
}
