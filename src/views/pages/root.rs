use crate::{auth::CurrentUser, flash::FlashMessage, views::layout::base::base_layout};
use maud::{Markup, html};

pub fn root(current_user: &CurrentUser, flash: Option<&FlashMessage>) -> Markup {
    let content = html! {
        h1 class="text-5xl font-bold text-gray-900" { "Welcome" }
    };

    base_layout(current_user, flash, "Home", "Home page", content)
}
