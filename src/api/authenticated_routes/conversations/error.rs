use crate::types::response::ErrResponse;
use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};

pub type Result<T> = core::result::Result<T, Error>;

pub enum Error {
	Internal,
	SearchEmpty,
}

impl From<crate::services::conversations::error::Error> for Error {
	fn from(value: crate::services::conversations::error::Error) -> Self {
		match value {
			crate::services::conversations::error::Error::Internal => Self::Internal,
		}
	}
}

impl IntoResponse for Error {
	fn into_response(self) -> Response {
		match self {
			Self::Internal => (
				StatusCode::INTERNAL_SERVER_ERROR,
				Json(ErrResponse::new(
					"Something went wrong! Please try again.",
				)),
			),
			Self::SearchEmpty => (
				StatusCode::BAD_REQUEST,
				Json(ErrResponse::new(
					"Something went wrong! Please try again.",
				)),
			),
		}
		.into_response()
	}
}
