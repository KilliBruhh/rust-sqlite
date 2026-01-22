use std::collections::HashMap;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};
use crate::app;
use crate::app::types::CommandHandler;
use tokio::runtime::Runtime;
use crate::app::session_context::SessionStatus;

pub fn rustyline_session() -> Result<()> {
    let mut rl = DefaultEditor::new()?;
    let runtime = Runtime::new()?;
    let command_map = app::merger::create_command_map(); // Create the map once at startup
    let mut ctx = SessionStatus::new()?;
    loop {
        if ctx.should_quit {
            break;
        }
        let readline = rl.readline(">> ");
        match readline {    
            Ok(line) => {
                rl.add_history_entry(line.as_str()).expect("TODO: panic message");
                println!("-> {}", line);
                handle_command(&line, &command_map, &runtime, &mut ctx);
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

fn handle_command(line: &str, command_map: &HashMap<String, CommandHandler>, rt: &Runtime, _ctx: &mut SessionStatus) {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return;
    }
    let parts: Vec<&str> = trimmed.split_whitespace().collect();

    if let Some(cmd) = parts.get(0) {
        if let Some(handler) = command_map.get(*cmd) {
            let (args, options) : (String, String) = split_up_command(trimmed.to_string());

            //
            app::option_handler::call_option_handler(&options);

            // TODO - Change options type to list / vector (evry marked / found option should be 1 item in list).
            let future_tasks = handler(args, options, _ctx);    // Command - Options - Session Status
            rt.block_on(future_tasks);
        } else {
            println!("Invalid command. Use 'help' for help.");
        }
    }
}

fn split_up_command(line: String) -> (String, String) {
    let mut parts = line.splitn(2, char::is_whitespace);        // Splits line up 2 times (command and option)
    let command = parts.next().unwrap();
    let options = parts.next().unwrap_or("");
    return (command.to_string(), options.to_string());
}