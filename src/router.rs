use axum::Router;
use redis::aio::MultiplexedConnection;

pub fn router() -> Router<MultiplexedConnection> {
    Router::new()
}
