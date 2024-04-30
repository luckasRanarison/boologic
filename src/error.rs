use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid token '{0}'")]
    InvalidToken(String),
    #[error("Expected '{0}'")]
    ExpectedSequence(String),
    #[error("Unexpected end of file")]
    UnexpectedEOF,
}

pub type Result<T> = std::result::Result<T, Error>;
