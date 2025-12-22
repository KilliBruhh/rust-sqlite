use std::collections::HashMap;
use std::pin::Pin;
use crate::app::session_context::SessionStatus;
use crate::app::types::CommandHandler;

fn cmd_quit_query(_args: String, _ctx: &mut SessionStatus) -> Pin<Box<dyn Future<Output=()> + Send>> {
    (& mut * _ctx).quit();
    Box::pin(async move {})
}

pub fn create_command_map() -> HashMap<String, CommandHandler> {
    let mut map = HashMap::new();
    map.insert("quit".to_string(), cmd_quit_query as CommandHandler);

    map
}