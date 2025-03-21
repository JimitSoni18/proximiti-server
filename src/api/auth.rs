use std::sync::Arc;
use std::time::Instant;

use crate::services::user::AuthenticatedUser;
use crate::services::user::User;
use crate::services::user::UserService;

use axum::{extract::State, routing::post, Json, Router};
use serde::Serialize;
use tracing::info;

pub mod error;

use crate::AppState;

pub fn router() -> Router<Arc<AppState>> {
	Router::new()
		.route("/login", post(login_handler))
		.route("/sign-up", post(sign_up_handler))
}

#[derive(Serialize)]
struct SignInResponse {
	status: &'static str,
	user: AuthenticatedUser,
}

impl From<AuthenticatedUser> for SignInResponse {
	fn from(user: AuthenticatedUser) -> Self {
		Self { status: "ok", user }
	}
}

#[tracing::instrument(skip_all)]
async fn sign_up_handler(
	model: State<Arc<AppState>>,
	Json(user): Json<User>,
) -> error::Result<Json<SignInResponse>> {
	if user.username.is_empty() || user.password.is_empty() {
		return Err(error::Error::BadInput);
	}
	let start = Instant::now();
	let authenticated_user = UserService::add_user(model.model.db(), user).await?.into();

	info!("sign_up executed in {:?}", start.elapsed());
	Ok(Json(authenticated_user))
}

async fn login_handler(
	model: State<Arc<AppState>>,
	Json(user): Json<User>,
) -> error::Result<Json<SignInResponse>> {
	if user.username.is_empty() || user.password.is_empty() {
		return Err(error::Error::BadInput);
	}
	let authenticated_user = UserService::verify_password(model.model.db(), user)
		.await?
		.into();

	Ok(Json(authenticated_user))
}
