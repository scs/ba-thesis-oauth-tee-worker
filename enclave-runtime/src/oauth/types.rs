extern crate sgx_tstd as std;
//use http::header::{HeaderMap, InvalidHeaderValue};
use std::vec::Vec;
use std::fmt;
use std::string::{String, ToString};
use std::io::{prelude::*, BufReader, ErrorKind};
use std::net::{TcpStream};
use std::collections::HashMap;
use serde_json::Value;

use crate::oauth::client_config::ACCESS_TOKEN;

pub struct Route {
    pub method: String,
    pub path: String,
    pub handler: fn() -> (String, String),  // Handler function signature
}

#[derive(Debug)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub args: String,
    pub cookies: HashMap<String,String>,
    pub body: serde_json::Value,
}

#[derive(Debug)]
pub struct AccessTokenRequest {
    pub request: Request,
    pub grant_type: String,
    pub client_id: String,
    pub username: String,
    pub password: String,
    pub redirect_uri: String,
}

#[derive(Debug)]
pub struct AccessTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
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

pub fn parse_request(stream: &TcpStream) -> Request {
    let mut buf_reader = BufReader::new(stream);

    let mut cookie_line = String::new();
    let mut request_line = String::new();
    let mut content_length: Option<usize> = None;
    let mut content_type: Option<String> = None;
    let mut body = String::new();

    // Hardcoded Http Interpreting
    for (index, line_wrap) in buf_reader.lines().enumerate() {
        let line = match line_wrap {
            Ok(line) => line,
            Err(e) => {
                // Check for timeout error and break the loop
                // Dirty hack to make the loop break when there is no body
                if e.kind() == ErrorKind::WouldBlock {
                    break;
                }
                "".to_string()
            }
        };

        if line.is_empty() {
            break;
        }

        if index == 0 {
            request_line = line.clone();
            continue;
        } else if line.starts_with("Cookie: ") {
            cookie_line = line.clone();
            continue;
        } else if line.starts_with("Content-Length:") {
            let length_str = line.trim_start_matches("Content-Length:").trim();
            if let Ok(length) = length_str.parse::<usize>() {
                content_length = Some(length);
                continue;
            }
        } else if line.starts_with("Content-Type:") {
            content_type = Some(line.trim_start_matches("Content-Type:").trim().to_string());
            continue;
        }

        if content_length.is_some() && body.len() < content_length.unwrap() {
            body = line.clone();
            break;
        }
    }

    let mut method = String::new();
    let mut path = String::new();
    let mut args = String::new();
    let mut cookies = extract_cookies(&cookie_line);
    let mut body_json = serde_json::Value::Null;

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

    if body.is_empty() != true {
        let parsed_body: Result<Value, _> = serde_json::from_str(&body);
        
        if let Ok(value) = parsed_body {
            body_json = value;
        } else {
            // Handle parsing error
            if let Err(err) = parsed_body {
                println!("Failed to parse JSON: {}", err);
            }
        }
    }

    Request {
        method,
        path,
        args,
        cookies,
        body: body_json,
    }
}

pub fn get_request_field(body: &serde_json::Value, field: &str) -> String {
    if let Some(value) = body.get(field) {
        return value.to_string();
    } else {
        return "Error parsing json".to_string();
    }
}

pub fn validate_token_request(access_token_request: &AccessTokenRequest) -> Result<(), String> {

    // validation checks...
    // Todo cleint/user cred validation trennen!

    Ok(())
}

pub fn generate_access_token() -> String {

    // some token generation...
    ACCESS_TOKEN.to_string()
}

pub fn validate_token(token: &String) -> bool {
    // Lookup with token, check timestamp
    // Bruteforce attack possible (spamming with keys)
    // Solution: Connection refusal einbauen
    println!("---> [AUTHOR] Validated the token!");
    return true;
}