# Rust SQLite Tool

An interactive command-line REPL (Read-Eval-Print Loop) tool for managing and interacting with SQLite databases, built with Rust. Features asynchronous operations, connection pooling, and an extensible command system.

## Features

- **Interactive REPL Interface** - Terminal-based command interface with readline support and command history
- **Async Operations** - Full async/await support using Tokio runtime for efficient database operations
- **Connection Pooling** - Automatic connection management and health monitoring
- **Nested Sessions** - Dual-REPL design with main session and specialized query sub-sessions
- **Command System** - Extensible command architecture with easy-to-add custom commands
- **Database Health Checks** - Real-time connection validation with status indicators
- **Auto-Schema Initialization** - Automatic table creation and sample data on first connection

## Installation

### Prerequisites

- Rust 1.70 or higher (uses Rust 2024 edition)
- Cargo package manager

### Build from Source

```bash
# Clone the repository
git clone https://github.com/KilliBruhh/rust-sqlite.git
cd rust-sqlite

# Build the project
cargo build --release

# Run the application
cargo run --release
```

## Usage

### Starting the REPL

```bash
cargo run
```

The application will:
1. Clear the screen
2. Connect to the default database (`database_files/library.db`)
3. Initialize the schema with sample data
4. Launch the main REPL with the `>>` prompt

### Basic Commands

#### Main Session (`>>` prompt)

```bash
>> help                  # Display available commands and usage information
>> state                 # Check database connection status with ping verification
>> show                  # Execute sample query (SELECT * FROM books)
>> test1                 # Enter nested query session
>> clear                 # Clear the terminal screen
>> quit                  # Exit the application
```

#### Query Session (`=>` prompt)

```bash
>> test1                 # Enter query session
=> help                  # Display query session commands
=> quit                  # Return to main session
```

### Example Session

```
>> state
Database connection is active ✅

>> show
Executing: SELECT * FROM books
[Query results displayed here]

>> test1
Entering query session...
=> help
Available query commands: ...
=> quit
Returning to main session...

>> quit
Goodbye!
```

## Commands Reference

### Main Session Commands

| Command | Description |
|---------|-------------|
| `help` | Display all available commands |
| `state` | Verify database connection with health check |
| `show` | Execute predefined query |
| `test1` | Launch nested query session |
| `clear` | Clear terminal screen |
| `quit` | Exit application |

### Query Session Commands

Query session provides specialized commands for query-specific operations (accessed via `test1` command).

## Architecture

The project follows a modular architecture with clear separation of concerns:

```
src/
├── main.rs                      # Entry point and initialization
├── app/                         # Application coordination
│   ├── mod.rs                   # Module exports
│   ├── merger.rs                # Command registry merging
│   ├── session_context.rs      # Session state management
│   └── types.rs                 # Type definitions (CommandFunc, etc.)
├── commands/                    # Command handlers
│   ├── mod.rs                   # Command exports
│   ├── basic_commands.rs        # Basic utilities (help, clear, quit)
│   ├── database_commands.rs    # Database operations (state, show)
│   ├── query_commands.rs        # Query session commands
│   └── test_commands.rs         # Testing/experimental features
├── db/                          # Database layer
│   ├── mod.rs                   # Database exports
│   ├── connect_db.rs            # Connection pooling and initialization
│   └── execute_query.rs         # Query execution wrapper
├── session/                     # Session management
│   ├── mod.rs                   # Session exports
│   ├── session.rs               # Main REPL implementation
│   └── query_session.rs         # Nested query session
└── ui/                          # UI utilities (future use)
```

### Key Components

- **Session Layer** ([src/session/](src/session/)) - Manages REPL loops and user input
- **Command System** ([src/commands/](src/commands/)) - Extensible command handlers with HashMap-based dispatch
- **Database Layer** ([src/db/](src/db/)) - SQLx-based async database operations
- **App Coordination** ([src/app/](src/app/)) - Command registry merging and session context

## Technology Stack

- **[SQLx](https://github.com/launchbadge/sqlx)** (0.8.6) - Async SQL toolkit with compile-time checked queries
- **[Tokio](https://tokio.rs/)** (1.0) - Async runtime for concurrent operations
- **[Rustyline](https://github.com/kkawakam/rustyline)** (17.0.2) - Readline implementation for command-line editing

## Development

### Project Structure

- `src/` - Source code
- `database_files/` - SQLite database files
  - `library.db` - Default database with books table
  - `test.db` - Test database
  - `tutorial.db` - Tutorial database
- `target/` - Build artifacts (gitignored)

### Adding New Commands

1. Create a new async function in the appropriate command module:

```rust
use crate::app::types::CommandFunc;
use crate::app::session_context::SessionContext;

pub fn my_command() -> CommandFunc {
    Box::pin(async move |_context: &mut SessionContext| {
        println!("My custom command!");
        Ok(())
    })
}
```

2. Add the command to the HashMap in the module's `get_commands()` function:

```rust
pub fn get_commands() -> HashMap<String, CommandFunc> {
    let mut commands = HashMap::new();
    commands.insert("mycommand".to_string(), my_command());
    commands
}
```

3. The command will be automatically merged into the main command registry.

### Running Tests

```bash
cargo test
```

### Building for Release

```bash
cargo build --release
./target/release/rust_sqlite_tool
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

### Development Roadmap

- [ ] Improve error handling (replace panics with proper error messages)
- [ ] Allow query session to return to parent session instead of quitting entirely
- [ ] Add support for custom database connections
- [ ] Implement query result formatting and pagination
- [ ] Add transaction support
- [ ] Create comprehensive test suite

## License

This project is open source. Please check the repository for license information.

## Author

**KilliBruhh** - [GitHub](https://github.com/KilliBruhh)

---

**Note**: This project is under active development. Some features may be incomplete or subject to change.

test