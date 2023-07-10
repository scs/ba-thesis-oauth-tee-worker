extern crate sgx_tstd as std;
use std::net::TcpStream;
use std::string::{ToString, String};

use super::types::*;
use super::html_elements::HTML_404;



/// The authorization server must respond to two routes:
/// /resource       => for delivering the resource
/// /token          => for delivering/validating a token 
pub fn handle_connection(mut stream: TcpStream) {
    let request: Request = parse_request(&stream);
    println!("[{}]: Received Request: {:?}","AUTHOR", request.request_line);

    let response = match request.request_line.path.as_str() {
        "/resource"     => handle_resource(&request),
        "/token"        => handle_token(&request),
        _               => handle_404(&request),
    };

    println!("[{}]: Responding: {:?}","AUTHOR", response.response_line);
    
    stream.write_all(response.as_bytes()).unwrap();
}