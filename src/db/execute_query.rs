use std::pin::Pin;
use sqlx::FromRow;
use crate::db::connect_db;

#[derive(FromRow, Debug)]
struct SqliteDatabase {
    name: String,
    file: Option<String>,
}

pub fn execute_query(query: String) -> Pin<Box<dyn Future<Output = ()> + Send>> {
    Box::pin(async move {
        if let Some(pool) = connect_db::get_db_dummy().await {
            let query_result = sqlx::query(&query).execute(&pool).await;
            println!("SUCCES Query result: {query_result:?}");
        } else {
            println!("ERROR During Query execution.");
            return;
        }
    })
}

pub fn show_databases()-> Pin<Box<dyn Future<Output = ()> + Send>>  {
    Box::pin(async move {
        if let Some(pool) = connect_db::get_db_dummy().await {
            let database_list = sqlx::query_as::<_, SqliteDatabase>("PRAGMA database_list").fetch_all(&pool).await.unwrap();
            for db in database_list {
                println!("Database Name: {} (File: {:?})", db.name, db.file);
            }
        }
    })
}
// TODO: get autop table
pub fn show_tables() -> Pin<Box<dyn Future<Output = ()> + Send>>  {
    Box::pin(async move {
    })
}