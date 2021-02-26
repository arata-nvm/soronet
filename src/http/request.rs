use std::io::Read;

use super::HttpHeaders;

#[derive(Debug)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub target: String,
    pub version: String,
    pub headers: HttpHeaders,
    pub body: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum HttpMethod {
    Get,
    Head,
    Post,
    Put,
    Delete,
    Connect,
    Options,
    Trace,
}

pub fn parse<R: Read>(r: &mut R) -> HttpRequest {
    let mut buf_bytes = vec![0; 1024];
    r.read(&mut buf_bytes).unwrap();

    let buf = String::from_utf8_lossy(&buf_bytes);
    let mut lines = buf.split("\r\n");

    // request-line
    let request_line = lines.next().unwrap();
    let mut request_line_tokens = request_line.split_ascii_whitespace();
    let method = parse_http_method(request_line_tokens.next().unwrap());
    let target = request_line_tokens.next().unwrap().to_string();
    let version = request_line_tokens.next().unwrap().to_string();

    // header-field
    let mut headers = HttpHeaders::new();
    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }

        let (name, value) = parse_http_header(line);
        headers.insert(name, value);
    }

    // message-body
    let content_length = headers
        .get("Content-Length")
        .map_or(0, |v| v.parse::<usize>().unwrap());

    let body = if content_length != 0 {
        Some(lines.collect::<String>()[..content_length].to_string())
    } else {
        None
    };

    HttpRequest {
        method,
        target,
        version,
        headers,
        body,
    }
}

fn parse_http_method(s: &str) -> HttpMethod {
    match s {
        "GET" => HttpMethod::Get,
        "HEAD" => HttpMethod::Head,
        "POST" => HttpMethod::Post,
        "PUT" => HttpMethod::Put,
        "DELETE" => HttpMethod::Delete,
        "CONNECT" => HttpMethod::Connect,
        "OPTIONS" => HttpMethod::Options,
        "TRACE" => HttpMethod::Trace,
        x => panic!("'{}' is not a http method", x),
    }
}

fn parse_http_header(s: &str) -> (String, String) {
    let mut tokens = s.split(":");
    let name = tokens.next().unwrap().to_string();
    let value = tokens.next().unwrap().trim_start().to_string();
    (name, value)
}
