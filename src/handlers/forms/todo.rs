use axum::{Extension, Form, extract::{Path, State}, http::StatusCode, response::{IntoResponse, Redirect, Response}};
use sqlx::PgPool;
use tower_sessions::Session;
use validator::Validate;

use crate::{
    auth::CurrentUser,
    data::commands,
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
        return Ok(render_validation_errors(&current_user, &form, &validation_errors));
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

fn render_validation_errors(
    current_user: &CurrentUser,
    form: &CreateTodoForm,
    validation_errors: &validator::ValidationErrors,
) -> Response {
    let errors = parse_validation_errors(validation_errors);
    (
        StatusCode::BAD_REQUEST,
        todos::todos(
            current_user,
            &None,
            vec![],
            Some(&form.task),
            errors.get(FIELD_TASK).map(String::as_str),
        ),
    )
        .into_response()
}
