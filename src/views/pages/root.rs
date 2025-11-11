use crate::{auth::CurrentUser, flash::FlashMessage, paths, views::layout::base::base_layout};
use maud::{Markup, html};

pub fn root(
    current_user: &CurrentUser,
    flash: Option<&FlashMessage>,
    site_name: &str,
    email: Option<&str>,
    message: Option<&str>,
    email_error: Option<&str>,
    message_error: Option<&str>,
) -> Markup {
    let content = html! {
        div class="max-w-2xl mx-auto" {
            h1 class="text-5xl font-bold text-gray-900 mb-8" { "Welcome" }

            div class="bg-white rounded-lg shadow-md p-6" {
                h2 class="text-2xl font-semibold text-gray-900 mb-4" { "Contact Us" }

                form method="post" action=(paths::forms::CONTACT) class="space-y-4" {
                    @match current_user {
                        CurrentUser::Authenticated { .. } => {
                            div {
                                label for="email" class="block text-sm font-medium text-gray-700 mb-1" { "Email" }
                                input
                                    type="email"
                                    id="email"
                                    name="email"
                                    value=[email]
                                    readonly
                                    class="w-full px-3 py-2 border border-gray-300 rounded-md bg-gray-50 text-gray-600 cursor-not-allowed"
                                    placeholder="your@email.com";
                                @if let Some(error) = email_error {
                                    p class="text-red-600 text-sm mt-1" { (error) }
                                }
                            }
                        }
                        CurrentUser::Guest => {
                            div {
                                label for="email" class="block text-sm font-medium text-gray-700 mb-1" { "Email" }
                                input
                                    type="email"
                                    id="email"
                                    name="email"
                                    value=[email]
                                    required
                                    class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-indigo-500"
                                    placeholder="your@email.com";
                                @if let Some(error) = email_error {
                                    p class="text-red-600 text-sm mt-1" { (error) }
                                }
                            }
                        }
                    }

                    div {
                        label for="message" class="block text-sm font-medium text-gray-700 mb-1" { "Message" }
                        textarea
                            id="message"
                            name="message"
                            required
                            rows="5"
                            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-indigo-500"
                            placeholder="Your message..." { (message.unwrap_or("")) }
                        @if let Some(error) = message_error {
                            p class="text-red-600 text-sm mt-1" { (error) }
                        }
                    }

                    button
                        type="submit"
                        class="w-full bg-indigo-600 text-white py-2 px-4 rounded-md hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500"
                        { "Send Message" }
                }
            }
        }
    };

    base_layout(current_user, flash, site_name, "Home", "Home page", content)
}
