use std::io::{BufReader, Read};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::string::{ToString, String};
use std::vec::Vec;

use crate::std::io::BufRead;
use crate::std::io::Write;

pub fn start_oauth_server() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        if let Ok(mut stream) = stream {
            let mut reader = BufReader::new(&stream);
            let mut request_line = String::new();
            reader.read_line(&mut request_line).unwrap();
            println!("Request line: {:?}", request_line);

            let response = "HTTP/1.1 200 OK\r\n\r\n";
            stream.write_all(response.as_bytes()).unwrap();
        }
    }
}