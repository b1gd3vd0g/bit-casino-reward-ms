//! This module keeps the types of structs that are returned in the JSON response bodies.
use serde::Serialize;

/// Returned whenever an error occured and the request could not be completed.
#[derive(Serialize)]
pub struct MessageResponse {
    message: String,
}

impl MessageResponse {
    pub fn new(message: &str) -> Self {
        MessageResponse {
            message: String::from(message),
        }
    }
}

/// Returned from a successful check request.
#[derive(Serialize)]
pub struct CheckResponse {
    available: bool,
    streak: u32,
}

impl CheckResponse {
    pub fn new(tup: (bool, u32)) -> Self {
        Self {
            available: tup.0,
            streak: tup.1,
        }
    }
}

/// Returned from a successful Claim request.
#[derive(Serialize)]
pub struct StreakResponse {
    streak: u32,
}

impl StreakResponse {
    pub fn new(streak: u32) -> Self {
        Self { streak: streak }
    }
}
