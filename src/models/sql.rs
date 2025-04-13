use crate::config::CONFIG;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub type Db = Pool<Postgres>;

pub async fn get_instance() -> Db {
	PgPoolOptions::new()
		.max_connections(5)
		.connect(&CONFIG.db_url)
		.await
		.unwrap()
}
