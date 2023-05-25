
use sgx_tstd as std;
use std::io::Read;
use std::io::Write;
use std::vec::{Vec};
use std::net::{TcpListener, TcpStream};
use std::str;

pub fn start_oauth_server() {
    println!("Hello World from inside the Enclave!");

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    
    for stream in listener.incoming() {
        let mut buffer = Vec::new();
        let mut stream = stream.unwrap();
        stream.read_to_end(&mut buffer);
        let message = str::from_utf8(&buffer);
        println!("Message received: {}", message.unwrap());
    }
}