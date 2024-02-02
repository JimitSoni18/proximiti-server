//! Model Layer
//!
//! Design:
//!
//! - The Model layer normalizes the application's data type
//!   structures and access.
//! - All application code data access must go through Model layer.
//! - The `ModelManager` holds the internal state/resources
//!   needed by ModelControllers to access data.
//!   (e.g., db_pool, S3 client, redis client).
//! - Model Controllers (e.g., `TaskBmc`, `ProjectBmc`) implement
//!   CRUD and other data access methods on a given "entity"
//!   (e.g., `Task`, `Project`).
//!   (`Bmc` is short for Backend Model Controller).
//! - Frameworks like Axum, Tauri, `ModelManager` are typically used as App State.
//!   ModelManager are designed to be passed as an argument
//!   to all Model Controller functions.

mod base;
mod error;
mod store;
pub mod task;
pub mod user;

pub use error::{Error, Result};
use store::{new_db_pool, Db};

#[derive(Clone)]
pub struct ModelManager {
	db: Db,
}

impl ModelManager {
	/// Constructor
	pub async fn new() -> Result<Self> {
		Ok(Self {
			db: new_db_pool().await?,
		})
	}

	pub(in crate::model) fn db(&self) -> &Db {
		&self.db
	}
}
