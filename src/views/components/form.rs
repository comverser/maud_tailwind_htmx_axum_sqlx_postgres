use maud::{Markup, html};

pub fn input(
    input_type: &str,
    name: &str,
    placeholder: &str,
    value: Option<&str>,
    error: Option<&str>,
) -> Markup {
    let input_class = if error.is_some() {
        "w-full px-3 py-2 border border-red-500 rounded-lg focus:outline-none focus:ring-2 focus:ring-red-500"
    } else {
        "w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500"
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
        button type="submit" class="w-full bg-indigo-600 text-white py-2 px-4 rounded-lg hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 font-medium" {
            (text)
        }
    }
}
