
use sgx_tstd as std;
use std::thread;
use std::str::from_utf8;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream, Shutdown};

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 50]; // using 50 byte buffer
    while match stream.read(&mut data) {
        Ok(size) => {
            // echo everything!
            stream.write(&data[0..size]).unwrap();
            true
        },
        Err(_) => {
            println!("[SERVER]: An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

pub fn start_oauth_server() {
    println!("Hello World from inside the Enclave!");

    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("[SERVER]: Server listening on port 7878");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("[SERVER]: New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move|| {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("[SERVER]: Error: {}", e);
                /* connection failed */
            }
        }
    }
}