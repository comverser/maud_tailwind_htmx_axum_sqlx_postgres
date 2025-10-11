use serde::Deserialize;
use validator::Validate;

pub const FIELD_TASK: &str = "task";

#[derive(Deserialize, Validate)]
pub struct CreateTodoForm {
    #[validate(length(min = 1, message = "Task cannot be empty"))]
    pub task: String,
}

pub struct Todo {
    pub todo_id: i32,
    pub task: String,
    pub is_done: bool,
}
