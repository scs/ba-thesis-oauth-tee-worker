extern crate sgx_tstd as std;
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
use std::string::{ToString, String};

use super::html_elements::*;
use super::types::*;

fn handle_root() -> (String, String) {
    ("HTTP/1.1 200 OK".to_string(), HTML_HELLO.to_string())
}
  
fn handle_service(request: &Request) -> (String, String) {
    // Check if access_token is set and valid
    /*if let Some(token) = request.cookies.get("access_token") {
        // Check if access_token is valid
        if is_valid_token(token) {
            // Access token is valid, handle the resource request
            // Request content from resource server
            return ("HTTP/1.1 200 OK".to_string(), HTML_RESOURCE_CONTENT.to_string());
        }
    }*/
    // Access token is not set or not valid, prompt for grant to access resource
    ("HTTP/1.1 200 OK".to_string(), HTML_AUTHORIZATION_PROMPT.to_string())
}

fn is_valid_token(token: &str) -> bool {
    // Implement logic to validate the token here
    true
}

fn handle_authorize(request: &Request) -> (String, String) {
    // Check if the request contains the grant access form submission
    if request.method == "POST" {
        // Check grant code
        // Access request is granted
        // Create AccessTokenRequest
        // ...

        // Return a success response
        // Should ideally redirect to given URI
        return ("HTTP/1.1 200 OK".to_string(), "Access Granted".to_string());
    }
    ("HTTP/1.1 400 BAD".to_string(), "Bad Request".to_string())
}


pub fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    let request_line = buf_reader.lines().next().unwrap().unwrap();
  
    let request: Request = parse_request(request_line.as_str());
    
    println!("Method: {}, Path: {}, Args: {}", request.method, request.path, request.args);
    
    let (status_line, contents) = match request.path.as_str() {
        "/" => handle_root(),
        "/service" => handle_service(&request), // sollicitation of protected resource
        "/authorize" => handle_authorize(&request), // granting permission to request access to protected resource
        _ => ("HTTP/1.1 404 NOT FOUND".to_string(), HTML_404.to_string()),
    };

    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
  }
  