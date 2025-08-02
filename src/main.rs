mod db;
mod requests;
mod router;

use std::net::SocketAddr;

use tokio::net::TcpListener;

use crate::router::router;

#[tokio::main]
async fn main() {
    let redis_connection = db::redis::connect().await;
    let app = router().with_state(redis_connection);

    let address = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(address).await.unwrap();
    println!("Listening on {}", address.to_string());
    axum::serve(listener, app).await.unwrap();
}
