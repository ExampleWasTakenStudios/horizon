use std::net::SocketAddr;

use crate::protocol::packet::DnsPacket;

/// An IES Command is sent to the IES to forward a packet to the specified resolver.
pub struct IesCommand {
    pub resolver_address: SocketAddr,
    pub packet: DnsPacket,
}
