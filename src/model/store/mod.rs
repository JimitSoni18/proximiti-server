mod error;

use crate::config;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub use error::{Error, Result};

pub type Db = Pool<Postgres>;

pub async fn new_db_pool() -> Result<Db> {
	match PgPoolOptions::new()
		.max_connections(1)
		.connect(&config().DB_URL)
		.await
	{
		Ok(pool) => Ok(pool),
		Err(ex) => Err(Error::FailedToCreatePool(ex.to_string())),
	}
}
