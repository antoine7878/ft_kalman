use std::fmt::Error as FmtError;
use std::num::ParseFloatError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum KalmanError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Message too long: {0}")]
    MessageTooLong(usize),
    #[error("Invalid utf8: {0}")]
    InvalidUtf8(#[from] std::str::Utf8Error),
    #[error("Float parsing: {0}")]
    FloatParsing(#[from] ParseFloatError),
    #[error("fmt error: {0}")]
    Fmt(#[from] FmtError),
    #[error("Parsing error")]
    Parsing,
    #[error("Inversion error")]
    Inversion(String),
}

impl From<&'static str> for KalmanError {
    fn from(s: &'static str) -> Self {
        KalmanError::Inversion(s.to_string())
    }
}
