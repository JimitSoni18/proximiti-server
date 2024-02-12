mod error;
mod store;

use error::{Error, Result};
use store::new_sql_pool;
use store::SqlDb;

pub struct ModelManager {
	sql_db: SqlDb,
}

impl ModelManager {
	pub async fn new() -> Result<Self> {
		Ok(Self {
			sql_db: new_sql_pool().await?,
		})
	}

	pub(in crate::model) fn sql_db(&self) -> &SqlDb {
		&self.sql_db
	}
}
