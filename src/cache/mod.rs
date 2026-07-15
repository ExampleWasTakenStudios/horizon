pub mod cache_entry;
pub mod cache_ticket;

use std::{
    sync::Arc,
    time::Duration,
};

pub use cache_entry::*;
pub use cache_ticket::*;

use dashmap::DashMap;
use tokio::sync::broadcast;

use crate::protocol::packet::DnsPacket;

/// # Cache
/// This struct is the representation of the DNS cache of the service.
/// It provides a clean API for other systems to use.
///
/// ## API
/// The API of the cache consists of four functions:
/// - [`Cache::check_for()`]
/// - [`Cache::register_ticket()`]
/// - [`Cache::add()`]
/// - [`Cache::clear()`]
///
/// See their individual documentation for more details.
///
/// #### Why is there no `remove()` method?
/// A vital part of DNS caching is so called *[negative caching](https://en.wikipedia.org/wiki/Negative_cache)*.
/// In short, it describes the practice of also caching negative responses like failed responses etc.
/// Because of that, the cache makes absolutely no assumption of what kind of data it stores, other than that it is a [`DnsPacket`].
/// This means that the only valid way a [`CacheEntry`] should ever be removed from the cache is when its TTL expires.
///
/// *So what if there is a fatal failure within the system that prevents a ticket from ever resolving?*
///
/// A fatal failure within the system will cause the task to panic. This inherently causes the [`broadcast::Sender`] to being dropped.
/// Dropping of the [`broadcast::Sender`] is detected by all tasks waiting for the associated ticket to resolve and causes them to restart the cache lookup.
/// For more details on this mechanism see the inline documentation in the [`Cache::check_for()`] method in the source code.
///
/// ## Caching Strategy
/// The cache optimizes network I/O operations by minimizing the amount of queries needed to be sent to an upstream resolver.
/// It does this by coalescing multiple, identical queries into one single upstream query.
///
/// ### Ticket System
/// Once a query is made to an upstream resolver, the system responsible for the query, shall register a ticket for a query with the cache system.
/// This tells the cache that it can expect a cacheable result for the query some finite, yet unknown, time in the future.
/// Subsequent queries for that domain will thus cause the cache to wait for the result instead of reporting a cache-miss thereby causing the same query to be sent to an upstream resolver.
///
/// ## RFC Compliance
/// At the moment, this system is not fully compliant with [RFC 1035](https://datatracker.ietf.org/doc/html/rfc1035) and [RFC 2181](https://datatracker.ietf.org/doc/html/rfc2181).
///
/// Namely,
/// - Cache entries are based on an entire query [`DnsPacket`] instead of just the [`DnsQuestion`](crate::protocol::question::DnsQuestion)
/// - TTL handling is done using the smallest TTL value in an entire [`DnsPacket`]. This is similar to RFC 2181,
///   however, the system does not explicitly recognize RRSets or other structures defined in RFC 2181.
///
/// ## Memory Safety
/// At the moment, registered tickets will remain in cache indefinitely.
/// This can cause memory leaks in cases where tickets are never resolved into an actual [`CacheEntry::Entry()`].
///
/// This vulnerability should be fixed in future version. [`CacheTicket`]s should ideally, have a timeout after which they are removed from cache.
///
/// The present challenge is that [`CacheEntry::Ticket()`] and [`CacheEntry::Entry()`] are stored using the same key.
/// This means that a cleanup function would clean up a valid [`CacheEntry::Entry()`]
/// if a ticket was previously stored for the specified key.
pub struct Cache {
    cache: Arc<DashMap<DnsPacket, CacheEntry>>,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            cache: Arc::new(DashMap::new()),
        }
    }

    /// Check if the [`DnsPacket`], passed as argument, is cached.
    pub async fn check_for(&self, query_packet: &DnsPacket) -> Option<DnsPacket> {
        loop {
            // Check if the cache contains an entry for the query packet
            let cache_entry = match self.cache.get(query_packet) {
                Some(entry) => entry.value().clone(), // This value needs to be cloned because a) it must eventually be returned as owned and b) the reference into the dashmap must be dropped to keep the thread-lock on it for as little time as possible.
                None => return None,
            };

            match cache_entry {
                CacheEntry::Entry(e) => return Some(e.clone()),
                CacheEntry::Ticket(t) => {
                    match t.broadcast_sender.subscribe().recv().await {
                        Err(e) => {
                            match e {
                                broadcast::error::RecvError::Closed => {
                                    // The sender of the channel was dropped, this indicates that the task which registered the ticket has failed.
                                    // This causes:
                                    //     1. The ticket to be removed
                                    //     2. The entire process of checking the cache should start from the beginning (restarting the loop).
                                    //        This has the effect that all tasks waiting for the ticket to resolve, will check the cache again.
                                    //        The first task to check the cache again will receive `None` and therefore advance through the system, eventually reaching the SRS and registering a new ticket.
                                    //        All other tasks will see the newly registered ticket when checking the cache again and wait on that to resolve.

                                    // Remove ticket
                                    let _ = self.cache.remove(query_packet);

                                    // Restart the loop
                                    continue;
                                }
                                broadcast::error::RecvError::Lagged(_) => {
                                    // There should usually only be one message sent through the broadcast channel (when the upstream answer is received)
                                    // As this is unexpected behavior, we return `None` to cause the task to not rely on the cache anymore.
                                    return None;
                                }
                            }
                        }
                        Ok(packet) => return Some(packet),
                    }
                }
            }
        }
    }

    /// Register a ticket for a query [`DnsPacket`] with the cache system.
    pub fn register_ticket(&self, query_packet: &DnsPacket) {
        let (broadcast_sender, _) = broadcast::channel(1);
        let ticket = CacheTicket {
            broadcast_sender
        };

        let cache_entry = CacheEntry::Ticket(ticket);

        let _ = self.cache.insert(query_packet.clone(), cache_entry);
    }

    /// Add an entry into the cache.
    ///
    /// If there is a cache ticket, it is replaced and all tasks subscribed to the tickets broadcast are notified.
    pub fn add(&self, query_packet: &DnsPacket, answer_packet: &DnsPacket) {
        self.schedule_removal(Duration::from_secs(self.get_shortest_ttl(answer_packet) as u64), query_packet.clone());

        // This may seem illogical at first.
        // `DashMap::insert()` inserts the passed key-value pair into the map and returns any value that may have previously been stored for the passed key.
        // We use this by checking if the `DashMap::insert()` method returns an entry. If the returned entry is a ticket the `DashMap::insert()` method
        // replaces the stored ticket with the new packet and we can notify all subscribed broadcast receivers of the ticket about the arrival of an actual `DnsPacket`.
        if let Some(entry) = self.cache.insert(
            query_packet.clone(),
            CacheEntry::Entry(answer_packet.clone()),
        ) && let CacheEntry::Ticket(t) = entry
        {
            let _ = t.broadcast_sender.send(answer_packet.clone());
        }
    }

    /// See [`DashMap::clear()`]
    pub fn clear(&self) {
        self.cache.clear();
    }

    /// Schedule the removal of a [`CacheEntry`] after a set duration.
    /// This method does not distinguish between [`CacheEntry::Entry()`] and [`CacheEntry::Ticket()`].
    ///
    /// `dur` - [`Duration`] after which the entry should be cleared
    /// `key` - The key under which the entry is stored in cache
    fn schedule_removal(&self, dur: Duration, key: DnsPacket) {
        let cache = self.cache.clone();
        tokio::spawn(async move {
            tokio::time::sleep(dur).await;

            cache.clone().remove(&key);
        });
    }

    /// Gets the shortest TTL present in the [`DnsPacket`]. This TTL will be used as caching reference.
    fn get_shortest_ttl(&self, answer_packet: &DnsPacket) -> u32 {
        let mut shortest_ttl = u32::MAX;

        // Check answer records
        for record in &answer_packet.answers {
            if record.ttl < shortest_ttl {
                shortest_ttl = record.ttl;
            }
        }

        // Check authoritative records
        for record in &answer_packet.authoritatives {
            if record.ttl < shortest_ttl {
                shortest_ttl = record.ttl;
            }
        }

        // Check additional records
        for record in &answer_packet.additionals {
            if record.ttl < shortest_ttl {
                shortest_ttl = record.ttl;
            }
        }

        shortest_ttl
    }
}
