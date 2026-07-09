use std::{net::SocketAddr, time::Duration};

use crate::protocol::packet::DnsPacket;

/// An IES Resolve Command is sent to the IES to forward a packet to the specified resolver.
///
/// It contains the `SocketAddr` of the target upstream resolver as well as the timeout after which the IES should
/// clear the memory associated with the query and the `DnsPacket` representing the query.
pub struct IesResolveCommand {
    pub resolver_address: SocketAddr,
    pub timeout: Duration,
    pub packet: DnsPacket,
}
