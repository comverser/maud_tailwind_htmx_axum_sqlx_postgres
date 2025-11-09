use axum::{Extension, Form, extract::{Path, State}, http::StatusCode, response::{IntoResponse, Redirect, Response}};
use sqlx::PgPool;
use tower_sessions::Session;
use validator::Validate;

use crate::{
    auth::CurrentUser,
    data::{commands, queries},
    flash::FlashMessage,
    handlers::dtos::todo::{CreateTodoForm, FIELD_TASK},
    handlers::errors::HandlerError,
    paths::pages,
    views::pages::todos,
};

use super::parse_validation_errors;

pub async fn post_forms_todos(
    State(db): State<PgPool>,
    Extension(current_user): Extension<CurrentUser>,
    session: Session,
    Form(form): Form<CreateTodoForm>,
) -> Result<Response, HandlerError> {
    let user_id = current_user.require_authenticated();

    if let Err(validation_errors) = form.validate() {
        return render_validation_errors(&db, &current_user, user_id, &form, &validation_errors).await;
    }

    commands::todo::create_todo(&db, user_id, form.task.trim()).await?;
    FlashMessage::success("Todo created successfully").set(&session).await?;
    Ok(Redirect::to(pages::TODOS).into_response())
}

pub async fn post_forms_todos_todo_id_toggle(
    State(db): State<PgPool>,
    Extension(current_user): Extension<CurrentUser>,
    Path(todo_id): Path<i32>,
) -> Result<Response, HandlerError> {
    let user_id = current_user.require_authenticated();

    commands::todo::toggle_todo(&db, user_id, todo_id).await?;
    Ok(Redirect::to(pages::TODOS).into_response())
}

async fn render_validation_errors(
    db: &PgPool,
    current_user: &CurrentUser,
    user_id: i32,
    form: &CreateTodoForm,
    validation_errors: &validator::ValidationErrors,
) -> Result<Response, HandlerError> {
    let errors = parse_validation_errors(validation_errors);
    let todos_list = queries::todo::get_todos_for_user(db, user_id).await?;

    Ok((
        StatusCode::BAD_REQUEST,
        todos::todos(
            current_user,
            &None,
            todos_list,
            Some(&form.task),
            errors.get(FIELD_TASK).map(String::as_str),
        ),
    )
        .into_response())
}
