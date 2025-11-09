use crate::data::{ensure_rows_affected, errors::DataError};
use sqlx::PgPool;

pub async fn create_todo(db: &PgPool, user_id: i32, task: &str) -> Result<(), DataError> {
    sqlx::query!(
        "INSERT INTO todos(task, author_id) VALUES($1, $2)",
        task,
        user_id
    )
    .execute(db)
    .await
    .map_err(DataError::Database)?;

    Ok(())
}

pub async fn toggle_todo(db: &PgPool, user_id: i32, todo_id: i32) -> Result<(), DataError> {
    let result = sqlx::query!(
        "UPDATE todos SET is_done = NOT is_done WHERE todo_id = $1 AND author_id = $2",
        todo_id,
        user_id
    )
    .execute(db)
    .await
    .map_err(DataError::Database)?;

    ensure_rows_affected(result, "Todo not found")
}

pub async fn delete_todo(db: &PgPool, user_id: i32, todo_id: i32) -> Result<(), DataError> {
    let result = sqlx::query!(
        "DELETE FROM todos WHERE todo_id = $1 AND author_id = $2",
        todo_id,
        user_id
    )
    .execute(db)
    .await
    .map_err(DataError::Database)?;

    ensure_rows_affected(result, "Todo not found")
}
