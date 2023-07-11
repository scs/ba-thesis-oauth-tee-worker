extern crate sgx_tstd as sgx;
use std::net::TcpListener;
use std::time::Duration;
use std::thread;

/************************************\
 *         Custom Modules           *
\************************************/

mod oauth_authorizer;
mod oauth_client;
mod oauth_client_config;

mod credential_checks;
mod html_elements;
mod token_base;
mod parser;
mod types;
mod tools;
mod token;

/************************************\
 *              Entry               *
\************************************/

/// This function serves as the entry point for the OAuth Service. 
/// It launches two seperate threads: the Authorization server on 7878 and the Client server on 7879.
pub fn start_oauth_server() {
    let read_timeout_duration = Duration::from_millis(10);
    
    let authorizer_listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    //let client_listener = TcpListener::bind("127.0.0.1:7879").unwrap();

    let authorizer_thread = thread::spawn(move || {
        for stream in authorizer_listener.incoming() {
            let stream = stream.unwrap();
            stream.set_read_timeout(Some(read_timeout_duration));
            oauth_authorizer::handle_connection(stream);
        }
    });

    // let client_thread = thread::spawn(move || {
    //     for stream in client_listener.incoming() {
    //         let stream = stream.unwrap();
    //         oauth_client::handle_connection(stream);
    //     }
    // });

    authorizer_thread.join().unwrap();
    //client_thread.join().unwrap();
}