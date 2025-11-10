use crate::{auth::CurrentUser, flash::FlashMessage, views::layout::base::base_layout};
use maud::{Markup, html};

pub fn server_error(current_user: &CurrentUser, flash: Option<&FlashMessage>, message: &str) -> Markup {
    let content = html! {
        h1 class="text-6xl font-bold text-gray-900 mb-4" { "500" }
        p class="text-red-600" { (message) }
    };

    base_layout(current_user, flash, "Server Error", "Server error", content)
}
