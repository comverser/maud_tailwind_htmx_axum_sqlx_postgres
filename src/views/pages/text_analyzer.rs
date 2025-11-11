use crate::{auth::CurrentUser, flash::FlashMessage, paths, views::layout::base::base_layout};
use maud::{Markup, html};

pub fn text_analyzer(
    current_user: &CurrentUser,
    flash: Option<&FlashMessage>,
    site_name: &str,
) -> Markup {
    let content = html! {
        div class="max-w-lg mx-auto" {
            h1 class="text-3xl font-bold text-gray-900 mb-2" { "Text Analyzer" }
            p class="text-sm text-gray-600 mb-6" { "Upload a text file for analysis (₩1 per character)" }

            div class="bg-white rounded-lg shadow-md p-6" {
                form method="post" action=(paths::forms::TEXT_ANALYZER) enctype="multipart/form-data" class="space-y-4" {
                    div {
                        label for="file" class="block text-sm font-medium text-gray-700 mb-2" {
                            "Text File (.txt, max 10MB)"
                        }
                        input
                            type="file"
                            id="file"
                            name="file"
                            accept=".txt"
                            required
                            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-indigo-500";
                    }

                    button
                        type="submit"
                        class="w-full bg-indigo-600 text-white py-2 px-4 rounded-md hover:bg-indigo-700 font-medium"
                        { "Get Quote" }
                }
            }
        }
    };

    base_layout(current_user, flash, site_name, "Text Analyzer", "Upload files for text analysis", content)
}
