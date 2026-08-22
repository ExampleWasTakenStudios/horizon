//! The central handling module of the service.
//!
//! This module acts as the global handler for the service.
//! Here, all comes together to handle a query according to the rules of the system.
//!
//! It is unopinionated about the background of the query. E.g. it understands DNS but does
//! not concern itself with the underlying network infrastructure or any of the business logic
//! of the service such as local zone authority or blocking queries. Instead, it knows about
//! the correct systems to handle the query accordingly and calls them accordingly.

use std::net::SocketAddr;

/// This is the central handler of a query through its flow through the system.
/// It acts the routing instance passing the query to all relevant subsystems like the local zone authority
/// and the sinkhole before forwarding it to an upstream resolver.
///
/// As this function is the global handler, a query can be considered dealt with when this function returns.
pub fn handle(packet: Vec<u8>, src_addr: SocketAddr) {
    // 1. Deserialize

    // 2. Local Zone Authority

    // 3. Sinkhole

    // 4. Network (integrated cache)
}
