//! This module contains everything needed for safe, controlled communication with the redis
//! database. Redis is currently being used to cache information related to daily bonuses.

pub mod daily_bonus;

use std::env;

use redis::{RedisError, aio::MultiplexedConnection};

use crate::failure::Failure;

/// Return a MultiplexedConnection to the redis database.
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

/// This is what is returned from the query functions.
pub enum RedisFailure {
    /// The redis query failed for some reason. Passes the error that occured.
    Query(RedisError),
    /// The request conflicts with the state of the data (trying to claim daily bonus twice).
    Conflict,
}

impl Failure for RedisFailure {
    fn message(&self) -> String {
        match self {
            Self::Query(e) => e.to_string(),
            Self::Conflict => {
                String::from("You cannot claim the daily bonus twice in a single UTC day.")
            }
        }
    }
}
