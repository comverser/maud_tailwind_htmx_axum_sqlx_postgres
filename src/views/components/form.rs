use maud::{Markup, html};

pub fn input(
    input_type: &str,
    name: &str,
    placeholder: &str,
    value: Option<&str>,
    error: Option<&str>,
) -> Markup {
    let input_class = if error.is_some() {
        "w-full px-3 py-2 border border-red-500 focus:outline-none focus:border-red-600"
    } else {
        "w-full px-3 py-2 border focus:outline-none focus:border-indigo-600"
    };

    html! {
        div {
            input type=(input_type) name=(name) required
                class=(input_class)
                placeholder=(placeholder)
                value=[value];

            @if let Some(error_msg) = error {
                p class="mt-1 text-sm text-red-600" { (error_msg) }
            }
        }
    }
}

pub fn submit_button(text: &str) -> Markup {
    html! {
        button type="submit" class="w-full bg-indigo-600 text-white px-3 py-2 hover:bg-indigo-700" {
            (text)
        }
    }
}
