use sqlx::sqlite::SqlitePool;

const DUMMY_DB: &str = "sqlite://books.db";

async fn get_db_dummy() {
    // Create database file if it doesn't exist and connect
    let db = SqlitePool::connect(DUMMY_DB).await.unwrap();
    
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
    
    let rows = sqlx::query("SELECT * FROM books").fetch_all(&db).await.unwrap();
    println!("Table Books has {} rows", rows.len());
}

pub async  fn connect_to_user_preference() {
    println!()
}

pub async fn connect_db() {
    get_db_dummy().await;
    connect_to_user_preference().await();
}