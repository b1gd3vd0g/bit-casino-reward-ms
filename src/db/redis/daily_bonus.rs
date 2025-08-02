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

/// Query the redis database to find out whether the player can claim their daily bonus or not.
/// # Arguments
/// - `conn`: The connection to the database
/// - `id`: The player's `player_id`
/// # Returns
/// - `true` when the bonus is available
/// - `false` when the bonus has already been claimed.
/// # Errors
/// - `RedisFailure::Query(e)` if the database query cannot be completed for some reason.
pub async fn check_bonus_availability(
    conn: &mut MultiplexedConnection,
    id: Uuid,
) -> Result<bool, RedisFailure> {
    let key = generate_key(id, 0);
    let search: Option<String> = match conn.get(key).await {
        Ok(opt) => opt,
        Err(e) => return Err(RedisFailure::Query(e)),
    };
    Ok(match search {
        Some(_) => false,
        None => true,
    })
}

/// Query the redis database to find out how many days long a player's current streak is.\
/// This function checks the database for records of today's streak - if it can't find that, it
/// checks yesterday as well.
/// # Arguments
/// * `conn` - The connection to the database.
/// * `id` - The player's `player_id`
/// # Returns
/// The player's streak (0 if it does not exist).
/// # Errors
/// - `RedisFailure::Query(e)` if either query fails.
pub async fn check_bonus_streak(
    conn: &mut MultiplexedConnection,
    id: Uuid,
) -> Result<u32, RedisFailure> {
    let today = generate_key(id, 0);
    let streak: Option<u32> = match conn.get(today).await {
        Ok(opt) => opt,
        Err(e) => return Err(RedisFailure::Query(e)),
    };
    match streak {
        Some(s) => return Ok(s),
        None => (),
    }

    let yesterday = generate_key(id, -1);
    let streak: Option<u32> = match conn.get(yesterday).await {
        Ok(s) => s,
        Err(e) => return Err(RedisFailure::Query(e)),
    };
    Ok(match streak {
        Some(s) => s,
        None => 0,
    })
}

/// Query the redis database to set the daily bonus as claimed.\
/// **Note:** This function does not actually reward a daily bonus - it simply marks it as claimed.
/// # Arguments
/// * `conn` - The connection to the database
/// * `id` - The player's `player_id`
/// # Returns
/// The player's streak after claiming the reward.
/// # Errors
/// - `RedisFailure::Query(e)` if the database query cannot be completed for some reason.
/// - `RedisFailure::Conflict` if the player has already claimed their daily bonus.
pub async fn set_bonus_claimed(
    conn: &mut MultiplexedConnection,
    id: Uuid,
) -> Result<u32, RedisFailure> {
    let already_claimed = match check_bonus_availability(conn, id).await {
        Ok(b) => b,
        Err(f) => return Err(f),
    };
    match already_claimed {
        true => return Err(RedisFailure::Conflict),
        false => (),
    }
    let streak = check_bonus_streak(conn, id).await?;
    let key = generate_key(id, 0);
    let streak = streak + 1;
    match conn.set_ex(key, streak, 48 * 3600).await {
        Ok(()) => Ok(streak),
        Err(e) => Err(RedisFailure::Query(e)),
    }
}
