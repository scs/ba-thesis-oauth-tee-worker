extern crate sgx_tstd as std;
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
use std::string::{ToString, String};

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

    // Todo: respond to the AccessTokenRequest
    //let status_line = "HTTP/1.1 200 OK".to_string();
    //let token = "123456789";

    /*
    let mut headers = Vec::new();
    headers.push(format!("Set-Cookie: access_token={}; Path=/; Max-Age=3600", token));

    let header_string = headers.join("\r\n"); */

    //(format!("{}\r\n{}", status_line, header_string), body)
    ("HTTP/1.1 200 OK".to_string(), "Token stuff".to_string())
}
  
pub fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let request: Request = parse_request(request_line.as_str());

    println!("Method: {}, Path: {}, Args: {}", request.method, request.path, request.args);

    let (status_line, contents) = match request.path.as_str() {
        "/" => handle_root(),
        "/resource" => handle_resource(&request),
        "/token" => handle_token(&request),
        _ => ("HTTP/1.1 404 NOT FOUND".to_string(), HTML_404.to_string()),
    };


    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}