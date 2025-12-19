mod db;
mod session;
mod app;
mod Commands;

fn main() {
    println!("\x1B[2J\x1B[1;1H");
    // TODO:
    // 1; Do not commect to a DB immediatly, let user choose 1 using cmd (this will prevent async / await shit in main
    let _ = db::connect_db::connect_db();
    session::session::rustyline_session().expect("TODO: panic message");
}