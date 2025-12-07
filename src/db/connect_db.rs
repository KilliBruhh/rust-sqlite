use sqlx::sqlite::SqlitePool;

const DUMMY_DB: &str = "sqlite://books.db";

async fn get_db_dummy() {
    // Create database file if it doesn't exist and connect
    let connection_result = SqlitePool::connect(DUMMY_DB).await;
    let db = match connection_result {
        Ok(pool) => {
            println!("✅ Connection successful!");
            pool
        }
        Err(e) => {
            eprintln!("❌ Connection failed: {}", e);
            return; // Stop the function here since we have no DB to work with
        }
    };
    // Create the books table if it doesn't exist
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS books (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            author TEXT NOT NULL
        )"
    )
    .execute(&db)
    .await
    .unwrap();

    // Insert some dummy data
    sqlx::query("INSERT OR IGNORE INTO books (id, title, author) VALUES (1, 'Sample Book', 'Sample Author')")
        .execute(&db)
        .await
        .unwrap();

}

pub async fn connect_db() {
    get_db_dummy().await;
}