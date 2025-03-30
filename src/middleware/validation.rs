use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};
use axum_extra::{
	headers::{authorization::Bearer, Authorization},
	TypedHeader,
};

use proximiti_server::utils::extensions::UserId;

use crate::crypt::token::verify_token;

#[axum::debug_middleware]
pub async fn validate(
	TypedHeader(token): TypedHeader<Authorization<Bearer>>,
	mut req: Request,
	next: Next,
) -> Result<Response, StatusCode> {
	let user_id = verify_token(token.token()).map_err(|_| StatusCode::UNAUTHORIZED)?;
	req.extensions_mut().insert(UserId(user_id));
	Ok(next.run(req).await)
}
