use std::collections::HashMap;
use std::pin::Pin;
use crate::app::session_context::SessionStatus;
use crate::app::types::CommandHandler;
// pub type CommandHandler = fn(String) -> Pin<Box<dyn Future<Output=()> + Send>>;

fn cmd_clear(_args: String, _ctx: &mut SessionStatus) -> Pin<Box<dyn Future<Output=()> + Send>> {
    Box::pin(async {
        println!("\x1B[2J\x1B[1;1H");
    })
}

fn cmd_help(_args: String, _ctx: &mut SessionStatus) -> Pin<Box<dyn Future<Output=()> + Send>> {
    Box::pin(async {
        println!("--- AVAILABLE COMMANDS ---");
        println!("=== Basic commands ==");
        println!("  clear : Wipes the terminal screen");
        println!("  help  : Shows this menu"    );
        println!("=== Data commands ===");
        println!("  state : Shows database connection state"    );
        println!("  show : will execute a SQL query"    );
    })
}

fn cmd_quit(_args: String, _ctx: &mut SessionStatus) -> Pin<Box<dyn Future<Output=()> + Send>> {
    println!("Quiting");
    (& mut *_ctx).quit();
    Box::pin(async {})
}

pub fn create_command_map() -> HashMap<String, CommandHandler> {
    let mut map = HashMap::new();
    map.insert("clear".to_string(), cmd_clear as CommandHandler);
    map.insert("help".to_string(), cmd_help as CommandHandler);
    map.insert("quit".to_string(), cmd_quit as CommandHandler);
    map
}