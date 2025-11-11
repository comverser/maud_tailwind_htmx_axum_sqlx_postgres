use crate::{auth::CurrentUser, flash::FlashMessage, models::order::Order, paths, views::layout::base::base_layout};
use maud::{Markup, html, PreEscaped};

pub fn result(
    current_user: &CurrentUser,
    flash: Option<&FlashMessage>,
    site_name: &str,
    order: &Order,
) -> Markup {
    let word_count = order.text_content.split_whitespace().count();
    let line_count = order.text_content.lines().count();
    let avg_word_length = if word_count > 0 {
        order.text_length as f64 / word_count as f64
    } else {
        0.0
    };

    let content = html! {
        div class="max-w-2xl mx-auto" {
            div class="bg-green-50 border border-green-200 rounded-lg p-3 mb-6" {
                p class="text-sm font-medium text-green-900" { "✓ Payment successful" }
            }

            h1 class="text-3xl font-bold text-gray-900 mb-6" { "Results" }

            div class="bg-white rounded-lg shadow-md p-6 space-y-6" {
                div {
                    h2 class="text-sm font-semibold text-gray-500 uppercase mb-3" { "Statistics" }
                    div class="grid grid-cols-2 gap-4" {
                        div class="text-center p-3 bg-gray-50 rounded" {
                            p class="text-2xl font-bold text-gray-900" { (order.text_length) }
                            p class="text-xs text-gray-600" { "Characters" }
                        }
                        div class="text-center p-3 bg-gray-50 rounded" {
                            p class="text-2xl font-bold text-gray-900" { (word_count) }
                            p class="text-xs text-gray-600" { "Words" }
                        }
                        div class="text-center p-3 bg-gray-50 rounded" {
                            p class="text-2xl font-bold text-gray-900" { (line_count) }
                            p class="text-xs text-gray-600" { "Lines" }
                        }
                        div class="text-center p-3 bg-gray-50 rounded" {
                            p class="text-2xl font-bold text-gray-900" { (format!("{:.1}", avg_word_length)) }
                            p class="text-xs text-gray-600" { "Avg Word Len" }
                        }
                    }
                }

                div {
                    h2 class="text-sm font-semibold text-gray-500 uppercase mb-2" { "Content" }
                    div class="bg-gray-50 rounded p-4 max-h-64 overflow-y-auto" {
                        pre class="text-sm text-gray-800 whitespace-pre-wrap" {
                            (PreEscaped(html_escape(&order.text_content)))
                        }
                    }
                }

                a
                    href=(paths::pages::TEXT_ANALYZER)
                    class="block w-full bg-indigo-600 text-white py-2 px-4 rounded hover:bg-indigo-700 font-medium text-center"
                    { "Analyze Another File" }
            }
        }
    };

    base_layout(current_user, flash, site_name, "Results", "Text analysis results", content)
}

fn html_escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
