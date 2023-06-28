extern crate sgx_tstd as std;
//use http::header::{HeaderMap, InvalidHeaderValue};
use std::vec::Vec;
use std::string::{String, ToString};
use std::io::{prelude::*, BufReader};
use std::net::{TcpStream};
use std::collections::HashMap;

pub struct Route {
    pub method: String,
    pub path: String,
    pub handler: fn() -> (String, String),  // Handler function signature
}

pub struct Request {
    pub method: String,
    pub path: String,
    pub args: String,
    pub cookies: HashMap<String,String>,
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

fn extract_cookies(cookie_line: &str) -> HashMap<String, String> {
    let cookie_line = cookie_line.trim_start_matches("Cookie: ");
    let mut cookies = HashMap::new();

    for cookie in cookie_line.split(';') {
        let parts: Vec<&str> = cookie.trim().splitn(2, '=').collect();
        if parts.len() == 2 {
            let key = parts[0].to_string();
            let value = parts[1].to_string();
            cookies.insert(key, value);
        }
    }
    cookies
}

pub fn parse_request(buf_reader: BufReader<&mut TcpStream>) -> Request {
    let mut cookie_line = String::new();
    let mut request_line = String::new();

    // Hardcoded Http Interpreting
    for (index, line_wrap) in buf_reader.lines().enumerate() {
        // First line is the request line e.g. "GET / HTTP/1.1"
        let line = line_wrap.unwrap();
        if line.is_empty() {
            // If there is more, it "should" be the body
            break;
        }
        if index == 0 {
            request_line = line;
            continue;
        } else if line.starts_with("Cookie: ") {
            cookie_line = line;
        }
    }

    let mut method = String::new();
    let mut path = String::new();
    let mut args = String::new();
    let mut cookies = extract_cookies(&cookie_line);

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
    
    println!("Cookies:");
    for (name, value) in &cookies {
        println!("{}: {}", name, value);
    };

    Request {
        method,
        path,
        args,
        cookies,
    }
}