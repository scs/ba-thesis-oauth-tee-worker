extern crate sgx_tstd as sgx;
use std::net::TcpListener;
use std::time::Duration;
use std::thread;

/************************************\
 *         Custom Modules           *
\************************************/

mod oauth_authorizer;
mod oauth_authorizer_config;
mod oauth_client;
mod oauth_client_config;

mod credential_checks;
mod html_elements;
mod token_base;
mod parser;
mod types;
mod tools;
mod token;

use crate::oauth::oauth_authorizer_config::*;
use crate::oauth::oauth_client_config::*;

/************************************\
 *              Entry               *
\************************************/

/// This function serves as the entry point for the OAuth Service Demo. 
/// It launches two seperate threads: the Authorization server on 7878 and the Client server on 7879.
pub fn start_oauth_server() {
    let read_timeout_duration = Duration::from_millis(500);
    
    let authorizer_listener = TcpListener::bind(AUTHOR_URL).unwrap();
    let client_listener = TcpListener::bind(CLIENT_URL).unwrap();

    let authorizer_thread = thread::spawn(move || {
        for stream in authorizer_listener.incoming() {
            let stream = stream.unwrap();
            stream.set_read_timeout(Some(read_timeout_duration));
            oauth_authorizer::handle_connection(stream);
        }
    });

    let client_thread = thread::spawn(move || {
        for stream in client_listener.incoming() {
            let stream = stream.unwrap();
            stream.set_read_timeout(Some(read_timeout_duration));
            oauth_client::handle_connection(stream);
        }
    });

    authorizer_thread.join().unwrap();
    client_thread.join().unwrap();
}