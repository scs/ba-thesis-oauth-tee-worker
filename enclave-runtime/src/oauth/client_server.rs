extern crate sgx_tstd as std;
use std::vec::Vec;
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
use std::string::{ToString, String};
use std::collections::HashMap;
use serde_json::json;
use serde_json::Value;

use super::client_config::*;
use super::html_elements::*;
use super::types::*;

fn handle_root() -> (String, String) {
    ("HTTP/1.1 200 OK".to_string(), HTML_HELLO.to_string())
}
  
fn handle_service(request: &Request) -> (String, String) {
    // Check if access_token is set
    if let Some(token) = request.cookies.get(&String::from("access_token")) {
        // Todo  Check if token is valid and not void or expired
        if true {
            // If Access token is valid, handle the resource request

            let payload = json!({
                "access_token": json!(token);
            });

            let payload_string = payload.to_string();
            let content_length = payload_string.len();

            let request = format!(
                "POST /resource HTTP/1.1\r\n\
                Host:localhost:7878\r\n\
                Content-Length:{}\r\n\
                Content-Type:application/json\r\n\
                {}\r\n\
                \r\n",
                content_length,
                payload
            );
            
            let mut stream = TcpStream::connect("localhost:7878");
            let mut stream = stream.unwrap();
            stream.write_all(request.as_bytes()).unwrap();
            let mut buf_reader = BufReader::new(&mut stream);
            let mut response = String::new();
            let mut content = false;
            let mut payload_json = serde_json::Value::Null;

            for (index, line_wrap) in buf_reader.lines().enumerate() {
                let line = match line_wrap {
                    Ok(line) => {
                        if line.starts_with("Content-Length:") {
                            content = true;
                            continue;
                        }
                        if content {
                            let payload = line.clone();
                            if payload.is_empty() != true {
                                let parsed_body: Result<Value, _> = serde_json::from_str(&payload);
                                
                                if let Ok(value) = parsed_body {
                                    payload_json = value;
                                    break;
                                } else {
                                    // Handle parsing error
                                    if let Err(err) = parsed_body {
                                        println!("Failed to parse JSON: {}", err);
                                        break;
                                    }
                                }
                            }
                        }
                    },
                    Err(e) => {
                        // Check for timeout error and break the loop
                        println!("Encountered an error {}", e);
                    }
                };
            }

            let resource_body = get_request_field(&payload_json, "resource_body"),
            
            let html_content = format!(
                "{}\r\n{}\r\n\r\n",
                "HTTP/1.1 200 OK",
                resource_body
            );

            return (html_content, "".to_string());

        } else {
            // Redirect to `/service`
            let redirect_response = ("HTTP/1.1 302 Found\nLocation: /service".to_string(), "Something went wrong...".to_string());
            return redirect_response;
        }
            // Proceed to request content from resource server

            // return resource to display
            return ("HTTP/1.1 200 OK".to_string(), HTML_RESOURCE_CONTENT.to_string());
        }
    }
    // Access token is not set or not valid, prompt for grant to access resource
    //let cookie_header = "Set-Cookie: access_token=test_value; Path=/";

    ("HTTP/1.1 200 OK".to_string(), HTML_AUTHORIZATION_PROMPT.to_string())
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
            // We're making immutable objects for a minimum of safety
            let access_token_request = AccessTokenRequest {
                request: Request{
                    method: "POST".to_string(), 
                    path: "localhost:7878/token".to_string(), 
                    args: "".to_string(), 
                    cookies: HashMap::new(),
                    body: serde_json::Value::Null,
                },
                grant_type: "user_credentials".to_string(),
                client_id: CLIENT_ID.to_string(),
                // TODO maybe add client_secret
                username: username.to_string(),
                password: password.to_string(),
                redirect_uri: "/service".to_string(),
            };
            // Send AccessTokenRequest to Authorization Server
            let payload = json!({
                "grant_type": json!(access_token_request.grant_type),
                "client_id": json!(access_token_request.client_id),
                "username": json!(access_token_request.username),
                "password": json!(access_token_request.password),
                "redirect_uri": json!(access_token_request.redirect_uri),
            });

            let payload_string = payload.to_string();
            let content_length = payload_string.len();

            let request = format!(
                "POST /token HTTP/1.1\r\n\
                Host:localhost:7878\r\n\
                Content-Length:{}\r\n\
                Content-Type:application/json\r\n\
                {}\r\n\
                \r\n",
                content_length,
                payload
            );
            
            let mut stream = TcpStream::connect("localhost:7878");
            let mut stream = stream.unwrap();
            stream.write_all(request.as_bytes()).unwrap();
            let mut buf_reader = BufReader::new(&mut stream);
            let mut response = String::new();
            let mut content = false;
            let mut payload_json = serde_json::Value::Null;

            for (index, line_wrap) in buf_reader.lines().enumerate() {
                let line = match line_wrap {
                    Ok(line) => {
                        if line.starts_with("Content-Length:") {
                            content = true;
                            continue;
                        }
                        if content {
                            let payload = line.clone();
                            if payload.is_empty() != true {
                                let parsed_body: Result<Value, _> = serde_json::from_str(&payload);
                                
                                if let Ok(value) = parsed_body {
                                    payload_json = value;
                                    break;
                                } else {
                                    // Handle parsing error
                                    if let Err(err) = parsed_body {
                                        println!("Failed to parse JSON: {}", err);
                                        break;
                                    }
                                }
                            }
                        }
                    },
                    Err(e) => {
                        // Check for timeout error and break the loop
                        println!("Encountered an error {}", e);
                    }
                };
            }

            let access_token_response = AccessTokenResponse {
                access_token: get_request_field(&payload_json, "access_token"),
                token_type: get_request_field(&payload_json, "token_type"),
                expires_in: get_request_field(&payload_json, "expires_in").parse().unwrap_or(0)
            };

            let cookie_setter = format!(
                "Set-Cookie: access_token={}; Path=/; Max-Age={}",
                &access_token_response.access_token,
                &access_token_response.expires_in
            );

            let cookie = format!(
                "Cookie: access_token={}",
                &access_token_response.access_token
            );
            
            let html_content = format!(
                "{}\r\n{}\r\nLocation: {}\r\n{}\r\n\r\n",
                "HTTP/1.1 302 Found",
                cookie_setter,
                access_token_request.redirect_uri,
                cookie
            );

            return (html_content, "".to_string());

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
    let request: Request = parse_request(&stream);
    println!("[{}]: Received Request: Action {} on {}","CLIENT", request.method, request.path);
    
    let (status_line, contents) = match request.path.as_str() {
        "/" => handle_root(),
        "/service" => handle_service(&request), // sollicitation of protected resource
        "/authorize" => handle_authorize(&request), // granting permission to request access to protected resource
        _ => ("HTTP/1.1 404 NOT FOUND".to_string(), HTML_404.to_string()),
    };

    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length:{length}\r\n{contents}\r\n\r\n");
    
    println!("[{}]: Responding: {}","CLIENT", status_line);

    stream.write_all(response.as_bytes()).unwrap();
}
  