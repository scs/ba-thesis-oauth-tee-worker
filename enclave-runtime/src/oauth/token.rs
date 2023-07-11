extern crate sgx_tstd as std;
use std::collections::HashMap;
use std::string::{String, ToString};

use super::types::*;
use super::token_base::*;
use super::credential_checks::*;


pub fn validate_access_token_request(access_token_request: &AccessTokenRequest) -> Result<(), (ErrorCode, String, String)> {
    // validation checks...
    // Todo client/user cred validation seperate!
    
    Ok(())
}

pub fn validate_token(token: &String) -> bool {
    println!("---> Token submitted for validation: {}", token);
    return get_token_validity(&token.as_str());
}

pub fn create_access_token_response() -> Response {
    let response_line = ResponseLine {
        http_version: "HTTP/1.1".to_string(),
        status_code: 404,
        response_type: HttpResponseType::ClientError,
    };

    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), "application/json".to_string());

    let token = generate_token();

    let body = serde_json::json!({
        "access_token": token,
        "token_type": TokenType::Bearer.to_string(),
        "expires": get_token_expiry(&token.as_str()).unwrap(),
    });

    Response {
        response_line,
        headers,
        body
    }
}
