extern crate sgx_tstd as std;
use std::io::Write;
use std::net::TcpStream;
use std::string::ToString;
use std::collections::HashMap;

use super::types::*;
use super::parser::*;
use super::tools::*;
use super::oauth_authorizer_config::*;
use super::oauth_client_config::*;
use super::html_elements::*;
use super::credential_checks::*;


/// The client must respond to two routes:
/// /authorize      => authorizing an access token request
/// /service        => access to the service that needs a resource
pub fn handle_connection(mut stream: TcpStream) {
    let request: Request = parse_request(&stream);
    println!("[CLIENT]: Received:\t\t {:?}", request.request_line);

    let result = match request.request_line.path.as_str() {
        "/authorize" => handle_authorize(&request),
        "/service" => handle_service(&request),
        _ => Ok(handle_404(&request)),
    };

    match result {
        Ok(response) => {
            println!("[CLIENT]: Responding:\t {:?}", response.response_line);
            stream.write_all(response.to_string().as_bytes()).unwrap();
        }
        Err(error_response) => {
            println!("[CLIENT]: Error:\t\t {:?}", error_response);
            let response = response_with_error_content_from_error(&error_response);
            stream.write_all(response.to_string().as_bytes()).unwrap();
        }
    }
}

fn handle_service(request: &Request) -> Result<Response, ErrorResponse> {
    match request.headers.get("Cookie") {
        Some(cookie_header) => {
            let cookie = parse_cookie_header(cookie_header);
            match cookie.get("access_token") {
                Some(token) => {
                    // A token is present so we can request the resource
                    Ok(request_resource(token.as_str()))
                }
                None => {
                    Ok(redirect_authorize())
                }
            }
        }
        None => {
            Ok(redirect_authorize())
        }, 
    }
}

fn handle_authorize(request: &Request) -> Result<Response, ErrorResponse>{
    match request.request_line.method {
        HttpMethod::Get => {
            let response_line = ResponseLine {
                http_version: "HTTP/1.1".to_string(),
                status_code: 200,
                response_type: HttpResponseType::Success,
            };
            
            let mut headers = HashMap::new();
            headers.insert("Content-Type".to_string(), "text/html".to_string());

            let html_content = match request.body.as_str() {
                Some(content) => {content}
                None => {""}
            };

            let body = serde_json::json!({
                "html_content": html_authorization_prompt(html_content),
            });

            Ok(Response {
                response_line,
                headers,
                body
            })
        }
        HttpMethod::Post => {
            let request_line = RequestLine {
                method: HttpMethod::Get,
                path: "/token".to_string(),
                http_version: "HTTP/1.1".to_string(),
            };
        
            let mut headers = HashMap::new();
            headers.insert("Content-Type".to_string(), "application/json".to_string());
            
            let username = request
                    .body
                    .get("username")
                    .ok_or((ErrorCode::InvalidGrant, "Missing username header", "https://datatracker.ietf.org/doc/html/rfc6749#section-4.3"))
                    .unwrap().as_str().unwrap();

            let password = request
                    .body
                    .get("password")
                    .ok_or((ErrorCode::InvalidGrant, "Missing password header", "https://datatracker.ietf.org/doc/html/rfc6749#section-4.3"))
                    .unwrap().as_str().unwrap();

            let request_body = serde_json::json!({
                "grant_type": GrantType::ResourceOwnerPasswordCredentials.to_string(),
                "client_id": CLIENT_ID,
                "client_secret": hash_value(CLIENT_SECRET),
                "username": username,
                "password": hash_value(password),
            });
            
            let access_token_request = Request {
                request_line,
                headers,
                body: request_body
            };

            let response = send(&access_token_request, AUTHOR_URL);

            let values = match response.body.get("access_token") { 
                Some(access_token) => {
                    serde_json::json!({
                        "access_token": access_token,
                        "token_type": response.body["token_type"],
                        "expires_in_s": response.body["expires_in_s"]
                    })
                }
                None => {
                    return Err(parse_error_response(&response));
                }
            };
            
            let response_line = ResponseLine {
                http_version: "HTTP/1.1".to_string(),
                status_code: 302,
                response_type: HttpResponseType::Redirection,
            };
            
            let mut headers = HashMap::new();
            headers.insert("Location".to_string(), "/service".to_string());
            let token = values["access_token"].as_str().unwrap();

            headers.insert("Cookie".to_string(), format!("access_token={}",token));

            let access_token_setter = format!(
                "access_token={}; Path=/; Max-Age=3600",
                token
            );

            headers.insert("Set-Cookie".to_string(), access_token_setter);

            let body = serde_json::json!({});

            Ok(Response {
                response_line,
                headers,
                body
            })
        }
        _ => Ok(handle_404(request))
    }
}

fn request_resource(token: &str) -> Response {
    let request_line = RequestLine {
        method: HttpMethod::Get,
        path: "/resource".to_string(),
        http_version: "HTTP/1.1".to_string(),
    };

    let mut headers = HashMap::new();
    headers.insert("Cookie".to_string(), format!("access_token={}", token));

    let body = serde_json::json!({
        "access_token": token
    });

    let request = Request {
        request_line,
        headers,
        body,
    };

    let response = send(&request, AUTHOR_URL);
    match response.body.get("resource_content") { 
        Some(resource_content) => {
            // This means we have a resource and the request was successfull
            response_with_resource_content(&resource_content.to_string(), token)
        }
        None => {
            // An error occured somewhere
            response_with_error_content(&response)
        } 
    }
}

fn redirect_authorize() -> Response {
    let response_line = ResponseLine {
        http_version: "HTTP/1.1".to_string(),
        status_code: 302,
        response_type: HttpResponseType::Redirection,
    };
    
    let mut headers = HashMap::new();
    headers.insert("Location".to_string(), "/authorize".to_string());

    let body = serde_json::json!({});

    Response {
        response_line,
        headers,
        body
    }
}