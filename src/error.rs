use crate::protocol::RawMessage;

/// The sum type of all possible errors that can exists within the application.
///
/// This custom implementation exists to ensure no dynamic dispatch can ever happen during error handling and still being able to use the `?` operator.
///
/// ## General Considerations
/// Before adding a new error here, consider if you could also use and [Option] instead.
/// A good indicator that an `Option` might work better is when the errors have no internal state themselves.
/// However, one may still elect to use an error if the added context contributes to keeping mental capacity low.
#[derive(Debug)]
pub enum DnsError {
    /// Indicates: The received packet is shorter than 12 bytes and can thus not make up a valid DNS packet
    PacketTooShort(Box<RawMessage>),
    /// Indicates: Attempted to start deserializing a packet from the network while the internal position of the cursor was not 0.
    /// This indicates that the deserialization process may not have been called in the correct order.
    CursorPositionNotAtZeroWhileDeserializingHeader,
    /// Indicates: Reading the requested amount of bytes from the cursor would have resulted in an out of bounds error.
    CursorOutOfBounds,
    /// Indicates: A message compression pointer was encountered during deserialization of a domain name that is invalid.
    IllegalPointer,
    /// Indicates: There are too many pointers in a singular domain name.
    TooManyPointersInDomainName,
    /// Indicates: The domain name in question exceeds the maximum allowed length of 255 bytes.
    DomainNameTooLong,
    /// Indicates: A domain name label (the section between two dots) exceeded the maximum allowed length of 63 bytes.
    DomainNameLabelTooLong,
}

/// Convenience type alias to [`std::result::Result<T, E>`] where `E` is defined as [`self::Error`].
pub type DnsResult<T> = std::result::Result<T, DnsError>;
