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
        daily_bonus::{check_bonus_availability, check_bonus_streak, set_bonus_claimed},
    },
    failure::Failure,
    handlers::{
        helper::authenticate_player,
        responses::{AvailabilityResponse, MessageResponse, StreakResponse},
    },
};

pub async fn handle_claim_daily_bonus(
    headers: HeaderMap,
    State(mut redis): State<MultiplexedConnection>,
) -> Response {
    let id = match authenticate_player(headers).await {
        Ok(u) => u,
        Err(r) => return r, // 401
    };
    match set_bonus_claimed(&mut redis, id).await {
        Ok(streak) => (StatusCode::OK, Json(StreakResponse::new(streak))).into_response(),
        Err(f) => (
            match f {
                RedisFailure::Conflict => StatusCode::CONFLICT,
                RedisFailure::Query(_) => StatusCode::INTERNAL_SERVER_ERROR,
            },
            Json(MessageResponse::new(&f.message())),
        )
            .into_response(),
    }
}

pub async fn handle_check_daily_bonus_availability(
    headers: HeaderMap,
    State(mut redis): State<MultiplexedConnection>,
) -> Response {
    let id = match authenticate_player(headers).await {
        Ok(u) => u,
        Err(r) => return r,
    };
    let available = match check_bonus_availability(&mut redis, id).await {
        Ok(b) => b,
        Err(f) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR, // 500
                Json(MessageResponse::new(&f.message())),
            )
                .into_response();
        }
    };
    (StatusCode::OK, Json(AvailabilityResponse::new(available))).into_response()
}

pub async fn handle_check_daily_bonus_streak(
    headers: HeaderMap,
    State(mut redis): State<MultiplexedConnection>,
) -> Response {
    let id = match authenticate_player(headers).await {
        Ok(u) => u,
        Err(r) => return r,
    };
    let streak = match check_bonus_streak(&mut redis, id).await {
        Ok(s) => s,
        Err(f) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(MessageResponse::new(&f.message())),
            )
                .into_response();
        }
    };
    (StatusCode::OK, Json(StreakResponse::new(streak))).into_response()
}
