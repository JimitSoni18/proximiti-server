use axum::Router;

pub fn routes() -> Router {
	Router::new()
}

struct LoginPayload {
	username: String,
	pwd: String,
}
