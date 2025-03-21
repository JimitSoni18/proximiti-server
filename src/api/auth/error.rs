use axum::{http::StatusCode, response::IntoResponse};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, serde::Serialize)]
pub enum Error {
	PasswordIncorrect,
	SignUpUsernameTaken,
	Internal,
	BadInput,
}

impl From<crate::services::user::error::Error> for Error {
	fn from(value: crate::services::user::error::Error) -> Self {
		match value {
			crate::services::user::error::Error::UsernamePasswordError => Self::PasswordIncorrect,
			crate::services::user::error::Error::InternalError
			| crate::services::user::error::Error::UnableToCreateUser => Self::Internal,
			crate::services::user::error::Error::UsernameTaken => Self::SignUpUsernameTaken,
		}
	}
}

impl IntoResponse for Error {
	fn into_response(self) -> axum::response::Response {
		(match self {
			Self::PasswordIncorrect => (
				StatusCode::UNAUTHORIZED,
				axum::Json(serde_json::json!({
					"status": "error",
					"message": "Incorrect credentials. Please try again."
				})),
			),
			Self::Internal => (
				StatusCode::INTERNAL_SERVER_ERROR,
				axum::Json(serde_json::json!({
					"status": "error",
					"message": "There was an issue processing your request. Please try again later."
				})),
			),
			Self::SignUpUsernameTaken => (
				StatusCode::CONFLICT,
				axum::Json(serde_json::json!( {
					"status": "error",
					"message": "This username is already taken."
				})),
			),
			Self::BadInput => (
				StatusCode::BAD_REQUEST,
				axum::Json(serde_json::json!({
					"status": "error",
					"message": "Username / Password should not be empty.",
				})),
			),
		})
		.into_response()
	}
}
