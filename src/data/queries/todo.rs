use crate::{data::errors::DataError, handlers::dtos::todo::Todo};
use sqlx::PgPool;

pub async fn get_todos_for_user(db: &PgPool, user_id: i32) -> Result<Vec<Todo>, DataError> {
    let todos = sqlx::query_as!(
        Todo,
        "SELECT todo_id, task, is_done FROM todos WHERE author_id = $1 ORDER BY created_at DESC",
        user_id
    )
    .fetch_all(db)
    .await
    .map_err(DataError::Database)?;

    Ok(todos)
}
