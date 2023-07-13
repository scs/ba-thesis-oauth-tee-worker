extern crate sgx_tstd as std;
use std::collections::HashMap;
use std::string::{String, ToString};
use std::time::SystemTime;

use super::types::*;
use super::token_base::*;
use super::credential_checks::*;


pub fn validate_access_token_request(access_token_request: &AccessTokenRequest) -> Result<(), (ErrorCode, String, String)> {
    let fields = [
        ("client_id", &access_token_request.client_id),
        ("client_secret", &access_token_request.client_secret),
        ("username", &access_token_request.username),
        ("password", &access_token_request.password),
    ];

    for (field_name, field_value) in &fields {
        if field_value.is_empty() {
            let error_description = format!("{} is empty", field_name);
            let error_uri = "https://datatracker.ietf.org/doc/html/rfc6749#section-4.3.2".to_string();
            return Err((ErrorCode::InvalidRequest, error_description, error_uri));
        }
    }

    match client(&access_token_request.client_id.as_str(), &access_token_request.client_secret.as_str()) {
        Ok(()) => {
            user(&access_token_request.username.as_str(), &access_token_request.password.as_str())
        }
        Err((error, error_description, error_uri)) => {
            Err((error, error_description, error_uri))
        }
    }
}

pub fn validate_token(token: &str) -> bool {
    return get_token_validity(&token.to_string());
}

pub fn expiry_response(token: &String) -> Response {
    let response_line = ResponseLine {
        http_version: "HTTP/1.1".to_string(),
        status_code: 200,
        response_type: HttpResponseType::Success,
    };

    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), "application/json".to_string());

    let expiry = match get_token_expiry(&token) {
        Some(expiry_time) => {
            expiry_time.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs().to_string()
        }
        None => {
            "Something went wrong while looking up the token's expiry".to_string()
        }
    };   

    let body = serde_json::json!({
        "expiry_time": expiry,
    });

    Response {
        response_line,
        headers,
        body,
    }
}

pub fn access_token_response() -> Response {
    let response_line = ResponseLine {
        http_version: "HTTP/1.1".to_string(),
        status_code: 404,
        response_type: HttpResponseType::ClientError,
    };

    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), "application/json".to_string());

    let token = generate_token();

    let expiry = get_token_expiry(&token).unwrap();

    let expires_in_s = expiry.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() 
                    - SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();

    let body = serde_json::json!({
        "access_token": token,
        "token_type": TokenType::Bearer.to_string(),
        "expires_in_s": expires_in_s.to_string(),
    });

    Response {
        response_line,
        headers,
        body
    }
}
