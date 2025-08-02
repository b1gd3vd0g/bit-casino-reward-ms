//! This module handles the HTTP router serving all routes for the application. It utilizes the
//! handlers found in the `handlers` module. The documentation for the API can be found described in
//! the `openapi.yaml` file in the root, or served with a GET request to the base path.

use axum::{Router, routing::get};
use redis::aio::MultiplexedConnection;

use crate::handlers::daily_bonus::{
    handle_check_daily_bonus_availability, handle_check_daily_bonus_streak,
    handle_claim_daily_bonus,
};

pub fn router() -> Router<MultiplexedConnection> {
    Router::new()
        .route(
            "/daily",
            get(handle_check_daily_bonus_availability).post(handle_claim_daily_bonus),
        )
        .route("/daily/streak", get(handle_check_daily_bonus_streak))
}
