#[derive(thiserror::Error, Debug)]
pub enum NetworkError {
    #[error("An inbound connection could not be accepted because the system is at capacity.")]
    NoPermitAvailable,

    #[error("The semaphore tracking active connections/queries is closed.")]
    ConnectionSemaphoreClosed,

    #[error("An error occurred while accepting an inbound connection: {0}")]
    ConnectionAcceptError(#[from] tokio::io::Error),

    #[error("An error occurred while reading a TCP stream: {0}")]
    TcpStreamReadError(#[from] tokio::io::Error),

    #[error("An error occurred while writing to a TCP stream.")]
    TcpStreamWriteError(#[from] tokio::io::Error),

    // The above errors are likely because a single From/Into impl can only be applied to one type.
    // However, we want to distinguish different IO errors based on where they occurred.
    // Therefore, we should find a way to separate them. Possibly, by creating a new sub enum that implements the From trait for io::Error
}
