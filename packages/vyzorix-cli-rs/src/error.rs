use thiserror::Error;

#[derive(Error, Debug)]
pub enum VyzoError {
    #[error("Configuration missing or invalid")]
    ConfigError,
    
    #[error("Network connection failed: {0}")]
    NetworkError(String),
    
    #[error("Authentication token expired")]
    AuthExpired,
    
    #[error("Cryptography error processing secure payload")]
    CryptoError,
    
    #[error("FileSystem I/O error: {0}")]
    IoError(#[from] std::io::Error),
}
