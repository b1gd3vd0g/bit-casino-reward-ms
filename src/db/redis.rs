use std::env;

use redis::Client;

pub fn connect() -> Client {
    redis::Client::open(format!(
        "redis://:{}@redis:6379/",
        env::var("REDIS_PASSWORD").expect("REDIS_PASSWORD environment variable is undefined.")
    ))
    .expect("Invalid redis url.")
}
