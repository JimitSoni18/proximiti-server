mod config;
mod ctx;
mod model;
mod web;

mod _dev_utils;

use axum::Router;
use tower_http::services::ServeDir;

pub use config::config;

#[tokio::main]
async fn main() {
	let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
		.await
		.unwrap();

	let app =
		Router::new().fallback_service(ServeDir::new(&config().STATIC_DIR));

	axum::serve(listener, app.into_make_service())
		.await
		.unwrap();
}
