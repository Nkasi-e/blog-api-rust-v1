use std::env;
use lazy_static::lazy_static;


lazy_static!{
    pub static ref ADDRESS: String = set_address();
    pub static ref PORT: u16 = set_port();
    pub static ref DATABASE_URL: String = set_database_url();
    pub static ref SECRET: String = set_secret();
}

fn set_address() -> String {
    dotenv::dotenv().ok();
    env::var("ADDRESS").unwrap_or("127.0.0.1".to_string())
}

fn set_port() -> u16 {
    dotenv::dotenv().ok();
    env::var("PORT").unwrap_or("3000".to_owned()).parse::<u16>().expect("Can't parse port ")
}

fn set_database_url() -> String {
    dotenv::dotenv().ok();
    env::var("DATABASE_URL").expect("Database Not Present")
}

fn set_secret() -> String {
    dotenv::dotenv().ok();
    env::var("SECRET").unwrap_or("development+localhost".to_string())
}