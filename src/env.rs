use dotenv::dotenv;
use std::env;

pub fn init() {
    dotenv().ok();
}

pub fn get_token() -> String {
    env::var("TOKEN").unwrap()
}

pub fn get_cwb_api() -> String {
    env::var("CWB_API").unwrap()
}
