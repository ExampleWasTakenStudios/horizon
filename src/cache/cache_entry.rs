use crate::{cache::CacheTicket, protocol::packet::DnsPacket};

pub enum CacheEntry {
    Entry(DnsPacket),
    Ticket(CacheTicket),
}
