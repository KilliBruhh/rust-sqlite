mod db;
mod session;
mod app;
mod Commands;

fn main() {
    db::connect_db::connect_db();
    session::session::rustyline_session().expect("TODO: panic message");
}