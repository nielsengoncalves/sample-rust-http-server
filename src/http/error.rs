use std::convert::From;
use std::fmt::Debug;
use std::str::Utf8Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Invalid Request")]
    InvalidRequest,
    #[error("Invalid Protocol")]
    InvalidProtocol,
    #[error("Invalid Encoding")]
    InvalidEncoding,
    #[error("Invalid Method")]
    InvalidMethod,
    #[error("Invalid Path")]
    InvalidPath,
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}
