use crate::protocol::packet::DnsPacket;

/// Returned by [`crate::cache::Cache`] methods whenever an error occurs while operating on the cache.
#[derive(Debug)]
pub struct CacheError {
    /// The message associated with the error.
    message: String,

    /// The [`crate::protocol::packet::DnsPacket`] upon which the error occurred.
    packet: DnsPacket,
}
