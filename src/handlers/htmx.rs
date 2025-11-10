//! HTMX response helpers for consistent interaction patterns.

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use maud::Markup;

pub fn empty_ok() -> Response {
    StatusCode::OK.into_response()
}

pub fn swap_html(markup: Markup) -> Response {
    markup.into_response()
}
