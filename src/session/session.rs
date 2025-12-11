use std::collections::HashMap;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};
use crate::Commands;

pub fn rustyline_session() -> Result<()> {
    let mut rl = DefaultEditor::new()?;
    let command_map = Commands::create_command_map(); // Create the map once at startup
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str()).expect("TODO: panic message");
                println!("-> {}", line);
                handle_command(&line, &command_map);
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    Ok(())
}

fn handle_command(line: &str, command_map: &HashMap<String, crate::Commands::basic_commands::CommandHandler>) {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return;
    }
    let parts: Vec<&str> = trimmed.split_whitespace().collect();
    if let Some(cmd) = parts.get(0) {
        if let Some(handler) = command_map.get(*cmd) {
            let args = &trimmed[cmd.len()..].trim_start();
            handler(args);
        } else {
            println!("Invalid command. Use 'help' for help.");
        }
    }
}