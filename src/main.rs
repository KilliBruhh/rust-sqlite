use crate::db::connect_db;

mod db;
mod session;
mod app;
mod commands;

fn main() {
    println!("\x1B[2J\x1B[1;1H");
    // TODO:
    // 1; Do not comment to a DB immediately, let user choose 1 using cmd (this will prevent async / await shit in main
    session::session::rustyline_session().expect("TODO: panic message");
}