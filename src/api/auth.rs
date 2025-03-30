use std::sync::Arc;

use crate::{
	services::user::{AuthenticatedUser, User, UserService},
	types::response::OkResponse,
};

use axum::{routing::post, Json, Router};

pub mod error;

use crate::AppState;

pub fn router() -> Router<Arc<AppState>> {
	Router::new()
		.route("/login", post(login_handler))
		.route("/sign-up", post(sign_up_handler))
}

type AuthResponse = error::Result<Json<OkResponse<AuthenticatedUser>>>;

async fn sign_up_handler(user_service: UserService, Json(user): Json<User>) -> AuthResponse {
	if user.username.is_empty() || user.password.is_empty() {
		return Err(error::Error::BadInput);
	}

	Ok(Json(OkResponse::new(
		user_service.add_user(user).await?,
	)))
}

async fn login_handler(user_service: UserService, Json(user): Json<User>) -> AuthResponse {
	if user.username.is_empty() || user.password.is_empty() {
		return Err(error::Error::BadInput);
	}
	let authenticated_user = user_service.verify_password(user).await?;

	Ok(Json(OkResponse::new(authenticated_user)))
}
