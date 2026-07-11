use std::{net::SocketAddr, time::SystemTime};

use crate::protocol::packet::DnsPacket;

#[derive(Debug)]
pub struct QueryState {
    origin: SocketAddr,
    packet: DnsPacket,
    timestamp: SystemTime,
    response: Option<DnsPacket>,
}

impl QueryState {
    pub fn new(origin: SocketAddr, packet: DnsPacket) -> QueryState {
        QueryState {
            origin,
            packet,
            timestamp: SystemTime::now(),
            response: None,
        }
    }

    pub fn get_origin(&self) -> &SocketAddr {
        &self.origin
    }

    pub fn get_packet(&self) -> &DnsPacket {
        &self.packet
    }

    pub fn get_timestamp(&self) -> &SystemTime {
        &self.timestamp
    }

    pub fn get_response(&self) -> &Option<DnsPacket> {
        &self.response
    }

    pub fn set_response(&mut self, response: DnsPacket) {
        self.response = Some(response);
    }
}
