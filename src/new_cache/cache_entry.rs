use std::time::SystemTime;

use tokio::sync::broadcast;

use crate::protocol::DnsRecord;

/// Represents data that is committed to the cache.
#[derive(Debug, Clone)]
pub struct CacheEntry {
    /// Timestamp at which the data was committed to cache.
    timestamp: SystemTime,
    /// The data
    data: CacheData,
}

impl CacheEntry {
    pub fn get_timestamp(&self) -> &SystemTime {
        &self.timestamp
    }

    pub fn get_data(&self) -> &CacheData {
        &self.data
    }
}

#[derive(Debug, Clone)]
pub enum CacheData {
    RRSet(RRSet),
    Ticket(CacheTicket),
}

#[derive(Debug, Clone)]
pub struct RRSet {
    ttl: u32,
    records: Vec<DnsRecord>,
}

impl RRSet {
    pub fn new(records: Vec<DnsRecord>) -> Self {
        Self {
            ttl: RRSet::get_shortest_ttl(&records),
            records,
        }
    }

    pub fn get_ttl(&self) -> u32 {
        self.ttl
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
