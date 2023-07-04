extern crate sgx_tstd as sgx;
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::string::{ToString, String};
use std::vec::Vec;
use std::fs;
use std::format;
use std::time::Duration;

use crate::std::io::BufRead;
use crate::std::io::Write;

mod types;
mod client_config;
mod client_server;
mod authorization_server;

mod html_elements;
use html_elements::*;
use types::{Route, Request, parse_request};

pub fn start_oauth_server() {
    let authorization_listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let client_listener = TcpListener::bind("127.0.0.1:7879").unwrap();

    let authorization_thread = thread::spawn(move || {
        for stream in authorization_listener.incoming() {
            let stream = stream.unwrap();
            // Dirty hack to make the parser timeout 
            //stream.set_read_timeout(Some(Duration::from_millis(50)));
            authorization_server::handle_connection(stream);
        }
    });

    let client_thread = thread::spawn(move || {
        for stream in client_listener.incoming() {
            let stream = stream.unwrap();
            // Dirty hack to make the parser timeout 
            //stream.set_read_timeout(Some(Duration::from_millis(50)));
            client_server::handle_connection(stream);
        }
    });

    // Wait for both threads to finish
    authorization_thread.join().unwrap();
    client_thread.join().unwrap();
}