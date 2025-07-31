use std::env;

use redis::{AsyncCommands, RedisError, aio::MultiplexedConnection};

/// Return a MultiplexedConnection to the redis database.\
/// As this function is essential to our API, it will panic if anything goes wrong.
/// # Panics:
/// This function panics if
/// - env::REDIS_PASSWORD undefined.
/// - Invalid redis url constructed.
/// - Fail to make multiplexed connection.
pub async fn connect() -> MultiplexedConnection {
    let redis_password =
        env::var("REDIS_PASSWORD").expect("REDIS_PASSWORD environment variable is undefined.");
    let redis_url = format!("redis://:{}@redis:6379/", redis_password);

    let client = redis::Client::open(redis_url).expect("Invalid redis url.");

    client
        .get_multiplexed_tokio_connection()
        .await
        .expect("Failed to create multiplexed redis connection.")
}

pub async fn get_key(
    conn: &mut MultiplexedConnection,
    key: &str,
) -> Result<Option<String>, RedisError> {
    conn.get(key).await
}

pub async fn set_key(
    conn: &mut MultiplexedConnection,
    key: &str,
    value: &str,
) -> Result<(), RedisError> {
    conn.set(key, value).await
}
