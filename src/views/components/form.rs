use maud::{Markup, html};

pub fn input(
    input_type: &str,
    name: &str,
    placeholder: &str,
    value: Option<&str>,
    error: Option<&str>,
) -> Markup {
    let input_class = if error.is_some() {
        "w-full px-3 py-2 border-2 border-red-500 rounded focus:outline-none focus:border-red-600"
    } else {
        "w-full px-3 py-2 border rounded focus:outline-none focus:border-blue-500"
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
        button type="submit" class="w-full bg-blue-600 text-white py-2 rounded hover:bg-blue-700" {
            (text)
        }
    }
}
