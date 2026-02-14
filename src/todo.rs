use leptos::server;
use leptos_ui::clx::ServerFnError;

use crate::db::get_db_pool;

#[server]
pub async fn add_todo(title: String) -> Result<(), ServerFnError> {
    let conn = get_db_pool()?;

    let result = sqlx::query("INSERT INTO todos (title, completed) VALUES ($1, false)").bind(title).execute(conn).await;
    match result {
        Ok(_row) => Ok(()),
        Err(e) => Err(ServerFnError::ServerError(e.to_string()))
    }
}