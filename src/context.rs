use std::{net::SocketAddr, sync::Arc, time};

use dashmap::DashMap;
use tokio::net;

use crate::{buffer::PacketBuffer, protocol::{DnsPacket, ResponseCode}};

pub struct QueryContext {
    udp_socket: Arc<net::UdpSocket>,
    original_id: u16,
    horizon_id: u16,
    source_addr: SocketAddr,
    received_timestamp: time::SystemTime,
    id_map: Arc<DashMap<u16, QueryContext>>,
    packet: DnsPacket,
}

impl QueryContext {
    pub fn create(udp_socket: Arc<net::UdpSocket>, source_addr: SocketAddr, buffer: [u8; 512], tx_id_map: u8) -> Self {
        let packet_buffer = PacketBuffer::from_raw_buffer(buffer);
        let packet_parsing_result = DnsPacket::parse_from(&mut packet_buffer);

        
    }
}

impl Drop for QueryContext {
    fn drop(&mut self) {
        self.id_map.remove(&self.horizon_id);
    }
}
