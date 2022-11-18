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
pub struct Header<'buf> {
    name: &'buf str,
    value: &'buf str,
}

#[derive(Debug)]
pub struct Query<'buf> {
    name: &'buf str,
    values: Vec<&'buf str>,
}

impl <'buf> Query <'buf> {
    pub fn new(name: &'buf str, values: Vec<&'buf str>) -> Self {
        Query { name, values }
    }
}

#[derive(Debug)]
pub struct Path<'buf> {
    path: &'buf str,
    queries: Vec<Query<'buf>>,
}

impl <'buf> TryFrom<&'buf str> for Path<'buf> {
    type Error = ParseError;

    fn try_from(value: &'buf str) -> Result<Self, Self::Error> {
        let (path, queries) = split_path_and_query_string(value)?;
        Ok(Self { path, queries })
    }
}

fn split_path_and_query_string(path_str: &str) -> Result<(&str, Vec<Query>), ParseError> {
    for (i, char) in path_str.chars().enumerate() {
        if char == '?' {
            let (path, query_string) = (&path_str[..i], &path_str[i + 1..]);
            let queries = parse_queries(query_string)?;
            return Ok((path, queries));
        }
    }

    Ok((path_str, Vec::<Query>::new()))
}

fn parse_queries(query_string: &str) -> Result<Vec<Query>, ParseError> {
    let mut queries_map: HashMap<&str, Vec<&str>> = HashMap::new();

    for query in query_string.split("&").into_iter() {
        let (key, value) = query.split_once("=").ok_or(ParseError::InvalidQuery)?;

        match queries_map.get_mut(key) {
            Some(values) => {
                values.push(value);
            }
            None => {
                queries_map.insert(key, vec![value]);
            }
        }
    }

    Ok(queries_map
        .into_iter()
        .map(|(key, values)| Query::new(key, values.to_vec()))
        .collect())
}

#[derive(Debug)]
pub struct Request<'buf> {
    protocol: Protocol,
    method: Method,
    path: Path<'buf>,
    headers: Option<Vec<Header<'buf>>>,
}

impl <'buf> TryFrom<&'buf [u8; 1024]> for Request<'buf> {
    type Error = ParseError;

    fn try_from(value: &'buf [u8; 1024]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(value)?;

        let (method_str, request) = get_next_token(&request).ok_or(ParseError::InvalidMethod)?;
        let method: Method = method_str.parse().map_err(|_| ParseError::InvalidMethod)?;

        let (path_str, request) = get_next_token(&request).ok_or(ParseError::InvalidPath)?;
        let path: Path = path_str.try_into().map_err(|_| ParseError::InvalidPath)?;

        let (protocol_str, _) = get_next_token(&request).ok_or(ParseError::InvalidProtocol)?;
        let protocol: Protocol = protocol_str.parse().map_err(|_| ParseError::InvalidProtocol)?;

        // TODO: Fix headers parser
        Ok(Self { protocol, method, path, headers: Option::None })
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
