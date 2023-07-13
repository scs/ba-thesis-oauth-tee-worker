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
pub struct TokenBase {
    base: HashMap<String, SystemTime>,
}

impl TokenBase {
    fn new() -> TokenBase {
        TokenBase {
            base: HashMap::new(),
        }
    }

    fn get_expiry(&mut self, token: &String) -> Option<SystemTime> {
        self.base.get(&token.as_str()).cloned()
    }

    fn insert_token(&mut self, token: &String, expiry: SystemTime) {
        println!("Base: {:?}", self.base);
        self.base.insert(&token, expiry);
    }

    fn is_token_valid(&mut self, token: &String) -> bool {
        println!("Base: {:?}", self.base);
        match self.base.get(&token.as_str()) {    
            Some(expiry) => {
                if expiry > &SystemTime::now() {
                    println!("\t--->[{}] Token {} is valid", "TOKEN_BASE", token);
                    true
                } else {
                    println!("\t--->[{}] Token {} is expired", "TOKEN_BASE", token);
                    false
                }
            }
            None => {
                println!("\t--->[{}] Token {} isn't in base", "TOKEN_BASE", token);
                false
            }
        }
    }
}

pub fn generate_token() -> String {
    let mut token = generate_random_token();
    let expiry = SystemTime::now() + Duration::from_secs(30);

    while get_token_base().is_token_valid(&token) {
        token = generate_random_token();
    }

    get_token_base().insert_token(&token, expiry);
    token
}

pub fn get_token_expiry(token: &String) -> Option<SystemTime> {
    get_token_base().get_expiry(token)
}

pub fn get_token_validity(token: &String) -> bool {
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
