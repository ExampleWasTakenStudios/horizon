use std::time::SystemTime;

use crate::{cache::CacheTicket, protocol::packet::DnsPacket};

#[derive(Debug, Clone)]
pub enum CacheEntry {
    Entry(DnsPacket),
    Ticket(CacheTicket),
}

/// Wraps a cached [`DnsPacket`], adding cache relevant meta-data.
pub struct PacketEntry {
    /// [`SystemTime`] at which the [`DnsPacket`] was entered into the cache.
    /// The [`crate::protocol::records::DnsRecord::ttl`] is compared against this value to determine the validity of the cached data.
    timestamp: SystemTime,
    /// The cached [`DnsPacket`]
    packet: DnsPacket,
}

impl PacketEntry {
    pub fn get_timestamp(&self) -> &SystemTime {
        &self.timestamp
    }

    pub fn get_packet(&self) -> &DnsPacket {
        &self.packet
    }

    pub fn new(packet: DnsPacket) -> Self {
        Self {
            timestamp: SystemTime::now(),
            packet,
        }
    }
}
