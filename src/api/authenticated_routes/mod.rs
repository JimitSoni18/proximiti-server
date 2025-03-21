// std
use std::sync::Arc;

// crates
use axum::Router;

// internal
use crate::AppState;

// mods
mod conversations;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
}
