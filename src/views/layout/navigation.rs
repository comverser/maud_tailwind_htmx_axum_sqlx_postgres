use crate::{auth::CurrentUser, paths};
use maud::{html, Markup};

pub fn navbar(current_user: &CurrentUser) -> Markup {
    html! {
        header class="border-b" {
            nav class="container mx-auto px-4 py-4" {
                div class="flex gap-4" {
                    a href=(paths::pages::ROOT) class="hover:text-indigo-600" { "Home" }
                    @match current_user {
                        CurrentUser::Authenticated { .. } => {
                            a href=(paths::pages::DASHBOARD) class="hover:text-indigo-600" { "Dashboard" }
                            a href=(paths::pages::TEXT_ANALYZER) class="hover:text-indigo-600" { "Text Analyzer" }
                            a href=(paths::pages::TODOS) class="hover:text-indigo-600" { "Todos" }
                            form method="post" action=(paths::actions::SIGN_OUT) class="inline" {
                                button type="submit" class="hover:text-indigo-600" { "Sign Out" }
                            }
                        }
                        CurrentUser::Guest => {
                            a href=(paths::pages::SIGN_IN) class="hover:text-indigo-600" { "Sign In" }
                        }
                    }
                }
            }
        }
    }
}