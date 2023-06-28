extern crate sgx_tstd as std;
//use http::header::{HeaderMap, InvalidHeaderValue};
use std::vec::Vec;
use std::string::{String, ToString};

pub struct Route {
    pub method: String,
    pub path: String,
    pub handler: fn() -> (String, String),  // Handler function signature
}

pub struct Request {
    pub method: String,
    pub path: String,
    pub args: String,
    pub cookies: Vec<(String, String)>,
}

pub struct AcessTokenRequest {
    pub request: Request,
    pub grant_type: String,
    pub code: String,
    pub redirect_uri: String,
    pub client_id: String,
}

pub struct ErrorResponse {
    pub error: String,
    pub error_description: String,
    pub error_uri: String,
}

pub fn parse_request(request_line: &str) -> Request {
    let mut method = String::new();
    let mut path = String::new();
    let mut args = String::new();
    let mut cookies = Vec::new();  // Added cookies field

    if let Some(end_method) = request_line.find(' ') {
        method = request_line[..end_method].to_string();
    }

    if let Some(start_path) = request_line.find('/') {
        if let Some(end_path) = request_line[start_path..].find(|c| c == '?' || c == ' ') {
            path = request_line[start_path..start_path + end_path].to_string();
        }
    }

    if let Some(start_args) = request_line.find('?') {
        if let Some(end_args) = request_line[start_args..].find(' ') {
            args = request_line[start_args + 1..start_args + end_args].to_string();
        }
    }

    // Extract cookies
    let cookie_prefix = "Cookie: ";
    if let Some(cookie_start) = request_line.find(cookie_prefix) {
        let cookie_str = &request_line[cookie_start + cookie_prefix.len()..];
        let cookie_parts: Vec<&str> = cookie_str.split(';').collect();

        for cookie in cookie_parts {
            let cookie_parts: Vec<&str> = cookie.trim().split('=').collect();
            if cookie_parts.len() == 2 {
                let name = cookie_parts[0].trim().to_string();
                let value = cookie_parts[1].trim().to_string();
                cookies.push((name, value));
            }
        }
    }

    Request {
        method,
        path,
        args,
        cookies,
    }
}