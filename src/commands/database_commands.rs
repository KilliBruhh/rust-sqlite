use std::collections::HashMap;
use std::pin::Pin;
use crate::app::session_context::SessionStatus;
use crate::db::{connect_db, execute_query};
use crate::app::types;

fn cmd_connect_db(_args: String, _ctx: &mut SessionStatus) -> Pin<Box<dyn Future<Output = ()> + Send>> {
    Box::pin(async move { // <--- Added 'move' to ensure ownership is handled safely
        println!("Trying to connect to database...");
        let pool = connect_db::get_db_dummy().await;
        match pool {
            Some(pool) => {
                connect_db::check_connection(&pool).await;
            }
            None => {
                eprintln!("❌ Could not initialize DB, cannot check connection.");
            }
        }
    })
}

fn cmd_show(_args: String, _ctx: &mut SessionStatus) -> Pin<Box<dyn Future<Output = ()> + Send>> {
    Box::pin(async move {
        let query : String = "SELECT * FROM books".to_string();
        execute_query::execute_query(query).await;
    })
}

fn cmd_show_databases(_args: String, _ctx: &mut SessionStatus) -> Pin<Box<dyn Future<Output = ()> + Send>> {
    Box::pin(async move {
        execute_query::show_databases().await;
    })
}
fn cmd_show_tables(_args: String, _ctx: &mut SessionStatus) -> Pin<Box<dyn Future<Output = ()> + Send>> {
    Box::pin(async move {
        execute_query::show_tables().await;
    })
}

pub fn create_command_map_db() -> HashMap<String, types::CommandHandler> {
    let mut map: HashMap<String, types::CommandHandler> = HashMap::new();
    map.insert("state".to_string(), cmd_connect_db as types::CommandHandler);
    map.insert("show".to_string(), cmd_show as types::CommandHandler);
    map.insert("$dbs".to_string(), cmd_show_databases as types::CommandHandler);
    map.insert("$tables".to_string(), cmd_show_tables as types::CommandHandler);
    // Return the HashMap
    map
}