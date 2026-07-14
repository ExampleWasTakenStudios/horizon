pub mod cache_entry;
pub mod cache_error;
pub mod cache_ticket;

pub use cache_entry::*;
pub use cache_error::*;
pub use cache_ticket::*;

use dashmap::DashMap;
use tokio::sync::broadcast;

use crate::protocol::{domain::DomainName, packet::DnsPacket};

pub struct Cache {
    cache: DashMap<DomainName, CacheEntry>,
}

impl Cache {
    /// Check if the [`DomainName`], passed as argument, is cached.
    pub async fn check_for(&self, domain: &DomainName) -> Option<DnsPacket> {
        // Check if the cache contains an entry for the domain name
        if self.cache.contains_key(domain)
            && let Some(entry) = self.cache.get(domain)
        {
            match entry.value() {
                CacheEntry::Entry(e) => Some(e.clone()), // The entry is a `DnsPacket` that has been received in the past so we return it immediately.
                CacheEntry::Ticket(t) => {
                    // The entry is a `CacheTicket` so we subscribe to the broadcast channel through which the `CacheTicket` will notify us once the response has been received.
                    match t.broadcast_sender.subscribe().recv().await {
                        Err(e) => {
                            eprintln!(
                                "Error occurred while receiving cache ticket resolve message: {e}"
                            );
                            None
                        }
                        Ok(packet) => Some(packet),
                    }
                }
            }
        } else {
            None
        }
    }

    /// Registers a ticket with the cache. See [`CacheTicket`].
    pub fn register_ticket(&self, domain: &DomainName) {
        // The sender is used by this ticket to send the DnsPacket to all other tasks once it has been received.
        let (broadcast_sender, _) = broadcast::channel(1);

        let ticket = CacheTicket {
            broadcast_sender: broadcast_sender.clone(),
        };

        let entry = CacheEntry::Ticket(ticket);

        self.cache.insert(domain.clone(), entry);
    }

    pub fn cancel_ticket(&self, domain: &DomainName) {
        if !self.cache.contains_key(domain) {
            return; // Cache does not contain any entry for the passed domain
        }

        if let Some(e) = self.cache.get(domain) {
            match e.value() {
                CacheEntry::Entry(_) => return,
                CacheEntry::Ticket(t) => {
                    if let Some((_, CacheEntry::Ticket(ticket))) = self.cache.remove(domain) {
                        //
                    }
                }
            }
        }
    }

    /// Adds an entry into the cache.
    pub fn add(&self, domain: &DomainName, packet: &DnsPacket) {
        if let Some(entry) = self
            .cache
            .insert(domain.clone(), CacheEntry::Entry(packet.clone()))
            && let CacheEntry::Ticket(t) = entry
        {
            let _ = t.broadcast_sender.send(packet.clone());
        }
    }
}
