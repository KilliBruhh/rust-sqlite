use std::collections::HashMap;
use std::pin::Pin;
use crate::db::connect_db;
use crate::app::types::CommandHandler;


// "A Function that that returns a Future we can await later"
// pub type CommandHandler = fn(String) -> Pin<Box<dyn Future<Output=()> + Send>>;

fn cmd_connect_db(_args: String) -> Pin<Box<dyn Future<Output = ()> + Send>> {
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

pub fn create_command_map_db() -> HashMap<String, CommandHandler> {
    let mut map: HashMap<String, CommandHandler> = HashMap::new();
    map.insert("state".to_string(), cmd_connect_db as CommandHandler);
    map
}