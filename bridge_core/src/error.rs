pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("io operation failed")]
    FileWriteError(#[from] std::io::Error),
    #[error("ring operation failed")]
    RingUnspecified(#[from] ring::error::Unspecified),
    #[error("crypto keys were rejected")]
    KeyRejected(#[from] ring::error::KeyRejected),
    #[error("failed to decode pem key")]
    PemDecodingError(#[from] pem::PemError),
}
