/// The sum type of all possible errors that can exists within the application.
///
/// This custom implementation exists for a couple of reasons:
/// 1. To have a centralized place where all possible errors states of the application are hosted. This makes it easy to document and maintain error states.
/// 2. To ensure no dynamic dispatch ever happens during error handling and still being able to use the `?` operator.
///
/// ## General Considerations
/// Before adding a new error here, consider if you could also use an [Option] instead. <br>
/// Errors are intended to represent a state from which the application cannot continue on the [happy path](https://en.wikipedia.org/wiki/Happy_path). E.g. the service cannot fulfill the DNS request.
#[derive(Debug, PartialEq, Eq)]
pub enum DnsError {
    /// Indicates: The received packet is shorter than 12 bytes and can thus not make up a valid DNS packet
    PacketTooShort,
    /// Indicates: Attempted to start deserializing a packet from the network while the internal position of the cursor was not 0.
    /// This indicates that the deserialization process may not have been called in the correct order.
    CursorPositionNotAtZeroWhileDeserializingHeader,
    /// Indicates: Reading the requested amount of bytes from the cursor would have resulted in an out of bounds error.
    CursorOutOfBounds,
    /// Indicates: A message compression pointer was encountered during deserialization of a domain name that is invalid.
    IllegalPointer,
    /// Indicates: There are too many pointers in a singular domain name.
    /// This value is set inside the private function `crate::protocol::deserialize_domain_name()`.
    PointerJumpThresholdExceeded,
    /// Indicates: The domain name in question exceeds the maximum allowed length of 255 bytes.
    DomainNameTooLong,
    /// Indicates: A domain name label (the section between two dots) exceeded the maximum allowed length of 63 bytes.
    DomainNameLabelTooLong,
    /// Indicates: A DNS query with OPCODE = 0 contained more than one question. This directly violates RFC 9619.
    TooManyQuestions,
    /// Indicates: We don't support the received OPCODE-QDCOUNT combination.
    UnsupportedQuestionCount { opcode: u8, count: u16 },
}

/// Convenience type alias to [`std::result::Result<T, E>`] where `E` is defined as [`self::Error`].
pub type DnsResult<T> = std::result::Result<T, DnsError>;
