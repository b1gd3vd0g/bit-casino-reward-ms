mod db;
mod router;

use std::net::SocketAddr;

use tokio::net::TcpListener;

use crate::router::router;

#[tokio::main]
async fn main() {
    let redis_client = db::redis::connect();
    let app = router().with_state(redis_client);

    let address = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(address).await.unwrap();
    println!("Listening on {}", address.to_string());
    axum::serve(listener, app).await.unwrap();
}
