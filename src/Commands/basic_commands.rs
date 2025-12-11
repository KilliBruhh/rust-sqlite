use std::collections::HashMap;
use std::pin::Pin;
use crate::app::types::CommandHandler;
// pub type CommandHandler = fn(String) -> Pin<Box<dyn Future<Output=()> + Send>>;

fn cmd_clear(_args: String) -> Pin<Box<dyn Future<Output=()> + Send>> {
    Box::pin(async {
        println!("\x1B[2J\x1B[1;1H");
    })
}

fn cmd_help(_args: String) -> Pin<Box<dyn Future<Output=()> + Send>> {
    Box::pin(async {
        println!("--- Available Commands ---");
        println!("  clear : Wipes the terminal screen");
        println!("  help  : Shows this menu"    );
        println!("  state : Shows database connection state"    );
    })
}

pub fn create_command_map() -> HashMap<String, CommandHandler> {
    let mut map = HashMap::new();
    map.insert("clear".to_string(), cmd_clear as CommandHandler);
    map.insert("help".to_string(), cmd_help as CommandHandler);
    map
}