// std
use std::sync::Arc;

// crates
use axum::{extract::State, response::IntoResponse, routing::get, Extension, Router};
use proximiti_server::utils::extensions::UserId;

// internal
use crate::AppState;

pub fn routes() -> Router<Arc<AppState>> {
	Router::new()
        .route("/list", get(list))
}

async fn list(
	Extension(UserId(user_id)): Extension<UserId>,
	state: State<Arc<AppState>>,
) -> impl IntoResponse {
}
