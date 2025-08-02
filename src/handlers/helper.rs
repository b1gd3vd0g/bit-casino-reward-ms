use axum::{
    Json,
    http::HeaderMap,
    response::{IntoResponse, Response},
};
use reqwest::StatusCode;
use uuid::Uuid;

use crate::{
    failure::Failure, handlers::responses::MessageResponse,
    requests::player::authenticate_player_token,
};

#[derive(Debug)]
pub enum AuthHeaderFailure {
    Nonparceable,
    NotFound,
    NoPrefix,
}

impl Failure for AuthHeaderFailure {
    fn message(&self) -> String {
        String::from(match self {
            Self::Nonparceable => "Authorization header value could not be parsed.",
            Self::NotFound => "Authorization header is missing.",
            Self::NoPrefix => "Authorization header missing \"Bearer \" prefix.",
        })
    }
}

/// Extract the player's authentication token from the HTTP headers.
/// # Arguments:
/// - `headers`: The HTTP headers from the request.
/// # Returns:
/// The value following the "Bearer " prefix in the Authorization header.
/// # Errors:
/// When the authentication token was not provided properly.
pub fn extract_authn_token(headers: &HeaderMap) -> Result<String, AuthHeaderFailure> {
    let authx_val = match headers.get("Authorization") {
        Some(value) => value,
        None => return Err(AuthHeaderFailure::NotFound),
    };

    let authx_val = match authx_val.to_str() {
        Ok(s) => s,
        Err(_) => return Err(AuthHeaderFailure::Nonparceable),
    };

    match authx_val.strip_prefix("Bearer ") {
        Some(tok) => Ok(String::from(tok)),
        None => Err(AuthHeaderFailure::NoPrefix),
    }
}

/// Authenticate a player based on the headers passed into the HTTP request.
/// # Arguments:
/// - `headers`: The HTTP headers from the request.
/// # Returns
/// The `player_id` of the authenticated player.
/// # Errors
/// Whenever the players cannot be authenticated for whatever reason.
/// It returns a valid HTTP response that can be directly returned from the handler.
pub async fn authenticate_player(headers: &HeaderMap) -> Result<Uuid, Response> {
    let token = match extract_authn_token(headers) {
        Ok(str) => str,
        Err(f) => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(MessageResponse::new(&f.message())),
            )
                .into_response());
        }
    };
    match authenticate_player_token(token).await {
        Ok(u) => Ok(u),
        Err(f) => Err((
            StatusCode::UNAUTHORIZED,
            Json(MessageResponse::new(&f.message())),
        )
            .into_response()),
    }
}
