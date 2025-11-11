use maud::{html, Markup};

pub fn stats_card(label: &str, value: &str) -> Markup {
    html! {
        div class="border p-4" {
            div class="text-sm text-gray-600 mb-1" { (label) }
            div class="text-2xl" { (value) }
        }
    }
}
