use std::sync::Arc;

use axum::{extract::Query, http::StatusCode, routing::get, Extension, Router};
use proximiti_server::utils::extensions::UserId;
use serde::Deserialize;

use crate::{
	services::user::{ListOnlineUser, ListUser, UserService},
	types::response::OkResponse,
	AppState,
};

mod error;

#[derive(Deserialize)]
struct SearchQuery {
	q: String,
}

pub fn routes() -> Router<Arc<AppState>> {
	Router::new()
		.route("/search", get(search))
		.route("/online", get(online))
}

type SearchResponse = error::Result<(StatusCode, OkResponse<Vec<ListUser>>)>;

async fn search(
	Extension(UserId(user_id)): Extension<UserId>,
	user_service: UserService,
	Query(SearchQuery { q }): Query<SearchQuery>,
) -> SearchResponse {
	if q.is_empty() {
		return Err(error::Error::SearchEmpty);
	}

	Ok((
		StatusCode::OK,
		OkResponse::new(
			user_service
				.search_by_username(user_id, q)
				.await
				.map_err(|_| error::Error::Internal)?,
		),
	))
}

async fn online(
	Extension(UserId(user_id)): Extension<UserId>,
	user_service: UserService,
) -> error::Result<(StatusCode, OkResponse<Vec<ListOnlineUser>>)> {
	Ok((
		StatusCode::OK,
		OkResponse::new(
			user_service
				.list_online(user_id)
				.await
				.map_err(|_| error::Error::Internal)?,
		),
	))
}
