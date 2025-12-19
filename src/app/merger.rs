use std::collections::HashMap;
use crate::app::types::CommandHandler;
use crate::commands;
pub fn create_command_map() -> HashMap<String, CommandHandler>{
    let mut merged_map = HashMap::new();
    let basic_cmds = commands::basic_commands::create_command_map();
    let db_cmds = commands::database_commands::create_command_map_db();
    merged_map.extend(basic_cmds);
    merged_map.extend(db_cmds);
    merged_map
}