use dotenvy::dotenv;
use sqlx::PgPool;
use std::env;

pub async fn create_pool() -> PgPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set in .env");

    PgPool::connect(&database_url).await.expect("Error")
}
