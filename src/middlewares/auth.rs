use axum::{extract::Request, http::header, middleware::Next, response::{IntoResponse, Redirect}};
use tower_sessions::Session;

use crate::{auth::CurrentUser, flash::FlashMessage, paths};

pub async fn require_authentication(req: Request, next: Next) -> axum::response::Response {
    match req.extensions().get::<CurrentUser>() {
        Some(CurrentUser::Authenticated { .. }) => {
            let mut res = next.run(req).await;
            res.headers_mut().insert(
                header::CACHE_CONTROL,
                "no-store, no-cache, must-revalidate, private".parse().unwrap(),
            );
            res
        }
        _ => {
            let session = req.extensions().get::<Session>().cloned();
            if let Some(session) = session {
                let _ = FlashMessage::error("Please sign in to continue").set(&session).await;
            }
            Redirect::to(paths::pages::SIGN_IN).into_response()
        }
    }
}
