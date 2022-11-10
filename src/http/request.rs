use super::Method;

pub struct Request {
    method: Method,
    path: String,
    query_string: Option<String>,
}

impl Request {
    pub fn new(method: Method, path: String, query_string: Option<String>) -> Self {
        Self { method, path, query_string }
    }
}