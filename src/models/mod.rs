pub mod cache;
pub mod relational;
pub mod s3;

pub struct Model {
	// TODO: add s3, cache and search
	sql: relational::Db,
}

impl Model {
	pub async fn new() -> Self {
		Self {
			sql: relational::get_instance().await,
		}
	}

	pub fn db(&self) -> &relational::Db {
		&self.sql
	}
}
