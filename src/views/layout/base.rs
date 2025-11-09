use super::navigation;
use crate::{auth::CurrentUser, config, flash::FlashMessage, paths, views::components};
use maud::{html, Markup, DOCTYPE};

pub fn base_layout(current_user: &CurrentUser, flash: &Option<FlashMessage>, title: &str, meta_description: &str, content: Markup) -> Markup {
    let site_name = config::site_name();
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="UTF-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                title { (title) " - " (site_name) }
                meta name="description" content=(meta_description);

                // Favicon
                link rel="icon" type="image/svg+xml" href=(paths::static_files::FAVICON);

                // Tailwind CSS CDN
                script src="https://cdn.jsdelivr.net/npm/@tailwindcss/browser@4" {}

                // HTMX CDN
                script src="https://cdn.jsdelivr.net/npm/htmx.org@2.0.7/dist/htmx.min.js"
                    integrity="sha384-ZBXiYtYQ6hJ2Y0ZNoYuI+Nq5MqWBr+chMrS/RkXpNzQCApHEhOt2aY8EJgqwHLkJ"
                    crossorigin="anonymous" {}

                // Hyperscript CDN
                script src="https://unpkg.com/hyperscript.org@0.9.14" {}
            }
            body class="bg-gray-50 text-gray-900 min-h-screen flex flex-col" {
                (navigation::navbar(current_user))
                main class="flex-grow container mx-auto px-4 py-8" {
                    (components::flash::flash(flash))
                    (content)
                }
            }
        }
    }
}
