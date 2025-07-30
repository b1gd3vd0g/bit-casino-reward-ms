mod db;

fn main() {
    let redis_client = db::redis::connect();
}
