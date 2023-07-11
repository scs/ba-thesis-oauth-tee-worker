extern crate sgx_tstd as std;
use std::collections::HashMap;
use std::io::{BufReader, BufRead, Read};
use std::net::TcpStream;
use std::vec::Vec;
use std::string::{String, ToString};
use std::str::FromStr;
use std::borrow::ToOwned;
use std::io::ErrorKind;

use super::types::*;

/************************************\
 *             Request              *
\************************************/

pub fn parse_request(stream: &TcpStream) -> Request {
    let mut buf_reader = BufReader::new(stream);
    let request_line = parse_request_line(&mut buf_reader);
    let headers = parse_headers(&mut buf_reader);
    let body = parse_body(&mut buf_reader);
    
    Request {
        request_line,
        headers,
        body,
    }
}

fn parse_request_line(buf_reader: &mut BufReader<&TcpStream>) -> RequestLine {
    let mut line = String::new();
    buf_reader.read_line(&mut line).expect("Failed to read request line");

    let parts: Vec<&str> = line.split_whitespace().collect();
    assert!(parts.len() >= 3, "Invalid request line");

    let method = HttpMethod::from_str(parts[0]).expect("Invalid HTTP method");
    let path = parts[1].to_owned();
    let http_version = parts[2].to_owned();

    RequestLine {
        method,
        path,
        http_version,
    }
}

pub fn parse_access_token_request(request: &Request) -> Result<AccessTokenRequest, (ErrorCode, String, String)> {
    let grant_type_str = request
        .body
        .get("grant_type")
        .ok_or((ErrorCode::InvalidGrant, "Missing grant_type header", "https://datatracker.ietf.org/doc/html/rfc6749#section-4"))
        .unwrap();

    let mut grant_type = GrantType::from_str(&grant_type_str.as_str()
                                    .ok_or((ErrorCode::InvalidGrant, "Field grant_type is invalid", "https://datatracker.ietf.org/doc/html/rfc6749#section-4"))
                                    .unwrap())
                        .unwrap();

    let client_id = request
        .body
        .get("client_id")
        .ok_or((ErrorCode::InvalidClient, "Missing client_id header", "https://datatracker.ietf.org/doc/html/rfc6749#section-4.1.1"))
        .unwrap()
        .to_string();

    let client_secret = request
        .body
        .get("client_secret")
        .ok_or((ErrorCode::InvalidClient, "Missing client_secret header", "https://datatracker.ietf.org/doc/html/rfc6749#section-4.1.1"))
        .unwrap()
        .to_string();

    let username = request
        .body
        .get("username")
        .ok_or((ErrorCode::InvalidGrant, "Missing username header", "https://datatracker.ietf.org/doc/html/rfc6749#section-4.3"))
        .unwrap()
        .to_string();

    let password = request
        .body
        .get("password")
        .ok_or((ErrorCode::InvalidGrant, "Missing password header", "https://datatracker.ietf.org/doc/html/rfc6749#section-4.3"))
        .unwrap()
        .to_string();

    Ok(AccessTokenRequest {
        request: request.clone(),
        grant_type: grant_type.clone(),
        client_id,
        client_secret,
        username,
        password,
    })
}




/************************************\
 *             Response             *
\************************************/

pub fn parse_response(stream: &TcpStream) -> Response {
    let mut buf_reader = BufReader::new(stream);
    let response_line = parse_response_line(&mut buf_reader);
    let headers = parse_headers(&mut buf_reader);
    let body = parse_body(&mut buf_reader);
    
    Response {
        response_line,
        headers,
        body,
    }
}

fn parse_response_line(buf_reader: &mut BufReader<&TcpStream>) -> ResponseLine {
    let mut line = String::new();
    buf_reader.read_line(&mut line).expect("Failed to read response line");

    let parts: Vec<&str> = line.split_whitespace().collect();
    assert!(parts.len() >= 3, "Invalid response line");

    let http_version = parts[0].to_owned();
    let status_code: u64 = parts[1].parse().expect("Invalid status code");
    let response_type = HttpResponseType::from(status_code);

    ResponseLine {
        http_version,
        status_code,
        response_type,
    }
}

fn parse_access_token_response(response: &Response) -> AccessTokenResponse {
    let access_token = response
        .body
        .get("access_token")
        .and_then(|val| val.as_str())
        .expect("Missing or invalid access_token field in response body")
        .to_owned();
    let token_type_str = response
        .body
        .get("token_type")
        .expect("Missing token_type field in response body");

    let token_type = TokenType::from_str(token_type_str.as_str().unwrap())
        .expect("Invalid token_type");
    
    let expires_in = response
        .body
        .get("expires_in")
        .and_then(|val| val.as_u64())
        .expect("Missing or invalid expires_in field in response body");

    AccessTokenResponse {
        response: response.clone(),
        access_token,
        token_type,
        expires_in,
    }
}

fn parse_error_response(response: &Response) -> ErrorResponse {
    let error_str = response
        .body
        .get("error")
        .and_then(|val| val.as_str())
        .expect("Missing or invalid error field in response body");

    let error = ErrorCode::from_str(error_str).expect("Failed to parse error field");

    let error_description = response
        .body
        .get("error_description")
        .and_then(|val| val.as_str())
        .unwrap_or_default()
        .to_owned();
    let error_uri = response
        .body
        .get("error_uri")
        .and_then(|val| val.as_str())
        .unwrap_or_default()
        .to_owned();

    ErrorResponse {
        response: response.clone(),
        error,
        error_description,
        error_uri,
    }
}



/************************************\
 *             Generics             *
\************************************/

fn parse_headers(buf_reader: &mut BufReader<&TcpStream>) -> HashMap<String, String> {
    let mut headers = HashMap::new();
    let mut line = String::new();

    while buf_reader.read_line(&mut line).unwrap_or(0) > 2 {
        let parts: Vec<&str> = line.splitn(2, ':').map(|s| s.trim()).collect();
        if parts.len() == 2 {
            headers.insert(parts[0].to_owned(), parts[1].to_owned());
        }
        line.clear();
    }

    headers
}

fn parse_body(buf_reader: &mut BufReader<&TcpStream>) -> serde_json::Value {
    let mut body = Vec::new();
    match buf_reader.read(&mut body) {
        Ok(_) => {
            // Successfully read the response body
        }
        Err(ref error) if error.kind() == ErrorKind::WouldBlock => {
            // Read timed out - buffer is probably done
        }
        Err(error) => {
            panic!("Failed to read response body: {:?}", error);
        }
    }

    if body.is_empty() {
        return serde_json::Value::Object(serde_json::Map::new());
    }

    serde_json::from_slice(&body).expect("Failed to parse response body as JSON")
}

pub fn parse_cookie_header(header: &str) -> HashMap<String, String> {
    let mut cookies = HashMap::new();
    for cookie_str in header.split(';') {
        let mut parts = cookie_str.trim().splitn(2, '=');
        if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
            cookies.insert(key.to_string(), value.to_string());
        }
    }
    cookies
}
