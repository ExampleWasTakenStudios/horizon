use core::time;
use std::{
    sync::Arc,
    time::{Duration, Instant, SystemTime},
};

use dashmap::DashMap;
use tokio::sync::broadcast;

use crate::{
    new_cache::cache_entry::{CacheData, CacheEntry, RRSet},
    protocol::{DnsQuestion, DnsRecord},
};

pub mod cache_entry;

pub struct Cache {
    cache: Arc<DashMap<DnsQuestion, CacheEntry>>,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            cache: Arc::new(DashMap::new()),
        }
    }

    pub async fn check_for(&self, question: &DnsQuestion) -> Option<Vec<DnsRecord>> {
        // A loop is used to to trigger a lookup should the broadcast be dropped.
        // If that happens the loop is `continue`ed and thus the process starts over.
        loop {
            // Check if the cache contains an entry for the query packet
            let cache_entry = match self.cache.get(question) {
                None => return None,
                Some(entry) => entry.value().clone(),
            };

            let cache_data = cache_entry.get_data();

            match cache_data {
                CacheData::RRSet(rr_set) => {
                    // Check TTL validity
                    let time_committed = *cache_entry.get_timestamp();
                    let duration_since_commit = match SystemTime::now()
                        .duration_since(time_committed)
                    {
                        Err(_) => {
                            eprintln!(
                                "The value for the cache entry timestamp was later than the current wall clock time."
                            );
                            return None;
                        }
                        Ok(d) => d,
                    };

                    if duration_since_commit > Duration::from_secs(rr_set.get_ttl() as u64) {
                        println!("Found stale cache entry -> reporting cache-miss");
                        return None;
                    }

                    return Some(rr_set.get_records().clone());
                }
                CacheData::Ticket(t) => {
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
}
