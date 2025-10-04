use crate::{auth::CurrentUser, paths};
use maud::{html, Markup};

pub(crate) fn navbar(current_user: &CurrentUser) -> Markup {
    html! {
        header class="bg-white shadow-sm" {
            nav class="container mx-auto px-4" {
                div class="flex justify-between h-16 items-center" {
                    div class="flex gap-6" {
                        a href=(paths::pages::ROOT) class="hover:text-blue-600 transition-colors" { "Home" }
                        @match current_user {
                            CurrentUser::Authenticated { .. } => {
                                a href=(paths::pages::CREATE) class="hover:text-blue-600 transition-colors" { "Create" }
                                a href=(paths::pages::TODOS) class="hover:text-blue-600 transition-colors" { "Todos" }
                                form method="post" action=(paths::actions::SIGN_OUT) class="inline" {
                                    button type="submit" class="hover:text-blue-600 transition-colors" { "Sign Out" }
                                }
                            }
                            CurrentUser::Guest => {
                                a href=(paths::pages::SIGN_UP) class="hover:text-blue-600 transition-colors" { "Sign Up" }
                                a href=(paths::pages::SIGN_IN) class="hover:text-blue-600 transition-colors" { "Sign In" }
                            }
                        }
                    }
                }
            }
        }
    }
}