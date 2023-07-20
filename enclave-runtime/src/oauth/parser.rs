extern crate sgx_tstd as std;
use std::collections::HashMap;
use std::io::{BufReader, BufRead, Read};
use std::net::TcpStream;
use std::vec::Vec;
use std::string::{String, ToString};
use std::str::FromStr;
use std::borrow::ToOwned;
use std::io::ErrorKind;
use url::form_urlencoded;

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

    let mut method = HttpMethod::Get;
    let mut path = String::new();
    let mut http_version = String::new();

    let parts: Vec<&str> = line.split_whitespace().collect();

    for (index, part) in parts.iter().enumerate() {
        match index {
            0 => method = HttpMethod::from_str(part).expect("Invalid HTTP method"),
            1 => path = part.to_string().to_owned(),
            2 => http_version = part.to_string().to_owned(),
            _ => {} // we might as well ignore the restf
        }
    }

    if parts.len() == 2 {
        http_version = "undefined".to_owned();
    }

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

    let grant_type = GrantType::from_str(grant_type_str.as_str()
                                    .ok_or((ErrorCode::InvalidGrant, "Field grant_type is invalid", "https://datatracker.ietf.org/doc/html/rfc6749#section-4"))
                                    .unwrap())
                                .unwrap();

    let client_id = request
            .body
            .get("client_id")
            .ok_or((ErrorCode::InvalidClient, "Missing client_id header", "https://datatracker.ietf.org/doc/html/rfc6749#section-4.1.1"))
            .unwrap().as_str().unwrap();

    let client_secret = request
            .body
            .get("client_secret")
            .ok_or((ErrorCode::InvalidClient, "Missing client_secret header", "https://datatracker.ietf.org/doc/html/rfc6749#section-4.1.1"))
            .unwrap().as_str().unwrap();

    let username = request
            .body
            .get("username")
            .ok_or((ErrorCode::InvalidGrant, "Missing username header", "https://datatracker.ietf.org/doc/html/rfc6749#section-4.3"))
            .unwrap().as_str().unwrap();

    let password = request
            .body
            .get("password")
            .ok_or((ErrorCode::InvalidGrant, "Missing password header", "https://datatracker.ietf.org/doc/html/rfc6749#section-4.3"))
            .unwrap().as_str().unwrap();

    Ok(AccessTokenRequest {
        request: request.clone(),
        grant_type,
        client_id: client_id.to_string(),
        client_secret: client_secret.to_string(),
        username: username.to_string(),
        password: password.to_string(),
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

pub fn parse_error_response(response: &Response) -> ErrorResponse {
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
    match buf_reader.read_to_end(&mut body) {
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
        return serde_json::json!({});
    }

    // Check if the body is a form data
    let body_str = String::from_utf8_lossy(&body);
    if body_str.contains('=') && body_str.contains('&') {
        // Parse the body as URL-encoded form data
        let form_data: HashMap<String, String> = form_urlencoded::parse(body_str.as_bytes())
            .into_owned()
            .collect();
        
        // Convert the form data to a JSON object
        serde_json::json!(form_data)
    } else {
        // Parse the body as JSON
        serde_json::from_str(&body_str).expect("Failed to parse response body as JSON")
    }
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