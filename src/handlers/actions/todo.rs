use axum::{Extension, extract::{Path, State}, http::StatusCode, response::{IntoResponse, Redirect, Response}};
use sqlx::PgPool;

use crate::{
    auth::CurrentUser,
    data::commands,
    handlers::errors::HandlerError,
    paths,
};

pub async fn delete_actions_todos_todo_id(
    State(db): State<PgPool>,
    Extension(current_user): Extension<CurrentUser>,
    Path(todo_id): Path<i32>,
) -> Result<Response, HandlerError> {
    let user_id = current_user.require_authenticated();

    commands::todo::delete_todo(&db, user_id, todo_id).await?;

    // Return empty 200 response for HTMX to handle client-side removal
    Ok(StatusCode::OK.into_response())
}

pub async fn patch_actions_todos_todo_id_toggle(
    State(db): State<PgPool>,
    Extension(current_user): Extension<CurrentUser>,
    Path(todo_id): Path<i32>,
) -> Result<Response, HandlerError> {
    let user_id = current_user.require_authenticated();

    commands::todo::toggle_todo(&db, user_id, todo_id).await?;
    Ok(Redirect::to(paths::pages::TODOS).into_response())
}
