pub type DbPool = sqlx::any::AnyPool;

pub async fn connection_builder() -> Result<DbPool, sqlx::Error> {
    let connectspec = dotenv::var("DATABASE_URL").expect("DATABASE_URL is not set");
    sqlx::any::AnyPool::connect(&connectspec).await
}
