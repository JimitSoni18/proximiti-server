pub mod cache;
pub mod sql;
pub mod s3;

pub struct Model {
	// TODO: add s3, cache and search
	sql: sql::Db,
}

impl Model {
	pub async fn new() -> Self {
		Self {
			sql: sql::get_instance().await,
		}
	}

	pub fn db(&self) -> sql::Db {
		self.sql.clone()
	}
}
