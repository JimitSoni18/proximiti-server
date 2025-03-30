// std
use std::sync::Arc;

// crates
use axum::{routing::get, Extension, Json, Router};
use proximiti_server::utils::extensions::UserId;

// internal
use crate::{
	services::conversations::{Conversation, ConversationService},
	types::response::OkResponse,
	AppState,
};

// mods
mod error;

pub fn routes() -> Router<Arc<AppState>> {
	Router::new().route("/list", get(list))
}

async fn list(
	Extension(UserId(user_id)): Extension<UserId>,
	conv_service: ConversationService,
) -> error::Result<Json<OkResponse<Vec<Conversation>>>> {
	Ok(Json(OkResponse::new(conv_service.list(user_id).await?)))
}
