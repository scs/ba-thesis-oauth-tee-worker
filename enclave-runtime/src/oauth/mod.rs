extern crate sgx_tstd as sgx;
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::string::{ToString, String};
use std::vec::Vec;
use std::fs;

use crate::std::io::BufRead;
use crate::std::io::Write;

static HTML_HELLO: &str ="
<!DOCTYPE html>
<html lang='en'>
  <head>
    <meta charset='utf-8'>
    <title>Hello!</title>
  </head>
  <body>
    <h1>Hello!</h1>
    <p>Hi from Rust</p>
  </body>
</html>
";

static HTML_OK_TEXT: &str = "
<html>
    Here should be the protected ressource;
</html>
";

static HTML_DENY_TEXT: &str = "
<html>
    This page should be accessed via an oauth token from the client in the example. Click
    <a href=\"http://localhost:8020/authorize?response_type=code&client_id=LocalClient\">
    here</a> to begin the authorization process.
</html>
";

static HTML_404: &str ="
<!DOCTYPE html>
<html lang='en'>
  <head>
    <meta charset='utf-8'>
    <title>Hello!</title>
  </head>
  <body>
    <h1>Oops!</h1>
    <p>Sorry, I don't know what you're asking for.</p>
  </body>
</html>
";

fn requesting_protected_ressource() -> (&'static str, String) {
    ("HTTP/1.1 200 OK", HTML_DENY_TEXT.to_string())
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, contents) = match request_line.as_str(){
        "GET / HTTP/1.1"                => ("HTTP/1.1 200 OK", HTML_HELLO.to_string()),
        "GET /ressource HTTP/1.1"       => requesting_protected_ressource(),
        _                   => ("HTTP/1.1 404 NOT FOUND", HTML_404.to_string()),
    };

    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}


pub fn start_oauth_server() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}