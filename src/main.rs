mod db;

#[tokio::main]
async fn main() {
    db::connect_db::connect_db().await;
}