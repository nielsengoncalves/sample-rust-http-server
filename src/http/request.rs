use super::error::ParseError;
use std::collections::HashMap;
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

impl Query {
    pub fn new(name: String, values: Vec<String>) -> Self {
        Query { name, values }
    }
}

#[derive(Debug)]
pub struct Path {
    path: String,
    queries: Vec<Query>,
}

impl TryFrom<&str> for Path {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (path, queries) = split_path_and_query_string(&value)?;
        Ok(Self { path, queries })
    }
}

fn split_path_and_query_string(full_path: &str) -> Result<(String, Vec<Query>), ParseError> {
    for (i, char) in full_path.chars().enumerate() {
        if char == '?' {
            let path = &full_path[..i];
            let query_string = &full_path[i + 1..];
            let queries = get_queries(query_string)?;
            return Ok((path.to_string(), queries));
        }
    }

    Ok((full_path.to_string(), Vec::<Query>::new()))
}

fn get_queries(query_string: &str) -> Result<Vec<Query>, ParseError> {
    let mut queries_map: HashMap<String, Vec<String>> = HashMap::new();

    for query in query_string.split("&").into_iter() {
        let (key, value) = query.split_once("=").ok_or(ParseError::InvalidQuery)?;

        match queries_map.get_mut(key) {
            Some(values) => {
                values.push(value.to_string());
            }
            None => {
                queries_map.insert(key.to_string(), vec![value.to_string()]);
            }
        }
    }

    Ok(queries_map
        .into_iter()
        .map(|(key, values)| Query::new(key.to_string(), values.to_vec()))
        .collect())
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
