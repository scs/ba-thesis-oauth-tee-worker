extern crate sgx_tstd as std;
use std::collections::HashMap;
use std::string::{String, ToString};

use super::types::*;
use super::html_elements::*;

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

pub fn create_resource_response() -> Response {
    let response_line = ResponseLine {
        http_version: "HTTP/1.1".to_string(),
        status_code: 200,
        response_type: HttpResponseType::Success,
    };

    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), "text/html".to_string());

    let body = serde_json::json!({
        "html_content": HTML_RESOURCE_CONTENT.to_string()
    });

    Response {
        response_line,
        headers,
        body,
    }
}

pub fn create_access_denied_response() -> Response {
    let response_line = ResponseLine {
        http_version: "HTTP/1.1".to_string(),
        status_code: 403,
        response_type: HttpResponseType::ClientError,
    };

    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), "text/html".to_string());

    let body = serde_json::json!({
        "html_content": HTML_ACCESS_DENIED.to_string()
    });

    Response {
        response_line,
        headers,
        body,
    }
}

pub fn create_error_response(error: ErrorCode, error_description: String, error_uri: String) -> ErrorResponse {
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