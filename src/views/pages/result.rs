use crate::{auth::CurrentUser, flash::FlashMessage, models::order::Order, paths, views::layout::base::base_layout};
use maud::{Markup, html};

pub fn result(
    current_user: &CurrentUser,
    flash: Option<&FlashMessage>,
    site_name: &str,
    order: &Order,
) -> Markup {
    let word_count = order.text_content.split_whitespace().count();

    let content = html! {
        div class="max-w-lg mx-auto" {
            div class="bg-green-50 border border-green-200 rounded-lg p-3 mb-4" {
                p class="text-sm font-medium text-green-900" { "✓ Payment successful" }
            }

            h1 class="text-2xl font-semibold mb-4" { "Results" }

            div class="bg-white rounded-lg shadow-md p-6 space-y-4" {
                div class="grid grid-cols-2 gap-4" {
                    div class="text-center p-4 bg-gray-50 rounded-lg" {
                        p class="text-3xl font-semibold" { (order.text_length) }
                        p class="text-sm text-gray-600 mt-1" { "Characters" }
                    }
                    div class="text-center p-4 bg-gray-50 rounded-lg" {
                        p class="text-3xl font-semibold" { (word_count) }
                        p class="text-sm text-gray-600 mt-1" { "Words" }
                    }
                }

                a
                    href=(paths::pages::TEXT_ANALYZER)
                    class="block w-full bg-indigo-600 text-white py-2 px-4 rounded-lg hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 font-medium text-center"
                    { "Analyze Another File" }
            }
        }
    };

    base_layout(current_user, flash, site_name, "Results", "Text analysis results", content)
}
