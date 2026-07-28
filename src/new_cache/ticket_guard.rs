use std::sync::Arc;

use dashmap::DashMap;
use tokio::sync::broadcast;

use crate::{
    new_cache::cache_entry::{CacheData, CacheEntry},
    protocol::{DnsQuestion, DnsRecord},
};

pub struct CacheTicketGuard {
    cache: Arc<DashMap<DnsQuestion, CacheEntry>>,
    question: DnsQuestion,
    sender: broadcast::Sender<Vec<DnsRecord>>,
}

impl CacheTicketGuard {
    pub fn new(
        cache: Arc<DashMap<DnsQuestion, CacheEntry>>,
        question: DnsQuestion,
        sender: broadcast::Sender<Vec<DnsRecord>>,
    ) -> Self {
        Self {
            cache,
            question,
            sender,
        }
    }

    pub fn get_sender(&self) -> &broadcast::Sender<Vec<DnsRecord>> {
        &self.sender
    }
}

impl Drop for CacheTicketGuard {
    fn drop(&mut self) {
        // The guard is dropped and thus we need to remove the ticket from the cache.

        // We only remove a ticket, not a valid RRSet.
        self.cache
            .remove_if(&self.question, |_, entry| match entry.get_data() {
                CacheData::Ticket(_) => true,
                CacheData::RRSet(_) => false,
            });
    }
}
