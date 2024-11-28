use std::env;

use sqlx::sqlite::SqlitePool;

pub async fn connect() -> SqlitePool {
    dotenv::dotenv().expect("Failed to read .env file");

    let db_url = match env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(_) => "sqlite:todos.db".to_string(),
    };

    SqlitePool::connect(db_url.as_str()).await.unwrap()
}
