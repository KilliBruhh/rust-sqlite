use std::collections::HashMap;

pub type CommandHandler = fn(&str);

fn cmd_clear(_args: &str) {
    println!("\x1B[2J\x1B[1;1H");
}

fn cmd_help(_args: &str) {
    println!("--- Available Commands ---");
    println!("  clear : Wipes the terminal screen");
    println!("  help  : Shows this menu"    );
}

pub fn create_command_map() -> HashMap<String, CommandHandler> {
    let mut map = HashMap::new();
    map.insert("clear".to_string(), cmd_clear as CommandHandler);
    map.insert("help".to_string(), cmd_help as CommandHandler);
    map
}