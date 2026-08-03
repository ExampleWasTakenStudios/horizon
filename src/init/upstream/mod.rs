use crate::network::upstream::{tcp::UpstreamTcpStreamPool, udp::UpstreamUdpSocketPool};

pub struct UpstreamAppState {
    pub udp_socket_pool: UpstreamUdpSocketPool,
    pub tcp_stream_pool: UpstreamTcpStreamPool,
}

pub(super) fn init() -> UpstreamAppState {
    UpstreamAppState {
        udp_socket_pool: UpstreamUdpSocketPool::new(),
        tcp_stream_pool: UpstreamTcpStreamPool::new(),
    }
}
