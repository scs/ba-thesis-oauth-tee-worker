extern crate sgx_tstd as std;
use std::collections::HashMap;
use std::string::{String, ToString};
use std::net::TcpStream;
use std::io::Write;
use std::borrow::ToOwned;

use super::types::*;
use super::html_elements::*;
use super::parser::*;
use super::oauth_authorizer_config::*;

/************************************\
 *             Generics             *
\************************************/

pub fn handle_404(request: &Request) -> Response {
    let response_line = ResponseLine {
        http_version: "HTTP/1.1".to_string(),
        status_code: 404,
        response_type: HttpResponseType::ClientError,
    };

    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), "text/html".to_string());

    let body = serde_json::json!({
        "html_content": HTML_404.to_string()
    });

    Response {
        response_line,
        headers,
        body,
    }
}

pub fn error_response(error: ErrorCode, error_description: String, error_uri: String) -> ErrorResponse {
    let response_line = ResponseLine {
        http_version: "HTTP/1.1".to_string(),
        status_code: 400,
        response_type: HttpResponseType::ClientError,
    };

    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), "application/json".to_string());

    let response = Response {
        response_line,
        headers,
        body: serde_json::Value::Null,
    };

    ErrorResponse {
        response,
        error,
        error_description,
        error_uri,
    }
}

pub fn send(request: &Request, adress: &str) -> Response {
    let mut stream = TcpStream::connect(adress);
    let mut stream = stream.unwrap();
    stream.write_all(request.to_string().as_bytes()).unwrap();
    parse_response(&stream)
}

pub fn clear_quotes(value: &String) -> String {
    let mut value = value.to_owned();
    if value.starts_with('"') && value.ends_with('"') {
        value = value[1..value.len()-1].to_owned();
    }
    value
}

/************************************\
 *           Authorizer             *
\************************************/

pub fn resource_response() -> Response {
    let response_line = ResponseLine {
        http_version: "HTTP/1.1".to_string(),
        status_code: 200,
        response_type: HttpResponseType::Success,
    };

    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), "application/json".to_string());

    let body = serde_json::json!({
        "resource_content": HTML_RESOURCE.to_string()
    });

    Response {
        response_line,
        headers,
        body,
    }
}

pub fn access_denied_response() -> ErrorResponse {
    let response_line = ResponseLine {
        http_version: "HTTP/1.1".to_string(),
        status_code: 403,
        response_type: HttpResponseType::ClientError,
    };

    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), "application/json".to_string());

    let body = serde_json::json!({});

    let response = Response {
        response_line,
        headers,
        body,
    };

    ErrorResponse {
        response,
        error: ErrorCode::InvalidRequest,
        error_description: "No token was provided".to_string(),
        error_uri: "https://datatracker.ietf.org/doc/html/rfc6749#section-7".to_string()
    }
}

pub fn invalid_token_response() -> ErrorResponse {
    let response_line = ResponseLine {
        http_version: "HTTP/1.1".to_string(),
        status_code: 403,
        response_type: HttpResponseType::ClientError,
    };

    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), "application/json".to_string());

    let body = serde_json::json!({});

    let response = Response {
        response_line,
        headers,
        body,
    };

    ErrorResponse {
        response,
        error: ErrorCode::InvalidRequest,
        error_description: "The token provided was invalid".to_string(),
        error_uri: "https://datatracker.ietf.org/doc/html/rfc6749#section-7".to_string()
    }
}

/************************************\
 *             Client               *
\************************************/

pub fn response_with_resource_content(resource_content: &String, token: &String) -> Response {
    let response_line = ResponseLine {
        http_version: "HTTP/1.1".to_string(),
        status_code: 200,
        response_type: HttpResponseType::Success,
    };

    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), "text/html".to_string());

    let expiry = request_expiry(&token.as_str());
    
    let body = serde_json::json!({
        "html_content": format!("{}\n{}\n{}", 
                                HTML_RESOURCE_HEADER, 
                                html_resource_table(&resource_content.as_str(), &token.as_str(), &expiry),
                                HTML_RESOURCE_FOOTER)
    });

    Response {
        response_line,
        headers,
        body
    }
}

pub fn response_with_error_content(response: &Response) -> Response {
    let response_line = ResponseLine {
        http_version: "HTTP/1.1".to_string(),
        status_code: 200,
        response_type: HttpResponseType::Success,
    };
    
    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), "text/html".to_string());

    let html_content = format!("<br>
                                <p>Error Code: {}<br>
                                Description: {}<br>
                                Helpful <strong><a href={}>link</a></strong>.<br></p>",
                                response.body.get("error").unwrap(),
                                response.body.get("error_description").unwrap(),
                                response.body.get("error_uri").unwrap()
                                );
    
    let body = serde_json::json!({
        "html_content": html_authorization_prompt(&html_content.as_str()),
    });

    Response {
        response_line,
        headers,
        body
    }
}

pub fn response_with_error_content_from_error(error_response: &ErrorResponse) -> Response {
    let response_line = ResponseLine {
        http_version: "HTTP/1.1".to_string(),
        status_code: 200,
        response_type: HttpResponseType::Success,
    };
    
    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), "text/html".to_string());

    let html_content = format!("<br>
                                <p>Error Code: {}<br>
                                Description: {}<br>
                                Helpful <strong><a href={}>link</a></strong>.<br></p>",
                                error_response.error.to_string(),
                                error_response.error_description,
                                error_response.error_uri
                                );
    
    let body = serde_json::json!({
        "html_content": html_authorization_prompt(&html_content.as_str()),
    });

    Response {
        response_line,
        headers,
        body
    }
}

pub fn request_expiry(token: &str) -> String {
    let request_line = RequestLine {
        method: HttpMethod::Get,
        path: "/expiry".to_string(),
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

    let response = send(&request, &AUTHOR_URL);
    match response.body.get("expires_in_s") { 
        Some(expires_in_s) => {
            expires_in_s.to_string()
        }
        None => {
            "Something went wrong while getting the expiry time".to_string()
        }
    }
}

