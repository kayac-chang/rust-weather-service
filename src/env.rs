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

pub fn get_port() -> u16 {
    env::var("PORT").unwrap_or("3000".into()).parse().unwrap()
}
