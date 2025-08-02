use serde::Serialize;

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

#[derive(Serialize)]
pub struct AvailabilityResponse {
    available: bool,
}

impl AvailabilityResponse {
    pub fn new(available: bool) -> Self {
        Self {
            available: available,
        }
    }
}

#[derive(Serialize)]
pub struct StreakResponse {
    streak: u32,
}

impl StreakResponse {
    pub fn new(streak: u32) -> Self {
        Self { streak: streak }
    }
}
