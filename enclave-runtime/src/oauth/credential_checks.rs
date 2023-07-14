extern crate sgx_tstd as sgx;
use std::string::{String, ToString};
use bcrypt::{hash, verify, BcryptError};
use super::types::*;
use super::tools::*;


static USERNAME: &str = "user";
static PASSWORD: &str = "asdf";

static CLIENT_ID: &str = "client_id";
static CLIENT_SECRET: &str = "client_secret";

pub fn hash_value(value: &str) -> String {
    match hash(value, bcrypt::DEFAULT_COST) {
        Ok(hashed_value) => hashed_value,
        Err(_) => "Something went wrong while hashing".to_string()
    }
}

fn check_value(value: &str, hashed_value: &str) -> bool {
    match verify(value, hashed_value) {
        Ok(valid) => valid,
        Err(_) => false,
    }
}

pub fn client(client_id: &str, client_secret: &str) -> Result<(), (ErrorCode, String, String)> {
    println!("Client Secret: {}", clear_quotes(&client_secret.to_string()));
    println!("Should be: {}", hash_value(CLIENT_SECRET));
    match CLIENT_ID.to_string() == client_id.to_string()
    && check_value(CLIENT_SECRET, &clear_quotes(&client_secret.to_string())) {
            true => Ok(()),
            false => Err((ErrorCode::InvalidGrant,
                "Client credentials not valid".to_string(),
                "https://datatracker.ietf.org/doc/html/rfc6749#section-2.1".to_string()))
        }
}

pub fn user(username: &str, password: &str) -> Result<(), (ErrorCode, String, String)> {
    println!("Password: {}", clear_quotes(&password.to_string()));
    println!("Should be: {}", hash_value(PASSWORD));
    match USERNAME.to_string() == username.to_string()
        && check_value(PASSWORD, &clear_quotes(&password.to_string())) {
            true => Ok(()),
            false => Err((ErrorCode::InvalidGrant,
                "User credentials not valid".to_string(),
                "https://datatracker.ietf.org/doc/html/rfc6749#section-4.3".to_string()))
        }
}