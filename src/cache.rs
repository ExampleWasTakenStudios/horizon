pub mod cache_entry;
pub mod cache_error;
pub mod cache_ticket;

pub use cache_entry::*;
pub use cache_error::*;
pub use cache_ticket::*;

use dashmap::DashMap;
use tokio::sync::broadcast;

use crate::{cache::CacheEntry::Ticket, protocol::{domain::DomainName, packet::DnsPacket}};

pub struct Cache {
    cache: DashMap<DnsPacket, CacheEntry>,
}

impl Cache {
    /// Check if the [`DnsPacket`], passed as argument, is cached.
    pub async fn check_for(&self, query_packet: &DnsPacket) -> Option<DnsPacket> {
        loop {
            // Check if the cache contains an entry for the query packet
            let cache_entry = match self.cache.get(query_packet) {
                Some(entry) => entry.value().clone(), // This value needs to be cloned because a) it must eventually be returned as owned and b) the reference into the dashmap must be dropped to keep the thread-lock on it for as little time as possible.
                None => return None
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
                        Ok(packet) => return Some(packet)
                    }
                }
            }
        }
    }

    /// Adds an entry into the cache.
    pub fn add(&self, query_packet: &DnsPacket, answer_packet: &DnsPacket) {
        if let Some(entry) = self
            .cache
            .insert(query_packet.clone(), CacheEntry::Entry(answer_packet.clone()))
            && let CacheEntry::Ticket(t) = entry
        {
            let _ = t.broadcast_sender.send(answer_packet.clone());
        }
    }
}
