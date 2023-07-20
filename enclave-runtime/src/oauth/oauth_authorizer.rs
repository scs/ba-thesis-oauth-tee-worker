extern crate sgx_tstd as std;
use std::io::Write;
use std::net::TcpStream;
use std::string::ToString;

use super::types::*;
use super::parser::*;
use super::tools::*;
use super::token::*;
use super::token_base::*;


/// The authorization server must respond to two routes:
/// /resource       => for delivering the resource
/// /token          => for delivering/validating a token 
pub fn handle_connection(mut stream: TcpStream) {
    let request: Request = parse_request(&stream);
    println!("[AUTHOR]: Received:\t\t {:?}", request.request_line);

    let result = match request.request_line.path.as_str() {
        "/resource" => handle_resource(&request),
        "/token" => handle_token(&request),
        "/expiry" => handle_expiry(&request),
        _ => Ok(handle_404(&request)),
    };

    match result {
        Ok(response) => {
            println!("[AUTHOR]: Responding:\t {:?}", response.response_line);
            stream.write_all(response.to_string().as_bytes()).unwrap();
        }
        Err(error_response) => {
            println!("[AUTHOR]: Error:\t\t {:?}", error_response);
            stream.write_all(error_response.to_string().as_bytes()).unwrap();
        }
    }
}

fn handle_resource(request: &Request) -> Result<Response, ErrorResponse> {
    // Bruteforce attack possible (spamming with keys)
    // Solution: Connection refusal system
    // DDOS-Mitigation see https://en.wikipedia.org/wiki/DDoS_mitigation 
    let access_token = match request.headers.get("Cookie") {
        Some(cookie_header) => {
            let cookie = parse_cookie_header(cookie_header);
            cookie.get("access_token").cloned()
        }
        None => None,
    };

    match access_token {
        Some(token) => {
            if get_token_validity(&token) {
                println!("[AUTHOR]: Validated the token: {}", token);
                println!("[AUTHOR]: It expires at: {:?}", get_token_expiry(&token).unwrap());
                Ok(resource_response())
            } else {
                Err(invalid_token_response())
            }
        }
        None => Err(access_denied_response()),
    }
}

fn handle_token(request: &Request) -> Result<Response, ErrorResponse> {
    match parse_access_token_request(request) {
        Ok(access_token_request) => {
            match validate_access_token_request(&access_token_request) {
                Ok(()) => {
                    Ok(access_token_response())
                }
                Err((error, error_description, error_uri)) => {
                    Err(error_response(error,
                                        error_description,
                                        error_uri))
                }
            }
        }
        Err((error, error_description, error_uri)) => {
            Err(error_response(error,
                                error_description,
                                error_uri))
        }
    }
}

fn handle_expiry(request: &Request) -> Result<Response, ErrorResponse> {
    // Bruteforce attack possible (spamming with keys)
    // Solution: Connection refusal system
    // DDOS-Mitigation see https://en.wikipedia.org/wiki/DDoS_mitigation 
    let access_token = match request.headers.get("Cookie") {
        Some(cookie_header) => {
            let cookie = parse_cookie_header(cookie_header);
            cookie.get("access_token").cloned()
        }
        None => None,
    };

    match access_token {
        Some(token) => {
            
            if get_token_validity(&token) {
                println!("[AUTHOR]: Validated the token: {}", token);
                println!("[AUTHOR]: It expires at: {:?}", get_token_expiry(&token).unwrap());
                Ok(expiry_response(&token))
            } else {
                Err(invalid_token_response())
            }
        }
        None => Err(access_denied_response()),
    }
}