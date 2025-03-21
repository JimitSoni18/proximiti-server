// modules
mod config;
mod models;

// routes
mod api;

// middlewares
mod middleware;

// use internal
use models::Model;
mod crypt;
mod services;

// use external
use axum::Router;

// use std

pub struct AppState {
	model: Model,
}

#[tokio::main]
async fn main() {
    // to ensure that config is correctly initialized before the app starts
    config::CONFIG.check();

    tracing_subscriber::fmt::init();

	let app = Router::new()
		.nest("/api", api::router().await)
		.layer(tower_http::cors::CorsLayer::permissive());

	let listener = tokio::net::TcpListener::bind("0.0.0.0:1234").await.unwrap();

	axum::serve(listener, app).await.unwrap();
}
