/// Simple enum representing UDP and TCP.
///
/// This is used to indicate whether a query was received over UDP or TCP and is used to determine which
/// protocol should be used to exit the query. Either to an upstream resolver or to the downstream client.
#[derive(Debug, Clone)]
pub enum TransmissionProtocol {
    UDP,
    TCP,
}
