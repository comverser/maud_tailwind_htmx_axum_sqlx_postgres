use crate::{
    auth::CurrentUser,
    flash::FlashMessage,
    models::user::FIELD_EMAIL,
    paths,
    views::{components::form, layout::base::base_layout},
};
use maud::{html, Markup};

pub fn sign_in(
    current_user: &CurrentUser,
    flash: Option<&FlashMessage>,
    email_value: Option<&str>,
    email_error: Option<&str>,
) -> Markup {
    let content = html! {
        div class="max-w-sm mx-auto" {
            h1 class="text-2xl font-bold mb-6 text-center" { "Sign In" }

            p class="mb-6 text-center text-gray-600" {
                "Enter your email and we'll send you a magic link to sign in."
            }

            form method="POST" action=(paths::forms::SIGN_IN) class="space-y-4" {
                (form::input("email", FIELD_EMAIL, "Email", email_value, email_error))
                (form::submit_button("Send Magic Link"))
            }
        }
    };

    base_layout(current_user, flash, "Sign In", "Sign in", content)
}
