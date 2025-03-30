use axum::response::IntoResponse;
use serde::Serialize;

#[derive(Serialize)]
pub struct OkResponse<T> {
	pub status: &'static str,
	pub body: T,
}

#[derive(Serialize)]
pub struct ErrResponse {
	pub status: &'static str,
	pub message: &'static str,
}

impl<T: Serialize> OkResponse<T> {
	#[inline]
	pub fn new(body: T) -> Self {
		Self { status: "ok", body }
	}
}

impl ErrResponse {
	pub const fn new(message: &'static str) -> Self {
		Self {
			status: "error",
			message,
		}
	}
}

impl<T: Serialize> IntoResponse for OkResponse<T> {
	fn into_response(self) -> axum::response::Response {
		axum::Json(self).into_response()
	}
}
