use std::time::{Duration, Instant};

use tokio::sync::broadcast;

use crate::protocol::DnsRecord;

#[derive(Debug, Clone)]
pub enum CacheEntry {
    RRSet(RRSet),
    Ticket(CacheTicket),
}

#[derive(Debug, Clone)]
pub struct RRSet {
    ttl: Instant,
    records: Vec<DnsRecord>,
}

impl RRSet {
    pub fn new(records: Vec<DnsRecord>) -> Self {
        Self {
            ttl: Instant::now() + Duration::from_secs(RRSet::get_shortest_ttl(&records) as u64),
            records,
        }
    }

    pub fn get_ttl(&self) -> &Instant {
        &self.ttl
    }

    pub fn get_records(&self) -> &Vec<DnsRecord> {
        &self.records
    }

    fn get_shortest_ttl(records: &Vec<DnsRecord>) -> u32 {
        let mut shortest_ttl = u32::MAX;

        for record in records {
            if record.ttl < shortest_ttl {
                shortest_ttl = record.ttl;
            }
        }

        shortest_ttl
    }
}

#[derive(Debug, Clone)]
pub struct CacheTicket {
    sender: broadcast::Sender<Vec<DnsRecord>>,
}

impl CacheTicket {
    pub fn new(sender: broadcast::Sender<Vec<DnsRecord>>) -> Self {
        Self { sender }
    }

    pub fn get_sender(&self) -> &broadcast::Sender<Vec<DnsRecord>> {
        &self.sender
    }
}
