mod db;
mod session;

#[tokio::main]
async fn main() {
    db::connect_db::connect_db().await;
    session::session::rustyline_session().expect("TODO: panic message");
}