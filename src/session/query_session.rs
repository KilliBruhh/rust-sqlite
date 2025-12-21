use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};
use tokio::runtime::Runtime;

pub fn query_rustyline_session() -> Result<()> {
    let mut rl = DefaultEditor::new()?;
    let _runtime = Runtime::new()?;
    // Add command map but for query
    loop {
        let read_query_line = rl.readline("=>");
        match read_query_line {
            Ok(line) => {
                rl.add_history_entry(line.as_str()).expect("TODO: panic message");
                println!("{}", line);
                // Add command handling for Query Results
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