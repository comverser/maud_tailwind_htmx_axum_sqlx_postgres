use maud::{html, Markup};

use crate::flash::{FlashKind, FlashMessage};

pub fn flash(message: Option<&FlashMessage>) -> Markup {
    match message {
        Some(flash) => {
            let (bg_color, border_color, text_color) = match flash.kind {
                FlashKind::Success => ("bg-green-50", "border-green-400", "text-green-800"),
                FlashKind::Error => ("bg-red-50", "border-red-400", "text-red-800"),
                FlashKind::Info => ("bg-indigo-50", "border-indigo-400", "text-indigo-800"),
            };

            html! {
                div
                    id="flash-message"
                    class={"mb-4 p-4 border-l-4 rounded-lg " (bg_color) " " (border_color)}
                    _="on load wait 5s then transition opacity to 0 over 500ms then remove me"
                {
                    p class={"font-medium " (text_color)} {
                        (flash.message)
                    }
                }
            }
        }
        None => html! {},
    }
}
