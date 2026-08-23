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

use crate::protocol;

/// This is the central handler of a query through its flow through the system.
/// It acts the routing instance passing the query to all relevant subsystems like the local zone authority
/// and the sinkhole before forwarding it to an upstream resolver.
///
/// As this function is the global handler, a query can be considered dealt with when this function returns.
pub fn handle(packet: Vec<u8>, src_addr: SocketAddr) {
    // 1. Deserialize
    let message = match protocol::deserialize(&packet) {
        Ok(m) => m,
        Err(e) => {
            // Analyze the error and react accordingly
            // This is the highest level of error handling.
            // Consequently, there are two options for handling them:
            //
            //      1. Communicate them to the client
            //      2. Drop the query without informing the client
            //
            // Errors are categorized by the nature of how the service intends to communicate them to the client.
            // They can thus be sorted into the following categories:
            //
            //      1. FORMERR
            //      The error was caused by a malformed packet and the service intends to communicate
            //      to the client that the received packet could not be understood.
            //      Note, that the service may elect to not communicate a malformed packet
            //      to the client either for security reasons or to save resources. In case of the former
            //      the error is automatically categorized as Category 4.
            //
            //      2. SERVFAIL
            //      The error was caused by a failure in the system (bug).
            //      As these types of errors are unexpected, this is a critical condition that should be investigated.
            //
            //      3. NOTIMP
            //      The packet was understood by the client, but the service does not support the operation.
            //
            //      4. DROP
            //      The error results in the query being dropped. This may be due to a unrecoverable state or by policy.
            //
            // These categories refer to the DNS response code that should be used in the response.
            todo!()
        }
    };

    println!("{:#?}", message);

    // 2. Local Zone Authority

    // 3. Sinkhole

    // 4. Network (integrated cache)
}
