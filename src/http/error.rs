use std::convert::From;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str::Utf8Error;
use strum::EnumMessage;
use strum_macros;

#[derive(strum_macros::EnumMessage, Debug)]
pub enum ParseError {
    #[strum(message = "Invalid Request")]
    InvalidRequest,
    #[strum(message = "Invalid Protocol")]
    InvalidProtocol,
    #[strum(message = "Invalid Encoding")]
    InvalidEncoding,
    #[strum(message = "Invalid Method")]
    InvalidMethod,
    #[strum(message = "Invalid Path")]
    InvalidPath,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.get_message().unwrap_or(""))
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Error for ParseError {}
