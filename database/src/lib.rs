use async_once::AsyncOnce;
use lazy_static::lazy_static;
use sqlx::sqlite::SqlitePool;

const DATABASE_URL: &'static str = env!("DATABASE_URL");

lazy_static! {
    static ref DB: AsyncOnce<SqlitePool> = AsyncOnce::new(async {
        match SqlitePool::connect(&DATABASE_URL).await {
            Ok(db_pool) => {
                println!("Database Connection: OK");

                db_pool
            }
            Err(e) => {
                eprintln!("Db pool connection errors: {}", e);
                std::process::exit(1);
            }
        }
    });
}
