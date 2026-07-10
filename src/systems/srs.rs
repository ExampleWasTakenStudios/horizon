use std::time::Duration;

use crate::protocol::packet::DnsPacket;

/// An IES Resolve Command is sent to the IES to forward a packet to the specified resolver.
///
/// It contains the timeout after which the IES should clear the memory associated with the query and the `DnsPacket` representing the query.
pub struct IesResolveCommand {
    pub timeout: Duration,
    pub packet: DnsPacket,
}
