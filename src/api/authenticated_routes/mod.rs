// std
use std::sync::Arc;

// crates
use axum::{middleware, Router};

// internal
use crate::{middleware::validation, AppState};

// mods
mod conversations;
mod user;

pub fn routes() -> Router<Arc<AppState>> {
	Router::new()
		.nest("/conversations", conversations::routes())
		.nest("/user", user::routes())
		.layer(tower::ServiceBuilder::new().layer(middleware::from_fn(validation::validate)))
}
