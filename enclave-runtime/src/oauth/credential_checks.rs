extern crate sgx_tstd as sgx;
use std::string::{String, ToString};
use std::borrow::ToOwned;
use bcrypt::{hash_with_salt, verify, BcryptError};
use bcrypt::Version::TwoB;
use super::types::*;
use super::tools::*;


static USERNAME: &str = "user";
static PASSWORD: &str = "asdf";

static CLIENT_ID: &str = "client_id";
static CLIENT_SECRET: &str = "client_secret";

// Some salt... 
// (It says: "helloworlditsme!" - you're welcome)
static SALT: [u8; 16] = [0x68, 0x65, 0x6C, 0x6C, 0x6F, 0x77, 0x6F, 0x72, 0x6C, 0x64, 0x69, 0x74, 0x73, 0x6D, 0x65, 0x21];

pub fn hash_value(value: &str) -> String {
    match hash_with_salt(value, bcrypt::DEFAULT_COST, SALT) {
        Ok(hashed_value) => hashed_value.format_for_version(TwoB),
        Err(_) => "Something went wrong while hashing".to_string()
    }
}

fn check_value(given_value: &str, known_value: &str) -> bool {
    given_value == known_value
}

pub fn client(client_id: &str, client_secret: &str) -> Result<(), (ErrorCode, String, String)> {
    match check_value(&clear_quotes(&client_id.to_string()), &clear_quotes(&CLIENT_ID.to_string()))
    && check_value(&clear_quotes(&client_secret.to_string()), &hash_value(&clear_quotes(&CLIENT_SECRET.to_string()))) {
            true => Ok(()),
            false => Err((ErrorCode::InvalidGrant,
                "Client credentials not valid".to_string(),
                "https://datatracker.ietf.org/doc/html/rfc6749#section-2.1".to_string()))
        }
}

pub fn user(username: &str, password: &str) -> Result<(), (ErrorCode, String, String)> {
    match check_value(&clear_quotes(&username.to_string()), &clear_quotes(&USERNAME.to_string()))
        && check_value(&clear_quotes(&password.to_string()), &hash_value(&clear_quotes(&PASSWORD.to_string()))) {
            true => Ok(()),
            false => Err((ErrorCode::InvalidGrant,
                "User credentials not valid".to_string(),
                "https://datatracker.ietf.org/doc/html/rfc6749#section-4.3".to_string()))
        }
}