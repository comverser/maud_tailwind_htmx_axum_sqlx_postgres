/// Macro that creates URL path constants with a base prefix.
///
/// Generates both absolute paths (prefixed) and relative paths (unprefixed)
/// for each route, providing flexibility in route composition and registration.
macro_rules! define_nested_routes {
    ($base:expr, { $($name:ident => $path:expr),* $(,)? }) => {
        pub const BASE: &str = $base;

        // Allow unused relative paths - these provide flexibility for future use
        // and path composition without requiring all paths to be used
        #[allow(dead_code)]
        pub mod relative {
            $(pub const $name: &str = $path;)*
        }

        // Allow unused absolute paths - not all defined routes may be used yet
        // as this is a template where routes are added as needed
        $(
            #[allow(dead_code)]
            pub const $name: &str = concat!($base, $path);
        )*
    };
}

pub mod pages {
    pub const ROOT: &str = "/";
    pub const SIGN_IN: &str = "/sign_in";
    pub const TODOS: &str = "/todos";
    pub const TEXT_ANALYZER: &str = "/text_analyzer";
    pub const QUOTE: &str = "/quote/{order_id}";
    pub const CHECKOUT: &str = "/checkout/{order_id}";
    pub const RESULT: &str = "/result/{order_id}";
}

pub mod forms {
    define_nested_routes!("/forms", {
        SIGN_IN => "/sign_in",
        TODOS => "/todos",
        CONTACT => "/contact",
        TEXT_ANALYZER => "/text_analyzer",
    });
}

pub mod actions {
    define_nested_routes!("/actions", {
        SIGN_OUT => "/sign_out",
        VERIFY_MAGIC_LINK => "/auth/verify",
        TODOS_TODO_ID => "/todos/{todo_id}",
        TODOS_TODO_ID_TOGGLE => "/todos/{todo_id}/toggle",
        PAYMENT_INITIATE => "/payment/initiate",
        PAYMENT_VERIFY => "/payment/verify",
    });
}

pub mod static_files {
    define_nested_routes!("/static", {
        FAVICON => "/img/favicon.svg",
    });
}

pub fn with_param(path: &str, param_name: &str, value: &impl ToString) -> String {
    path.replace(&format!("{{{}}}", param_name), &value.to_string())
}
