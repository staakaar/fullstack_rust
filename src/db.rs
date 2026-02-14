use sqlx::postgres::PgPool;
use leptos_ui::clx::ServerFnError;

static DB_POOL: tokio::sync::OnceCell<PgPool> = tokio::sync::OnceCell::const_new();

pub async fn init_db_pool(database_url: &str) -> Result<(), sqlx::Error> {
    let pool = PgPool::connect(database_url).await?;
    
    // マイグレーションを実行
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;
    
    DB_POOL.set(pool).ok();
    Ok(())
}

pub fn get_db_pool() -> Result<&'static PgPool, ServerFnError> {
    DB_POOL.get().ok_or_else(|| {
        ServerFnError::ServerError("Database pool not initialized".to_string())
    })
}