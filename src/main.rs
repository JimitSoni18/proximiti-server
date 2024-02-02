mod config;
mod crypt;
mod ctx;
mod error;
mod model;
mod utils;
mod web;

pub mod _dev_utils;

use axum::response::Html;
pub use config::config;
pub use ctx::Ctx;
pub use error::{Error, Result};

use axum::routing::get;
use axum::{middleware, Router};
use model::ModelManager;
use tower_cookies::CookieManagerLayer;
use tracing_subscriber::EnvFilter;
use web::mw_auth::{mw_ctx_require, mw_ctx_resolve};
use web::{mw_res_map::mw_response_map, routes_login, routes_static};

#[tokio::main]
async fn main() -> Result<()> {
	tracing_subscriber::fmt()
		.without_time()
		.with_target(false)
		.with_env_filter(EnvFilter::from_default_env())
		.init();

	_dev_utils::init_dev().await;

	let mm = ModelManager::new().await?;

	let routes_hello = Router::new()
		.route("/hello", get(|| async { Html("Hello World") }))
		.route_layer(middleware::from_fn(mw_ctx_require));

	let app = Router::new()
		.merge(routes_login::routes(mm.clone()))
		.merge(routes_hello)
		.layer(middleware::map_response(mw_response_map))
		.layer(middleware::from_fn_with_state(mm.clone(), mw_ctx_resolve))
		.layer(CookieManagerLayer::new())
		.fallback_service(routes_static::serve_dir());

	let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
		.await
		.unwrap();

	axum::serve(listener, app).await.unwrap();

	Ok(())
}
