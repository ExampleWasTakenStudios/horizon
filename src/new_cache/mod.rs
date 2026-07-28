use std::{sync::Arc, time::Instant};

use dashmap::DashMap;
use tokio::sync::broadcast;

use crate::{
    new_cache::{
        cache_entry::{CacheEntry, CacheTicket, RRSet},
        ticket_guard::CacheTicketGuard,
    },
    protocol::{DnsQuestion, DnsRecord},
};

pub mod cache_entry;
pub mod ticket_guard;

/// # Cache
/// This struct is the representation of the DNS cache of the service.
/// It provides a clean API for other systems to use.
///
/// ## Caching Strategy
/// The cache optimizes network I/O operations by minimizing the amount of queries needed to be sent to an upstream resolver.
/// It does this by coalescing multiple, identical queries into one single upstream query.
///
/// ## Ticket System
/// Once a query is made to an upstream resolver by the system responsibly for the query, shall register a ticket with the cache system.
/// This tells the cache that it can expect a cacheable response to the query in some finite, yet unknown time in the future.
/// Subsequent queries with the specific DNS question will thus cause the cache to wait for the already inflight query instead of reporting a cache-miss.
///
/// #### Registering a Ticket
/// [`Cache::register_ticket()`] returns a [`Result<T, E>`] where `T` is a
/// [`CacheTicketGuard`] and `E` is a [`broadcast::Receiver`] through which
/// the caller attempting to register the new ticket may subscribe to the query
/// that is already in progress.
///
/// #### Redeeming a Ticket
/// When an RRSet is committed to cache through the [`Cache::commit()`] method,
/// the system will automatically redeem the ticket if one is present for the specified DNS question.
pub struct Cache {
    enabled: bool,
    cache: Arc<DashMap<DnsQuestion, CacheEntry>>,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            enabled: true,
            cache: Arc::new(DashMap::new()),
        }
    }

    pub async fn check_for(&self, question: &DnsQuestion) -> Option<Vec<DnsRecord>> {
        // If the cache is disabled, we always return `None`
        if !self.enabled {
            return None;
        }

        // A loop is used to to trigger a lookup should the broadcast be dropped.
        // If that happens the loop is `continue`ed and thus the process starts over.
        loop {
            // Check if the cache contains an entry for the query packet
            let cache_entry = match self.cache.get(question) {
                None => return None,
                Some(entry) => entry.value().clone(),
            };

            let cache_data = cache_entry;

            match cache_data {
                CacheEntry::RRSet(rr_set) => {
                    // Check TTL validity
                    if Instant::now() > *rr_set.get_ttl() {
                        // The entry is stale so we remove it from cache and return `None`
                        self.cache.remove(question);

                        return None;
                    }

                    return Some(rr_set.get_records().clone());
                }
                CacheEntry::Ticket(t) => {
                    match t.get_sender().subscribe().recv().await {
                        Err(e) => {
                            match e {
                                broadcast::error::RecvError::Closed => {
                                    // The sender of the channel was dropped, this indicates that the task which registered the ticket has failed.
                                    // This causes:
                                    //     1. The ticket to be removed (this happens because the CacheTicketGuard goes out of scope and removes the ticket in its drop method)
                                    //     2. The entire process of checking the cache should start from the beginning (restarting the loop).
                                    //        This has the effect that all tasks waiting for the ticket to resolve, will check the cache again.
                                    //        The first tasks to check the cache again will receive `None` and therefore advance through the system, eventually reaching the SRS and registering a new ticket.
                                    //        All other tasks will see the newly registered ticket when checking the cache again and wait on that to resolve.
                                    continue;
                                }
                                broadcast::error::RecvError::Lagged(_) => {
                                    // There should usually only be one message sent through the broadcast channel (when the upstream answer is received)
                                    // As this is unexpected behavior, we return `None` to cause the task to not rely on the cache anymore.
                                    return None;
                                }
                            }
                        }
                        Ok(rr_set) => return Some(rr_set.clone()),
                    }
                }
            }
        }
    }

    /// Register a ticket with the cache for a DNS question.
    /// - Returns [`Ok()`] containing a [`CacheTicketGuard`] when the ticket was successfully registered.
    /// - Returns [`Err()`] containing a [`broadcast::Receiver<Vec<DnsRecord>>`] when there is already a ticket registered for the DNS question.
    ///   The Receiver can be used to wait for response to the already in progress query.
    pub fn register_ticket(
        &self,
        question: DnsQuestion,
    ) -> Result<CacheTicketGuard, broadcast::Receiver<Vec<DnsRecord>>> {
        // Check if ticket already exists
        // If true, we return a receiver for the sender giving the caller the chance to be notified once the ticket is redeemed.

        if let Some(entry) = self.cache.get(&question)
            && let CacheEntry::Ticket(t) = entry.value()
        {
            return Err(t.get_sender().subscribe());
        }

        let (ticket_sender, _) = broadcast::channel(1);
        let ticket_guard_sender = ticket_sender.clone();
        let ticket = CacheTicket::new(ticket_sender);
        let entry = CacheEntry::Ticket(ticket);

        self.cache.insert(question.clone(), entry);

        let ticket_guard = CacheTicketGuard::new(
            self.cache.clone(),
            question.clone(),
            ticket_guard_sender.clone(),
        );

        Ok(ticket_guard)
    }

    /// Commit an RRSet to cache.
    ///
    /// If an RRSet is currently cached for the specified DNS question, it will be overwritten.
    /// If a ticket is currently cached for the specified DNS question, it will be redeemed.
    pub fn commit(&self, question: DnsQuestion, records: Vec<DnsRecord>) {
        self.cache
            .insert(question, CacheEntry::RRSet(RRSet::new(records)));
    }

    pub fn clear(&self) {
        self.cache.clear();
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}
