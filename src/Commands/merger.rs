use std::collections::HashMap;
use std::pin::Pin;
use crate::app::types::CommandHandler;
use crate::Commands;
pub fn create_command_map() -> HashMap<String, CommandHandler>{
    let mut merged_map = HashMap::new();
    let basic_cmds = Commands::basic_commands::create_command_map();
    let db_cmds = Commands::database_commands::create_command_map_db();
    merged_map.extend(basic_cmds);
    merged_map.extend(db_cmds);
    merged_map
}