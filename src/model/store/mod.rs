mod error;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::time::Duration;

use crate::config;
pub use error::{Error, Result};

pub type SqlDb = Pool<Postgres>;

pub async fn new_sql_pool() -> Result<SqlDb> {
	PgPoolOptions::new()
		.max_connections(20)
		.acquire_timeout(Duration::from_millis(200))
		.connect(&config().DB_URL)
		.await
		.map_err(|e| Error::FailedToCreatePool(e.to_string()))
}
