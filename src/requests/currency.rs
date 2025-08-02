use chrono::Utc;
use reqwest::{Client, StatusCode, header::HeaderMap};
use serde::Serialize;

use crate::failure::Failure;

#[derive(Serialize)]
pub struct PayoutRequestBody {
    amount: u32,
    reason: String,
}

impl PayoutRequestBody {
    fn new(amt: u32, reason: &str) -> Self {
        Self {
            amount: amt,
            reason: String::from(reason),
        }
    }
}

pub enum PayoutFailure {
    // The request failed to reach the currency microservice.
    RequestFailed,
    // Token doesn't authenticate a player.
    Unauthorized,
    Internal(String),
}

impl Failure for PayoutFailure {
    fn message(&self) -> String {
        String::from(match self {
            Self::RequestFailed => "Could not reach the currency microservice.",
            Self::Unauthorized => "Token authentication failed.",
            Self::Internal(m) => m,
        })
    }
}

pub async fn payout_daily_bonus(token: String, streak: u32) -> Result<(), PayoutFailure> {
    let client = Client::new();

    let mut hm = HeaderMap::new();
    hm.insert(
        "Authorization",
        format!("Bearer {}", token).parse().unwrap(),
    );

    let mut date = Utc::now().to_rfc3339();
    let _ = date.split_off(10);

    let amt = 128 * streak;
    let body = PayoutRequestBody::new(amt, &format!("DAILY_BONUS DATE={}", date));

    let response = client
        .post("http://currency-ms:3000/transaction")
        .headers(hm)
        .json(&body)
        .send()
        .await;

    let response = match response {
        Ok(r) => r,
        Err(_) => return Err(PayoutFailure::RequestFailed),
    };
    match response.status() {
        StatusCode::OK => Ok(()),
        StatusCode::UNAUTHORIZED => Err(PayoutFailure::Unauthorized),
        StatusCode::CONFLICT => Err(PayoutFailure::Internal(String::from(
            "This should not happen for a point increase.",
        ))),
        _ => Err(PayoutFailure::Internal(String::from(
            "Internal server error with payout service.",
        ))),
    }
}
