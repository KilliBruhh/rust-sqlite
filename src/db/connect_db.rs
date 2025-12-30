use std::panic::Location;
use sqlx::Executor;
use sqlx::sqlite::SqlitePool;

pub const DUMMY_DB: &str = "sqlite://books.db";

struct DatabaseConnection {
    pool: SqlitePool,
    name: String,
    location: String,
}

impl DatabaseConnection {
    pub fn set_active_pool(&mut self, pool: SqlitePool) {
        self.pool = pool
    }
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
            eprintln!("❌ Failed to create pool: {}", e);
            return;
        }
    };
    if !check_connection(&pool).await {
        return;
    }
    let mut db_connection = DatabaseConnection {
        pool,
        name: "".to_string(),
        location: "".to_string(),
    };
}

/// 2. Check if the connection is successful (Ping)
/// This executes a simple query to ensure the database is actually responsive.
pub async fn check_connection(pool: &SqlitePool) -> bool {
    // "SELECT 1" is a standard lightweight query to test connectivity
    match sqlx::query("SELECT 1").execute(pool).await {
        Ok(_) => {
            println!("✅ Ping successful!");
            true
        }
        Err(e) => {
            eprintln!("❌ Ping failed: {}", e);
            false
        }
    }
}

pub fn on_call_db() {

}

/// 3. The Orchestrator (Get the pool)
/// This calls the previous two functions, initializes the schema, and returns the usable pool.
pub async fn get_db_dummy() -> Option<SqlitePool> {
    // Step 1: Make the connection
    let pool = match make_connection(DUMMY_DB).await {
        Ok(p) => p,
        Err(e) => {
            eprintln!("❌ Failed to create pool: {}", e);
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
        eprintln!("❌ Failed to initialize database schema: {}", e);
        return None;
    }

    println!("✅ Database ready.");
    Some(pool)
}

