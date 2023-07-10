extern crate sgx_tstd as std;
use std::collections::HashMap;
use std::io::BufReader;
use std::net::TcpStream;
use std::vec::Vec;
use std::string::String;

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

fn parse_access_token_request(buf_reader: &mut BufReader<&TcpStream>) -> AccessTokenRequest {
    let request = parse_request(buf_reader);

    let grant_type = request
        .headers
        .get("grant_type")
        .expect("Missing grant_type header")
        .parse()
        .expect("Invalid grant_type");
    let client_id = request
        .headers
        .get("client_id")
        .expect("Missing client_id header")
        .to_owned();
    let client_secret = request
        .headers
        .get("client_secret")
        .expect("Missing client_secret header")
        .to_owned();
    let username = request
        .headers
        .get("username")
        .expect("Missing username header")
        .to_owned();
    let password = request
        .headers
        .get("password")
        .expect("Missing password header")
        .to_owned();

    AccessTokenRequest {
        request,
        grant_type,
        client_id,
        client_secret,
        username,
        password,
    }
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

fn parse_access_token_response(buf_reader: &mut BufReader<&TcpStream>) -> AccessTokenResponse {
    // Parse the HTTP response parts (response line, headers, and body)
    let response = parse_response(buf_reader);

    // Extract necessary fields from the response
    let access_token = response
        .body
        .get("access_token")
        .and_then(|val| val.as_str())
        .expect("Missing or invalid access_token field in response body")
        .to_owned();
    let token_type = response
        .body
        .get("token_type")
        .and_then(|val| val.as_str())
        .expect("Missing or invalid token_type field in response body")
        .and_then(|error_str| TokenType::from_str(error_str))
        .expect("Failed to parse token_type field");
    let expires_in = response
        .body
        .get("expires_in")
        .and_then(|val| val.as_u64())
        .expect("Missing or invalid expires_in field in response body");

    AccessTokenResponse {
        response,
        access_token,
        token_type,
        expires_in,
    }
}

fn parse_error_response(buf_reader: &mut BufReader<&TcpStream>) -> ErrorResponse {
    // Parse the HTTP response parts (response line, headers, and body)
    let response = parse_response(buf_reader);

    // Extract necessary fields from the response
    let error = response
        .body
        .get("error")
        .and_then(|val| val.as_str())
        .expect("Missing or invalid error field in response body")
        .and_then(|error_str| ErrorCode::from_str(error_str))
        .expect("Failed to parse error field");
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
        response,
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
    // Read the content length header
    let content_length: usize = buf_reader
        .get_ref()
        .metadata()
        .map(|md| md.len() as usize)
        .unwrap_or(0);

    let mut body = vec![0; content_length];
    buf_reader.read_exact(&mut body).expect("Failed to read response body");

    serde_json::from_slice(&body).expect("Failed to parse response body as JSON")
}
