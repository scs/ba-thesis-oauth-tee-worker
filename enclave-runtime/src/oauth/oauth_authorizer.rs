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
    println!("[{}]: Received:\t\t {:?}", "AUTHOR", request.request_line);

    let result = match request.request_line.path.as_str() {
        "/resource" => handle_resource(&request),
        _ => Ok(handle_404(&request)),
    };

    match result {
        Ok(response) => {
            println!("[{}]: Responding:\t\t {:?}", "AUTHOR", response.response_line);
            stream.write_all(response.to_string().as_bytes()).unwrap();
        }
        Err(error_response) => {
            println!("[{}]: Error:\t\t {:?}", "AUTHOR", error_response);
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
            
            let is_valid_token = validate_token(&token);

            if is_valid_token {
                println!("[{}] Validated the token: ", "AUTHOR");
                println!("[{}] It expires at: {:?}", "AUTHOR", get_token_expiry(&token.as_str()));
                Ok(create_resource_response())
            } else {
                Ok(create_access_denied_response())
            }
        }
        None => Ok(create_access_denied_response()),
    }
}

fn handle_token(request: &Request) -> Result<Response, ErrorResponse> {
    match parse_access_token_request(request) {
        Ok(access_token_request) => {
            match validate_access_token_request(&access_token_request) {
                Ok(()) => {
                    Ok(create_access_token_response())
                }
                Err((error, error_description, error_uri)) => {
                    Err(create_error_response(error,
                                        error_description,
                                        error_uri))
                }
            }
        }
        Err((error, error_description, error_uri)) => {
            Err(create_error_response(error,
                                error_description,
                                error_uri))
        }
    }
}