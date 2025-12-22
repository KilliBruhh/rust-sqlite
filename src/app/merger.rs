use std::collections::HashMap;
use crate::app::types::CommandHandler;
use crate::commands;

//TODO make a merged map for sahred commands (quit, help, info, etc)

fn shared_map() -> HashMap<String, CommandHandler> {
    let mut map = HashMap::new();
    let basic_cmds = commands::basic_commands::create_command_map();
    map.extend(basic_cmds);
    map
}

pub fn create_command_map() -> HashMap<String, CommandHandler>{
    let mut merged_map = HashMap::new();

    let db_cmds = commands::database_commands::create_command_map_db();
    let test_cmds = commands::test_commands::create_command_map_test();

    merged_map.extend(shared_map());
    merged_map.extend(db_cmds);
    merged_map.extend(test_cmds);

    merged_map
}
pub fn create_query_command_map() -> HashMap<String, CommandHandler> {
    let mut merged_query_map = HashMap::new();

    let query_cmd = commands::query_commands::create_command_map();

    merged_query_map.extend(shared_map());
    merged_query_map.extend(query_cmd);

    merged_query_map
}