extern crate sgx_tstd as sgx;
use std::string::{String, ToString};
use bcrypt::{hash_with_salt};
use bcrypt::Version::TwoB;
use super::types::*;


static USERNAME: &str = r#"user"#;
static PASSWORD: &str = r#"asdf"#;

static CLIENT_ID: &str = r#"client_id"#;
static CLIENT_SECRET: &str = r#"client_secret"#;

// Some salt... 
// (It says: "helloworlditsme!" - you're welcome)
static SALT: [u8; 16] = [0x68, 0x65, 0x6C, 0x6C, 0x6F, 0x77, 0x6F, 0x72, 0x6C, 0x64, 0x69, 0x74, 0x73, 0x6D, 0x65, 0x21];

pub fn hash_value(value: &str) -> String {
    match hash_with_salt(value, bcrypt::DEFAULT_COST, SALT) {
        Ok(hashed_value) => hashed_value.format_for_version(TwoB),
        Err(_) => "Something went wrong while hashing".to_string()
    }
}

pub fn verify_client(client_id: &str, client_secret: &str) -> Result<(), (ErrorCode, String, String)> {
    match client_id == CLIENT_ID && client_secret == hash_value(CLIENT_SECRET) {
            true => Ok(()),
            false => Err((ErrorCode::InvalidGrant,
                "Client credentials not valid".to_string(),
                "https://datatracker.ietf.org/doc/html/rfc6749#section-2.1".to_string()))
        }
}

pub fn verify_user(username: &str, password: &str) -> Result<(), (ErrorCode, String, String)> {
    match username == USERNAME && password == hash_value(PASSWORD) {
            true => Ok(()),
            false => Err((ErrorCode::InvalidGrant,
                "User credentials not valid".to_string(),
                "https://datatracker.ietf.org/doc/html/rfc6749#section-4.3".to_string()))
        }
}