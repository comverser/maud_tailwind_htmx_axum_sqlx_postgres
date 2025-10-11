/// Macro that creates URL path constants with a base prefix.
///
/// # What it does
/// Takes a base path like "/forms" and creates constants for each route.
/// Each route gets TWO versions: relative and absolute.
///
/// # Simple Example
/// ```
/// define_nested_routes!("/api", {
///     USERS => "/users",
/// });
/// ```
///
/// This creates:
/// - `BASE` = "/api"
/// - `relative::USERS` = "/users"
/// - `USERS` = "/api/users" (BASE + relative::USERS)
///
/// # Usage
/// ```
/// // Register route handler
/// app.route(forms::SIGN_IN, post(handler))  // Uses "/forms/sign_in"
///
/// // Access just the path segment
/// let path = forms::relative::SIGN_IN;  // Just "/sign_in"
///
/// // Access the base
/// let base = forms::BASE;  // Just "/forms"
/// ```
macro_rules! define_nested_routes {
    ($base:expr, { $($name:ident => $path:expr),* $(,)? }) => {
        pub const BASE: &str = $base;

        #[allow(dead_code)]
        pub mod relative {
            $(pub const $name: &str = $path;)*
        }

        $(
            #[allow(dead_code)]
            pub const $name: &str = concat!($base, $path);
        )*
    };
}

pub mod pages {
    pub const ROOT: &str = "/";
    pub const SIGN_UP: &str = "/sign_up";
    pub const SIGN_IN: &str = "/sign_in";
    pub const ABOUT: &str = "/about";
    pub const TODOS: &str = "/todos";
}

pub mod forms {
    define_nested_routes!("/forms", {
        SIGN_UP => "/sign_up",
        SIGN_IN => "/sign_in",
        TODOS => "/todos",
        TODOS_TODO_ID_TOGGLE => "/todos/{todo_id}/toggle",
    });
}

pub mod actions {
    define_nested_routes!("/actions", {
        SIGN_OUT => "/sign_out",
        TODOS_TODO_ID => "/todos/{todo_id}",
    });
}

pub mod static_files {
    define_nested_routes!("/static", {
        FAVICON => "/img/favicon.svg",
    });
}
