use std::pin::Pin;
use crate::db::connect_db;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{query, Executor};
use sqlx::query::Query;
use crate::db::connect_db::connect_db;

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