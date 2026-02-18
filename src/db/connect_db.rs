use std::{fs};
use std::ffi::OsStr;
use sqlx::Executor;
use sqlx::sqlite::SqlitePool;

pub const DUMMY_DB: &str = "sqlite://books.db";

#[allow(dead_code)]
struct DatabaseConnection {
    pool: SqlitePool,
    name: String,
    location: String,
}

impl DatabaseConnection {


    #[allow(dead_code)]
    pub fn set_active_pool(&mut self, pool: SqlitePool) {
        self.pool = pool
    }
    #[allow(dead_code)]
    pub fn get_active_pool(&self) -> &SqlitePool {
        &self.pool
    }
}

/// 1. Create the connection pool
/// This function purely attempts to create the pool object.
pub async fn make_connection(db_url: &str) -> Result<SqlitePool, sqlx::Error> {
    SqlitePool::connect(db_url).await
}

pub async fn create_connection() {
    let pool = match make_connection(DUMMY_DB).await {
        Ok(p) => p,
        Err(e) => {
            eprintln!("ERROR: Failed to create pool: {}", e);
            return;
        }
    };
    if !check_connection(&pool).await {
        return;
    }
    let _db_connection = DatabaseConnection {
        pool,
        name: "".to_string(),
        location: "".to_string(),
    };
}


pub fn database_connection() {
    // Flow of function
    println!("- choose a database to connect to: ");
    search_db_files();
}

pub fn search_db_files() {
    let current_directory = std::env::current_dir().unwrap();
    let entries = fs::read_dir(current_directory).expect("Failed to read current directory");
    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_file() {
                if path.extension() == Some(OsStr::new("db")) {
                    println!("Found .db file at: {}", entry.path().display());
                }
            }
        }
    }
}

/// 2. Check if the connection is successful (Ping)
/// This executes a simple query to ensure the database is actually responsive.
pub async fn check_connection(pool: &SqlitePool) -> bool {
    // "SELECT 1" is a standard lightweight query to test connectivity
    match sqlx::query("SELECT 1").execute(pool).await {
        Ok(_) => {
            println!("SUCCES: Ping successful!");
            true
        }
        Err(e) => {
            eprintln!("FAIL: Ping failed: {}", e);
            false
        }
    }
}

/// 3. The Orchestrator (Get the pool)
/// This calls the previous two functions, initializes the schema, and returns the usable pool.
pub async fn get_db_dummy() -> Option<SqlitePool> {
    // Step 1: Make the connection
    let pool = match make_connection(DUMMY_DB).await {
        Ok(p) => p,
        Err(e) => {
            eprintln!("FAIL: Failed to create pool: {}", e);
            return None;
        }
    };

    // Step 2: Check the connection
    if !check_connection(&pool).await {
        return None;
    }

    // Step 3: Initialize Schema (Table creation & Dummy data)
    // It is good practice to keep migration logic separate, but we keep it here to match your original flow.
    let schema_setup = r#"
        CREATE TABLE IF NOT EXISTS books (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            author TEXT NOT NULL
        );
        INSERT OR IGNORE INTO books (id, title, author) VALUES (1, 'Sample Book', 'Sample Author');
    "#;

    // We use .execute_many() here to run multiple queries at once if supported,
    // or just run them individually as you did before.
    if let Err(e) = pool.execute(schema_setup).await {
        eprintln!("FAIL: Failed to initialize database schema: {}", e);
        return None;
    }

    println!("Database ready.");
    Some(pool)
}

