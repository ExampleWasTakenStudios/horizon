use tokio::sync::broadcast;

use crate::protocol::packet::DnsPacket;

/// Represents a ticket for a future cache entry.
///
/// This is used to indicate to the cache that a cacheable result for a specified domain is expected in some finite amount of time in the future.
/// The cache uses this to "reserve" a cache entry for the specified domain.
///
/// Subsequent cache checks will wait until the cache result for the domain becomes available instead of reporting a cache-miss.
///
/// This is an internal struct that external callers (outside the cache system) should not require.
#[derive(Debug, Clone)]
pub struct CacheTicket {
    pub broadcast_sender: broadcast::Sender<DnsPacket>,
}
