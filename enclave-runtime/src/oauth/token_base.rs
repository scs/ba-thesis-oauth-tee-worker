extern crate sgx_tstd as std;
use std::collections::HashMap;
use std::time::{SystemTime, Duration};
use std::string::{String, ToString};
use std::sync::SgxMutex;
use std::borrow::ToOwned;
use sgx_rand::Rng;
use lazy_static::lazy_static;

// Token base singleton
lazy_static! {
    static ref TOKEN_BASE: SgxMutex<TokenBase> = SgxMutex::new(TokenBase::new());
}

fn get_token_base() -> std::sync::SgxMutexGuard<'static, TokenBase> {
    TOKEN_BASE.lock().unwrap()
}

#[derive(Debug)]
struct TokenBase {
    tokens: HashMap<String, SystemTime>,
}

impl TokenBase {
    fn new() -> TokenBase {
        TokenBase {
            tokens: HashMap::new(),
        }
    }

    fn insert_token(&mut self, token: &String, expiry: SystemTime) {
        self.tokens.insert(token.to_owned(), expiry);
    }

    fn get_expiry(&self, token: &str) -> Option<SystemTime> {
        self.tokens.get(&token.to_string()).copied()
    }

    fn is_token_valid(&self, token: &str) -> bool {
        match self.get_expiry(token) {
            Some(expiry) => expiry > SystemTime::now(),
            None => false,
        }
    }
}

pub fn generate_token() -> String {
    let mut token = generate_random_token();
    let expiry = SystemTime::now() + Duration::from_secs(30);

    while get_token_base().is_token_valid(token.as_str()) {
        token = generate_random_token();
    }

    get_token_base().insert_token(&token, expiry);
    token
}

pub fn get_token_expiry(token: &str) -> Option<SystemTime> {
    get_token_base().get_expiry(token)
}

pub fn get_token_validity(token: &str) -> bool {
    get_token_base().is_token_valid(token)
}

fn generate_random_token() -> String {
    // For now this will do ...
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
    
    const TOKEN_LEN: usize = 30;

    let token: String = (0..TOKEN_LEN)
        .map(|_| {
            // Note: this is still the old version of gen_range.
            // In newer versions it's pub fn gen_range<T, R>(&mut self, range: R) -> T
            let idx = sgx_rand::thread_rng().gen_range(0, CHARSET.len()); 
            CHARSET[idx] as char
        })
        .collect();
    token
}
