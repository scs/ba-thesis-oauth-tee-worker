extern crate sgx_tstd as std;
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
use std::string::{ToString, String};
use serde_json::json;

use super::html_elements::*;
use super::types::*;

fn handle_root() -> (String, String) {
    ("HTTP/1.1 200 OK".to_string(), HTML_HELLO.to_string())
}

fn handle_resource(request: &Request) -> (String, String) {
    let mut token_valid = false;

    // Check if token is set and valid
    for (name, value) in &request.cookies {
        if name == "token" && value == "123456789" {
            token_valid = true;
            break;
        }
    }

    if token_valid {
        // Token is valid, perform resource handling logic
        // ...

        ("HTTP/1.1 200 OK".to_string(), HTML_RESOURCE_CONTENT.to_string())
    } else {
        // Token is not set or not valid
        ("HTTP/1.1 200 OK".to_string(), HTML_DENY_TEXT.to_string())
    }
}
  
fn handle_token(request: &Request) -> (String, String) {
    // GET requests should not mutate server state and are extremely
    // vulnerable accidental repetition as well as Cross-Site Request
    // Forgery (CSRF).

    let access_token_request = AccessTokenRequest {
        request: Request{
            method: request.method.clone(), 
            path: request.path.clone(),
            args: request.args.clone(), 
            cookies: request.cookies.clone(),
            body: serde_json::Value::Null,
        },
        grant_type: get_request_field(&request.body, "grant_type"),
        client_id: get_request_field(&request.body, "client_id"),
        username: get_request_field(&request.body, "username"),
        password: get_request_field(&request.body, "password"),
        redirect_uri: get_request_field(&request.body, "redirect_uri"),
    };

    match validate_token_request(&access_token_request) {
        Ok(()) => {

            let access_token = generate_access_token();
            let token_type = "Bearer".to_string();
            let expires_in = 3600; // Example expiration time in seconds

            // Create and return the access token response
            let access_token_response = AccessTokenResponse {
                access_token,
                token_type,
                expires_in,
            };

            let payload = json!({
                "access_token": json!(access_token_response.access_token),
                "token_type": json!(access_token_response.token_type),
                "expires_in": json!(access_token_response.expires_in),
            });
        
            return ("HTTP/1.1 200 OK".to_string(), payload.to_string());
        }
        Err(error) => {
            println!("Token request validation failed: {}", error);
            return ("HTTP/1.1 500 Internal Server Error".to_string(), "error".to_string());
        }
    }
}
  
pub fn handle_connection(mut stream: TcpStream) { 
    let request: Request = parse_request(&stream);
    
    println!("Method: {}, Path: {}, Args: {}", request.method, request.path, request.args);

    let (status_line, contents) = match request.path.as_str() {
        "/" => handle_root(),
        "/resource" => handle_resource(&request),
        "/token" => handle_token(&request),
        _ => ("HTTP/1.1 404 NOT FOUND".to_string(), HTML_404.to_string()),
    };

    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length:{length}\r\n{contents}\r\n\r\n");

    stream.write_all(response.as_bytes()).unwrap();
}