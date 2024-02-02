use axum::{http::StatusCode, response::IntoResponse};
use tracing::debug;

use crate::{model, web};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
	// -- Config
	ConfigMissingEnv(&'static str),
	ConfigWrongFormat(&'static str),

	// -- Model
	Model(model::Error),

	// -- Auth
	AuthFailTokenNotFound,

	// -- CtxExtError
	CtxExt(web::mw_auth::CtxExtError),
}

impl From<model::Error> for Error {
	fn from(value: model::Error) -> Self {
		Self::Model(value)
	}
}

impl IntoResponse for Error {
	fn into_response(self) -> axum::response::Response {
		debug!("{:<12} - {self:?}", "INTO_RES");

		(StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR")
			.into_response()
	}
}
