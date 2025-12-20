use std::collections::HashMap;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};
use crate::app;
use crate::app::types::CommandHandler;
use tokio::runtime::Runtime;

pub fn rustyline_session() -> Result<()> {
    let mut rl = DefaultEditor::new()?;
    let runtime = Runtime::new()?;
    let command_map = app::merger::create_command_map(); // Create the map once at startup
    loop {
        let readline = rl.readline(">> ");
        match readline {    
            Ok(line) => {
                rl.add_history_entry(line.as_str()).expect("TODO: panic message");
                println!("-> {}", line);
                handle_command(&line, &command_map, &runtime);
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err     ) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    Ok(())
}

// TODO FIX THIS SO TI WILL WORK WITH NEW COMMANDHANDLER
fn handle_command(line: &str, command_map: &HashMap<String, CommandHandler>, rt: &Runtime) {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return;
    }
    let parts: Vec<&str> = trimmed.split_whitespace().collect();

    if let Some(cmd) = parts.get(0) {
        if let Some(handler) = command_map.get(*cmd) {
            let args: String = (&trimmed[cmd.len()..].trim_start()).to_string();
            let future_tasks = handler(args);
            rt.block_on(future_tasks);
        } else {
            println!("Invalid command. Use 'help' for help.");
        }
    }
}