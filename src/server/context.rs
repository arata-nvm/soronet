use std::collections::HashMap;

use crate::http::request::HttpRequest;

pub struct Context {
    pub request: HttpRequest,
    pub params: HashMap<String, String>,
}

impl Context {
    pub fn new(request: HttpRequest) -> Self {
        Self {
            request,
            params: HashMap::new(),
        }
    }
}
