use std::collections::HashMap;
use std::pin::Pin;
use crate::app::session_context::SessionStatus;
use crate::app::types::CommandHandler;
use crate::session::query_session;


fn cmd_test1(_args: String, _ctx: &mut SessionStatus) -> Pin<Box<dyn Future<Output=()> + Send>> {
    Box::pin(async move {})
}

pub fn create_command_map_test() -> HashMap<String, CommandHandler> {
    let mut map = HashMap::new();
    map.insert("test1".to_string(), cmd_test1 as CommandHandler);
    map
}