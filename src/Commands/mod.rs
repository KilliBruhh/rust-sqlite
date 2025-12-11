pub mod basic_commands;
mod database_commands;

// TODO make a super file that will merge the command maps
pub use database_commands::create_command_map_db;
pub use basic_commands::create_command_map;
