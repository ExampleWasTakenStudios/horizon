use crate::{cache::CacheTicket, protocol::packet::DnsPacket};

#[derive(Debug, Clone)]
pub enum CacheEntry {
    Entry(DnsPacket),
    Ticket(CacheTicket),
}
