use std::io;

use thiserror::Error;

pub type Result<T> = std::result::Result<T, ParseError>;

#[derive(Error, Debug)]
pub enum ParseError {
    // #[error("...")]
    // InvalidRowLength(usize, String),

    // #[error("...")]
    // BagDuplicate(),

    // #[error("...")]
    // InvalidChar(ParseCharError, String),

    // #[error("...")]
    // CharNotAllowed(String),

    // #[error("...")]
    // InvalidU8(ParseIntError, String),

    #[error("...")]
    InvalidMultiplier(String),

    #[error(transparent)]
    Io(#[from] io::Error),

    #[error(transparent)]
    Csv(#[from] csv::Error),

    #[error(transparent)]
    JsonError(#[from] serde_json::Error)
}
