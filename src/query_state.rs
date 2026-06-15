use std::{net::SocketAddr, time::SystemTime};

use crate::protocol::packet::DnsPacket;

pub struct QueryState<'a> {
    query: DnsPacket,
    sender: SocketAddr,
    timestamp: SystemTime,
    pub master_zone_file: Option<&'a MasterZoneFile>,
    pub blacklist: Option<&'a Blacklist>,
    pub response: Option<DnsPacket>,
}

impl QueryState {
    pub fn new(query: DnsPacket, sender: SocketAddr, timestamp: SystemTime) -> Self {
        QueryState {
            query,
            sender,
            timestamp,
            master_zone_file: None,
            blacklist: None,
            response: None
        }
    }

    pub fn get_query(&self) -> DnsPacket {
        self.query
    }

    pub fn get_sender(&self) -> SocketAddr {
        self.sender
    }

    pub fn get_timestamp(&self) -> SystemTime {
        self.timestamp
    }
}
