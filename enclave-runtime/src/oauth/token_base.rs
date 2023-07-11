extern crate sgx_tstd as std;
use std::collections::HashMap;
use std::time::{SystemTime, Duration};
use std::string::String;
use std::sync::SgxMutex;
use sgx_rand::Rng;
use sgx_rand::thread_rng;
use lazy_static::lazy_static;

/// Token base singleton
lazy_static! {
    static ref TOKEN_BASE: SgxMutex<TokenBase> = SgxMutex::new(TokenBase::new());
}

fn get_token_base() -> std::sync::SgxMutexGuard<'static, TokenBase> {
    TOKEN_BASE.lock().unwrap()
}

pub struct TokenBase {
    base: HashMap<String, SystemTime>,
}

impl TokenBase {
    fn new() -> TokenBase {
        TokenBase {
            base: HashMap::new(),
        }
    }

    fn get_expiry(&mut self, token: &str) -> Option<SystemTime> {
        self.base.get(token).cloned()
    }

    fn insert_token(&mut self, token: String, expiry: SystemTime) {
        self.base.insert(token, expiry);
    }

    fn is_token_valid(&mut self, token: &str) -> bool {
        match self.base.get(token) {
            Some(expiry) => expiry > &SystemTime::now(),
            None => false,
        }
    }
}

pub fn generate_token() -> String {
    let mut rng = thread_rng();
    let mut token = generate_random_token(&mut rng, 32);
    let expiry = SystemTime::now() + Duration::from_secs(30);

    while get_token_base().is_token_valid(&token) {
        token = generate_random_token(&mut rng, 32);
    }

    get_token_base().insert_token(token.clone(), expiry);
    token
}

pub fn get_token_expiry(token: &str) -> Option<SystemTime> {
    get_token_base().get_expiry(token)
}

pub fn get_token_validity(token: &str) -> bool {
    get_token_base().is_token_valid(token)
}

fn generate_random_token<R: Rng>(rng: &mut R, length: usize) -> String {
    let mut token = String::new();
    for _ in 0..length {
        token.push(rng.gen::<char>());
    }
    token
}
