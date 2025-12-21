use std::collections::HashMap;
use std::pin::Pin;
use crate::app::types::CommandHandler;
use crate::session::query_session;

fn cmd_test1(_args: String) -> Pin<Box<dyn Future<Output=()> + Send>> {
    Box::pin(async move {
        // 1. spawn_blocking moves the closure to a thread where blocking is okay
        let handle = tokio::task::spawn_blocking(move || {
            // This is your synchronous/blocking code
            query_session::query_rustyline_session().expect("Query Session Failed");
        });
        // The .unwrap() here handles the case where the thread itself panicked
        handle.await.unwrap();
    })
}

pub fn create_command_map_test() -> HashMap<String, CommandHandler> {
    let mut map = HashMap::new();
    map.insert("test1".to_string(), cmd_test1 as CommandHandler);
    map
}