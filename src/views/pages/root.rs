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
        div class="max-w-lg mx-auto" {
            h1 class="text-xl mb-3" { "Contact" }

            form method="post" action=(paths::forms::CONTACT) class="space-y-3" {
                    @match current_user {
                        CurrentUser::Authenticated { .. } => {
                            div {
                                label for="email" class="block text-sm mb-1" { "Email" }
                                input
                                    type="email"
                                    id="email"
                                    name="email"
                                    value=[email]
                                    readonly
                                    class="w-full px-3 py-2 border bg-gray-50 cursor-not-allowed"
                                    placeholder="your@email.com";
                                @if let Some(error) = email_error {
                                    p class="text-red-600 text-sm mt-1" { (error) }
                                }
                            }
                        }
                        CurrentUser::Guest => {
                            div {
                                label for="email" class="block text-sm mb-1" { "Email" }
                                input
                                    type="email"
                                    id="email"
                                    name="email"
                                    value=[email]
                                    required
                                    class="w-full px-3 py-2 border focus:outline-none focus:border-indigo-600"
                                    placeholder="your@email.com";
                                @if let Some(error) = email_error {
                                    p class="text-red-600 text-sm mt-1" { (error) }
                                }
                            }
                        }
                    }

                    div {
                        label for="message" class="block text-sm mb-1" { "Message" }
                        textarea
                            id="message"
                            name="message"
                            required
                            rows="5"
                            class="w-full px-3 py-2 border focus:outline-none focus:border-indigo-600"
                            placeholder="Your message..." { (message.unwrap_or("")) }
                        @if let Some(error) = message_error {
                            p class="text-red-600 text-sm mt-1" { (error) }
                        }
                    }

                    button
                        type="submit"
                        class="w-full bg-indigo-600 text-white px-3 py-2 hover:bg-indigo-700"
                        { "Send Message" }
                }
        }
    };

    base_layout(current_user, flash, site_name, "Home", "Home page", content)
}
