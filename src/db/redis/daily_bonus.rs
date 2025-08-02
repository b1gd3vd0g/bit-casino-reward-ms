//! This module handles queries related to daily bonuses.
//!
//! # About Daily Bonuses
//!
//! Daily bonuses can be claimed one time by each player, each **day**. A **day** in this context
//! means the 24 hour period between midnights in the UTC time zone. For example, if you live in
//! the Pacific time zone, you can claim your daily reward for March 17 between 5:00 pm March 16 -
//! 5:00 pm March 17.
//!
//! Each cumulative daily bonus reclamation contributes to that player's **streak**. A streak ends
//! whenever a player fails to claim their daily bonus within any given UTC **day**.
//!
//! # How it works
//!
//! When a daily bonus is claimed, a key is added to the database including the player's id and the
//! UTC date. The associated value will be the player's **streak** achieved following that
//! reclamation. This key-value pair will stay in the database for 48 hours.
//!
//! **Claiming a daily bonus** involves making three queries to the database, to ensure that
//! player's streaks are maintained, and that bonuses are not claimed twice within the same UTC day.
//!
//! 1. Check to make sure that today's bonus has not already been claimed.
//! 2. Check to see whether or not *yesterday's* bonus was claimed. If it was, we will set the
//!    streak to equal to 1 greater than that streak; otherwise, it will simply be set to 1.
//! 3. Set the streak equal to the value determined in the previous step.
//!
//! This process ensures that the data will be cached in memory and readily available for just long
//! as it needs to be, so that a player could claim their bonus at UTC midnight on day 1, then at
//! UTC 11:59 PM on day 2, and not lose their streak - but it also allows for players to claim their
//! bonuses back to back, at UTC 11:59 day 1 and UTC midnight day 2.

use chrono::{Duration, Utc};
use redis::{AsyncCommands, aio::MultiplexedConnection};
use uuid::Uuid;

use crate::db::redis::RedisFailure;

fn generate_key(id: Uuid, days_from_now: i64) -> String {
    let mut date = (Utc::now() + Duration::hours(24 * days_from_now)).to_rfc3339();
    let _ = date.split_off(10);
    format!("daily_bonus_claimed:{}:{}", id, date)
}

/// Query the database to find out about a player's daily bonus.
/// # Arguments
/// - `redis`: The db connection.
/// - `id`: The player's id.
/// # Returns
/// - `0`: Whether or not the player's bonus is available for today.
/// - `1`: The player's streak.
/// # Errors
/// `RedisFailure::Query` if the database query fails due to a redis error.
pub async fn check_bonus(
    redis: &mut MultiplexedConnection,
    id: Uuid,
) -> Result<(bool, u32), RedisFailure> {
    let today = generate_key(id, 0);
    let streak: Option<u32> = match redis.get(today).await {
        Ok(opt) => opt,
        Err(e) => return Err(RedisFailure::Query(e)),
    };
    match streak {
        Some(s) => return Ok((false, s)),
        None => (),
    }

    let yesterday = generate_key(id, -1);
    let streak: Option<u32> = match redis.get(yesterday).await {
        Ok(s) => s,
        Err(e) => return Err(RedisFailure::Query(e)),
    };
    Ok(match streak {
        Some(s) => (true, s),
        None => (true, 0),
    })
}

/// Query the redis database to set the daily bonus as claimed.\
/// **Note:** This function does not actually reward a daily bonus - it simply marks it as claimed.
/// # Arguments
/// - `redis`: The db connection.
/// - `id`: The player's id.
/// # Returns
/// The player's streak after claiming the reward.
/// # Errors
/// - `RedisFailure::Query(e)` if the database query cannot be completed for some reason.
/// - `RedisFailure::Conflict` if the player has already claimed their daily bonus.
pub async fn set_bonus_claimed(
    conn: &mut MultiplexedConnection,
    id: Uuid,
) -> Result<u32, RedisFailure> {
    let bonus_info = match check_bonus(conn, id).await {
        Ok(tup) => tup,
        Err(f) => return Err(f),
    };
    match bonus_info.0 {
        true => (),
        false => return Err(RedisFailure::Conflict),
    }
    let key = generate_key(id, 0);
    let streak = bonus_info.1 + 1;
    match conn.set_ex(key, streak, 48 * 3600).await {
        Ok(()) => Ok(streak),
        Err(e) => Err(RedisFailure::Query(e)),
    }
}
