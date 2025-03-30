use axum::{http::StatusCode, response::IntoResponse, Json};

use crate::types::response::ErrResponse;

pub type Result<T> = core::result::Result<T, Error>;

pub enum Error {
	SearchEmpty,
	Internal,
}

impl IntoResponse for Error {
	fn into_response(self) -> axum::response::Response {
		(match self {
			Self::SearchEmpty => (
				StatusCode::BAD_REQUEST,
				Json(ErrResponse::new("Search query should not be empty")),
			),
			Self::Internal => (
				StatusCode::INTERNAL_SERVER_ERROR,
				Json(ErrResponse::new("Something went wrong")),
			),
		})
		.into_response()
	}
}
