use crate::{auth::CurrentUser, paths};
use maud::{html, Markup};

pub fn navbar(current_user: &CurrentUser) -> Markup {
    html! {
        header class="bg-white shadow-sm" {
            nav class="container mx-auto px-4" {
                div class="flex justify-between h-16 items-center" {
                    div class="flex gap-6" {
                        a href=(paths::pages::ROOT) class="hover:text-blue-600 transition-colors" { "Home" }
                        @match current_user {
                            CurrentUser::Authenticated { .. } => {
                                a href=(paths::pages::DASHBOARD) class="hover:text-blue-600 transition-colors" { "Dashboard" }
                                a href=(paths::pages::TEXT_ANALYZER) class="hover:text-blue-600 transition-colors" { "Text Analyzer" }
                                a href=(paths::pages::TODOS) class="hover:text-blue-600 transition-colors" { "Todos" }
                                form method="post" action=(paths::actions::SIGN_OUT) class="inline" {
                                    button type="submit" class="hover:text-blue-600 transition-colors" { "Sign Out" }
                                }
                            }
                            CurrentUser::Guest => {
                                a href=(paths::pages::SIGN_IN) class="hover:text-blue-600 transition-colors" { "Sign In" }
                            }
                        }
                    }
                }
            }
        }
    }
}