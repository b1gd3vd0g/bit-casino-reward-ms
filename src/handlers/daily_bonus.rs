//! This module holds the HTTP handler functions for checking and claiming daily bonuses.

use axum::{
    Json,
    extract::State,
    http::HeaderMap,
    response::{IntoResponse, Response},
};
use redis::aio::MultiplexedConnection;
use reqwest::StatusCode;

use crate::{
    db::redis::{
        RedisFailure,
        daily_bonus::{check_bonus, set_bonus_claimed},
    },
    failure::Failure,
    handlers::{
        helper::{authenticate_player, extract_authn_token},
        responses::{CheckResponse, MessageResponse, StreakResponse},
    },
    requests::currency::{PayoutFailure, payout_daily_bonus},
};

/// Handle the HTTP request to check somebodys daily bonus information.
/// # Arguments:
/// - `headers`: The request HTTP headers (should include a bearer token in the Authorization
///   header).
/// - `redis`: The async redis connection.
pub async fn handle_check_daily_bonus(
    headers: HeaderMap,
    State(mut redis): State<MultiplexedConnection>,
) -> Response {
    let id = match authenticate_player(&headers).await {
        Ok(u) => u,
        Err(r) => return r,
    };
    match check_bonus(&mut redis, id).await {
        Ok(tup) => (StatusCode::OK, Json(CheckResponse::new(tup))).into_response(),
        Err(f) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(MessageResponse::new(&f.message())),
        )
            .into_response(),
    }
}

/// Handle the HTTP request to claim a daily bonus.
/// # Arguments:
/// - `headers`: The request HTTP headers (should include a bearer token in the Authorization
///   header).
/// - `redis`: The async redis connection.
pub async fn handle_claim_daily_bonus(
    headers: HeaderMap,
    State(mut redis): State<MultiplexedConnection>,
) -> Response {
    let id = match authenticate_player(&headers).await {
        Ok(u) => u,
        Err(r) => return r,
    };
    let token = extract_authn_token(&headers).unwrap();

    let streak = match set_bonus_claimed(&mut redis, id).await {
        Ok(s) => s,
        Err(f) => {
            return (
                match f {
                    RedisFailure::Conflict => StatusCode::CONFLICT,
                    RedisFailure::Query(_) => StatusCode::INTERNAL_SERVER_ERROR,
                },
                Json(MessageResponse::new(&f.message())),
            )
                .into_response();
        }
    };
    match payout_daily_bonus(token, streak).await {
        Ok(_) => return (StatusCode::OK, Json(StreakResponse::new(streak))).into_response(),
        Err(f) => {
            return (
                match f {
                    PayoutFailure::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
                    PayoutFailure::Unauthorized => StatusCode::UNAUTHORIZED,
                    PayoutFailure::RequestFailed => StatusCode::INTERNAL_SERVER_ERROR,
                },
                Json(MessageResponse::new(&f.message())),
            )
                .into_response();
        }
    }
}
