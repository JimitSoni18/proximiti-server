use std::sync::Arc;

use axum::Router;

use crate::{models::Model, AppState};

pub mod auth;

pub mod authenticated_routes;

pub async fn router() -> Router {
	Router::new()
		.nest("/auth", auth::router())
		.with_state(Arc::new(AppState {
			model: Model::new().await,
		}))
}
