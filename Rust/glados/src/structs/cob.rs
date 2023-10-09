use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum GLaDOSError {
    #[error("Failed Generic")]
    ERROR
}