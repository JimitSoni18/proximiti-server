use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

const DB_URL: &str = "postgres://postgres:welcome@localhost:5432/proximiti";

pub type Db = Pool<Postgres>;

pub async fn get_instance() -> Db {
	PgPoolOptions::new()
		.max_connections(5)
		.connect(DB_URL)
		.await
		.unwrap()
}
