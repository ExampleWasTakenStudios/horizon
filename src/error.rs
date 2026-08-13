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
    // Indicates: The received packet is shorter than 12 bytes and can thus not make up a valid DNS packet
    PacketTooShort,
}

/// Convenience type alias to [`std::result::Result<T, E>`] where `E` is defined as [`self::Error`].
pub type DnsResult<T> = std::result::Result<T, DnsError>;
