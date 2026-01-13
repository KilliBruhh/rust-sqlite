use std::collections::HashMap;
use std::pin::Pin;
use crate::app::session_context::SessionStatus;
use crate::app::types::CommandHandler;
use crate::session::query_session;
// pub type CommandHandler = fn(String) -> Pin<Box<dyn Future<Output=()> + Send>>;

fn cmd_clear(_args: String, _option: String, _ctx: &mut SessionStatus) -> Pin<Box<dyn Future<Output=()> + Send>> {
    Box::pin(async {
        println!("\x1B[2J\x1B[1;1H");
    })
}

fn cmd_help(_args: String, _option: String, _ctx: &mut SessionStatus) -> Pin<Box<dyn Future<Output=()> + Send>> {
    Box::pin(async {
        // println!("   ")
        println!("--- AVAILABLE COMMANDS ---");
        println!("=== Basic commands ==");
        println!("  clear : Wipes the terminal screen");
        println!("  help  : Shows this menu"    );
        println!("  quit  : Quits the session");
        println!("  search: Looks for .db files");
        println!("  connect <.db File> : connect to file");
        // println!("  query : Starts the query session");
        println!("=== Database commands ===");
        println!("  state : Shows database connection state"    );
        println!("  show: : Runs a dummy select query "    );
    })
}


fn cmd_quit(_args: String, _option: String, _ctx: &mut SessionStatus) -> Pin<Box<dyn Future<Output=()> + Send>> {
    println!("Quiting");
    (& mut *_ctx).quit();
    Box::pin(async {})
}

fn cmd_query_session(_args: String, _option: String, _ctx: &mut SessionStatus) -> Pin<Box<dyn Future<Output=()> + Send>> {
    Box::pin(async move {
        let handle = tokio::task::spawn_blocking(move || {
            query_session::query_rustyline_session().expect("Query Session Failed");
        });
        handle.await.unwrap();
    })
}

fn cmd_search(_args: String, _option: String, _ctx: &mut SessionStatus) -> Pin<Box<dyn Future<Output=()> + Send>> {
    Box::pin(async move {
        println!("-> ARGS: {_args}  OPTION: {_option}");
    })
}
fn cmd_connect(_args: String, _option: String, _ctx: &mut SessionStatus) -> Pin<Box<dyn Future<Output=()> + Send>> {
    Box::pin(async move {

    })
}

// Todo replace with enum for performance
pub fn create_command_map() -> HashMap<String, CommandHandler> {
    let mut map = HashMap::new();
    map.insert("clear".to_string(), cmd_clear as CommandHandler);
    map.insert("help".to_string(), cmd_help as CommandHandler);
    map.insert("quit".to_string(), cmd_quit as CommandHandler);
    map.insert("query".to_string(), cmd_query_session as CommandHandler);
    map.insert("search".to_string(), cmd_search as CommandHandler);
    map.insert("connect".to_string(), cmd_connect as CommandHandler);
    map
}