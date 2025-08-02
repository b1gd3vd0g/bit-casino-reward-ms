//! This module makes HTTP requests to the player microservice.

use reqwest::{Client, StatusCode, header::HeaderMap};
use serde::Deserialize;
use uuid::Uuid;

use crate::failure::Failure;

/// This can be used to pull the `id` field from a JSON response body (for example, `GET
/// <player-ms>/authn`).
#[derive(Deserialize)]
pub struct PlayerId {
    pub id: Uuid,
}

pub enum TokenAuthnFailure {
    /// The request failed to reach the player microservice.
    RequestFailed,
    /// The provided token could not be used to authenticate a player.
    BadToken,
}

impl Failure for TokenAuthnFailure {
    fn message(&self) -> String {
        String::from(match self {
            Self::RequestFailed => "Could not reach the player microservice to authenticate token.",
            Self::BadToken => "The provided token could not be used to authenticate the player.",
        })
    }
}

/// Make a request to the player microservice attempting to validate a player authentication token.\
/// **Request made:** `GET <player-ms>/authn`
/// # Arguments
/// - `token`: The player's authentication JWT.
/// # Returns
/// The player_id of the authenticated player.
/// # Errors
/// - `TokenAuthnFailure::RequestFailed` if the player microservice can't be reached.
/// - `TokenAuthnFailure::BadToken` if the player can't be authenticated.
pub async fn authenticate_player_token(token: String) -> Result<Uuid, TokenAuthnFailure> {
    let client = Client::new();

    let mut hm = HeaderMap::new();
    hm.insert(
        "Authorization",
        format!("Bearer {}", token).parse().unwrap(),
    );

    let response = client
        .get("http://player-ms:3000/authn")
        .headers(hm)
        .send()
        .await;

    let response = match response {
        Ok(r) => r,
        Err(_) => return Err(TokenAuthnFailure::RequestFailed),
    };

    match response.status() {
        StatusCode::OK => {
            let player: PlayerId = response.json().await.unwrap();
            Ok(player.id)
        }
        _ => Err(TokenAuthnFailure::BadToken),
    }
}
