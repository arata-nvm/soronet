use std::collections::HashMap;

pub mod request;
pub mod response;
pub mod status;

pub type HttpHeaders = HashMap<String, String>;
