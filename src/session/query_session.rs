use std::collections::HashMap;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};
use tokio::runtime::Runtime;
use crate::app::session_context::SessionStatus;
use crate::app::merger::create_query_command_map;
use crate::app::types::CommandHandler;

pub fn query_rustyline_session() -> Result<()> {
    let mut rl = DefaultEditor::new()?;
    let _runtime = Runtime::new()?;
    let query_command_map = create_query_command_map();
    let mut ctx_query = SessionStatus::new()?;
    // Add command map but for query
    loop {
        if ctx_query.should_quit {
            break;
        }
        let read_query_line = rl.readline("$: ");
        match read_query_line {
            Ok(line) => {
                rl.add_history_entry(line.as_str()).expect("TODO: panic message");
                println!("{}", line);
                // Add command handling for Query Results
                handle_command(&line, &query_command_map, &_runtime, &mut ctx_query);
            }
            Err(ReadlineError::Interrupted) => {    // TODO: Quits the Query session not the parent session
                break;
            }
            Err(ReadlineError::Eof) => {            // TODO: Quits the Query session not the parent session
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

fn handle_command(line: &str, command_map: &HashMap<String, CommandHandler>, rt: &Runtime, _ctx: &mut SessionStatus) {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return;
    }
    let parts: Vec<&str> = trimmed.split_whitespace().collect();

    if let Some(cmd) = parts.get(0) {
        if let Some(handler) = command_map.get(*cmd) {
            let args: String = (&trimmed[cmd.len()..].trim_start()).to_string();
            let future_tasks = handler(args, _ctx);
            rt.block_on(future_tasks);
        } else {
            println!("Invalid command. Use 'help' for help.");
        }
    }
}