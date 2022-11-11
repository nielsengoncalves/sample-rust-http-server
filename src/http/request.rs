use super::error::ParseError;
use std::convert::TryFrom;
use std::str;
use strum_macros::EnumString;

#[derive(Debug, EnumString)]
pub enum Method {
    #[strum(ascii_case_insensitive)]
    Get,
    #[strum(ascii_case_insensitive)]
    Head,
    #[strum(ascii_case_insensitive)]
    Post,
    #[strum(ascii_case_insensitive)]
    Put,
    #[strum(ascii_case_insensitive)]
    Delete,
    #[strum(ascii_case_insensitive)]
    Connect,
    #[strum(ascii_case_insensitive)]
    Options,
    #[strum(ascii_case_insensitive)]
    Trace,
    #[strum(ascii_case_insensitive)]
    Patch,
}

#[derive(Debug, EnumString)]
pub enum Protocol {
    #[strum(serialize = "HTTP/1.1")]
    Http1_1,
}

#[derive(Debug)]
pub struct Header {
    name: String,
    value: String,
}

#[derive(Debug)]
pub struct Query {
    name: String,
    values: Vec<String>,
}

#[derive(Debug)]
pub struct Path {
    path: String,
    queries: Option<Vec<Query>>,
}

impl TryFrom<&str> for Path {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // TODO: Fix query parser
        Ok(Self {
            path: value.to_string(),
            queries: Option::None,
        })
    }
}

#[derive(Debug)]
pub struct Request {
    protocol: Protocol,
    method: Method,
    path: Path,
    headers: Option<Vec<Header>>,
}

impl TryFrom<&[u8; 1024]> for Request {
    type Error = ParseError;

    fn try_from(value: &[u8; 1024]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(value)?;

        let (method_str, request) = get_next_token(&request).ok_or(ParseError::InvalidMethod)?;
        let method: Method = method_str
            .try_into()
            .map_err(|_| ParseError::InvalidMethod)?;

        let (path_str, request) = get_next_token(&request).ok_or(ParseError::InvalidPath)?;
        let path: Path = path_str.try_into()?;

        let (protocol_str, _) = get_next_token(&request).ok_or(ParseError::InvalidProtocol)?;
        let protocol: Protocol = protocol_str
            .try_into()
            .map_err(|_| ParseError::InvalidProtocol)?;

        // TODO: Fix headers parser
        Ok(Self {
            protocol,
            method,
            path,
            headers: Option::None,
        })
    }
}

fn get_next_token(request: &str) -> Option<(&str, &str)> {
    for (i, char) in request.chars().enumerate() {
        if char == ' ' || char == '\r' || char == '\n' {
            return Some((&request[..i], &request[i + 1..]));
        }
    }
    return None;
}
