use crate::{
    auth::CurrentUser,
    flash::FlashMessage,
    handlers::dtos::todo::{FIELD_TASK, Todo},
    paths,
    views::{components::form, layout::base::base_layout},
};
use maud::{html, Markup};

pub fn todos(
    current_user: &CurrentUser,
    flash: &Option<FlashMessage>,
    todos: Vec<Todo>,
    task_value: Option<&str>,
    task_error: Option<&str>,
) -> Markup {
    let content = html! {
        div class="max-w-2xl mx-auto" {
            h1 class="text-2xl font-bold mb-6" { "Todos" }

            form method="POST" action=(paths::forms::TODOS) class="mb-6 space-y-4" {
                (form::input("text", FIELD_TASK, "New Todo", task_value, task_error))
                (form::submit_button("Add Todo"))
            }

            @if todos.is_empty() {
                p class="text-gray-500 text-center py-8" { "No todos yet. Add one above!" }
            } @else {
                ul class="space-y-2" {
                    @for todo in todos {
                        li class="flex items-center gap-3 p-3 bg-white border rounded-lg" id={"todo-" (todo.todo_id)} {
                            form
                                hx-patch={(paths::with_param(paths::actions::TODOS_TODO_ID_TOGGLE, "todo_id", &todo.todo_id))}
                                hx-swap="none"
                                class="flex-shrink-0"
                            {
                                input
                                    type="checkbox"
                                    checked[todo.is_done]
                                    onchange="this.form.requestSubmit()"
                                    class="w-5 h-5 cursor-pointer";
                            }

                            span class={
                                "flex-1 "
                                @if todo.is_done { "line-through text-gray-500" }
                            } {
                                (todo.task)
                            }

                            form
                                hx-delete={(paths::with_param(paths::actions::TODOS_TODO_ID, "todo_id", &todo.todo_id))}
                                hx-confirm="Are you sure you want to delete this todo?"
                                hx-target={"#todo-" (todo.todo_id)}
                                hx-swap="outerHTML"
                                class="flex-shrink-0"
                            {
                                button
                                    type="submit"
                                    class="text-red-600 hover:text-red-800 px-2 py-1"
                                {
                                    "Delete"
                                }
                            }
                        }
                    }
                }
            }
        }
    };

    base_layout(current_user, flash, "Todos", "Manage your todos", content)
}