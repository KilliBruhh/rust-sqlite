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

## License

This project is open source. Please check the repository for license information.

## Author

**KilliBruhh** - [GitHub](https://github.com/KilliBruhh)

---

**Note**: This project is under active development. Some features may be incomplete or subject to change.