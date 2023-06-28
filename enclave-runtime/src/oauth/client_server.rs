extern crate sgx_tstd as std;
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
use std::string::{ToString, String};
use std::collections::HashMap;

use super::html_elements::*;
use super::types::*;

fn handle_root() -> (String, String) {
    ("HTTP/1.1 200 OK".to_string(), HTML_HELLO.to_string())
}
  
fn handle_service(request: &Request) -> (String, String) {
    // Check if access_token is set
    if let Some(token) = request.cookies.get(&String::from("access_token")) {
        // Check if token is valid and not void
        if token == "test_value" {
            // If Access token is valid, handle the resource request
            // Proceed to request content from resource server
            return ("HTTP/1.1 200 OK".to_string(), HTML_RESOURCE_CONTENT.to_string());
        }
    }
    // Access token is not set or not valid, prompt for grant to access resource
    let cookie_header = "Set-Cookie: access_token=test_value; Path=/";

    let html_content = format!(
        "{}\r\n{}",
        "HTTP/1.1 200 OK",
        cookie_header
    );

    (html_content, HTML_AUTHORIZATION_PROMPT.to_string())
}

fn is_valid_token(token: &str) -> bool {
    // Implement logic to validate the token here
    true
}

fn handle_authorize(request: &Request) -> (String, String) {
    // Check if the request contains the username + password form submission
    if request.method == "POST" {
        // Get the values from the arguments
        let mut params: HashMap<&str, &str> = HashMap::new();

        for param_str in request.args.split('&') {
            let mut param_parts = param_str.split('=');
            let key = param_parts.next().unwrap_or("");
            let value = param_parts.next().unwrap_or("");
            params.insert(key, value);
        }

        if let (Some(username), Some(password)) = (params.get("username"), params.get("password")) {
            // Access request is granted
            // Create AccessTokenRequest with username and password
            // ...

            // Return a success response
            // Should ideally redirect to given URI
        } else {
            // Redirect to `/service`
            let redirect_response = ("HTTP/1.1 302 Found\nLocation: /service".to_string(), "Something went wrong...".to_string());
            return redirect_response;
        }
        return ("HTTP/1.1 200 OK".to_string(), "Access Granted".to_string());
    }
    ("HTTP/1.1 400 BAD".to_string(), "Bad Request".to_string())
}


pub fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request: Request = parse_request(buf_reader);
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
  