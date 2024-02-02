use axum::{http::StatusCode, response::IntoResponse};
use std::sync::Arc;
use tracing::debug;

use crate::{crypt, model};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
	// -- Login
	LoginFailUsernameNotFound,
	LoginFailUserHasNoPwd { user_id: i64 },
	LoginFailPwdNotMatching { user_id: i64 },

	// -- Auth
	// CtxExt(_),

	// -- Modules
	Model(model::Error),
	Crypt(crypt::Error),
}

#[allow(non_camel_case_types)]
enum ClientError {
	LOGIN_FAIL,
	NO_AUTH,
	SERVICE_ERROR,
	INVALID_PARAMS,
}

impl IntoResponse for Error {
	fn into_response(self) -> axum::response::Response {
		debug!("{:<12} - model::Error {self:?}", "INTO_RES");

		// Create a placeholder Axum response.
		let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

		// Insert the Error into the response.
		response.extensions_mut().insert(Arc::new(self));

		response
	}
}

impl From<crypt::Error> for Error {
	fn from(value: crypt::Error) -> Self {
		Self::Crypt(value)
	}
}

impl From<model::Error> for Error {
	fn from(value: model::Error) -> Self {
		Self::Model(value)
	}
}

// region:   --- Client Error
impl Error {
	pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
		use Error::*;

		#[allow(unreachable_code)]
		match self {
			// CtxExt(_) => (StatusCode::FORBIDDEN, ClientError::)

			// -- Login
			LoginFailUsernameNotFound
			| LoginFailUserHasNoPwd { .. }
			| LoginFailPwdNotMatching { .. } => {
				(StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL)
			}

			// CtxExt(_) => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),
			_ => (
				StatusCode::INTERNAL_SERVER_ERROR,
				ClientError::SERVICE_ERROR,
			),
		}
	}
}
// endregion:   --- Client Error
