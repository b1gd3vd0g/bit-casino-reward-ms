use axum::Router;
use redis::Client;

pub fn router() -> Router<Client> {
    Router::new()
}
