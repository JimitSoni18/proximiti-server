use std::{fs, path::PathBuf, time::Duration};

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tracing::info;

use crate::{
	model::{
		user::{User, UserBmc},
		ModelManager,
	},
	Ctx,
};

type Db = Pool<Postgres>;

// -- NOTE: Hardcode to prevent deployed system db update;
const PG_DEV_POSTGRES_URL: &str =
	"postgres://postgres:welcome@localhost/postgres";
const PG_DEV_APP_URL: &str =
	"postgres://app_user:dev_only_pwd@localhost/app_db";

// sql files
const SQL_RECREATE_DB: &str = "sql/dev_initial/00-recreate-db.sql";
const SQL_DIR: &str = "sql/dev_initial";

const DEMO_PWD: &str = "welcome";

pub async fn init_dev_db() -> Result<(), Box<dyn std::error::Error>> {
	info!("{:<12} - init_dev_db()", "FOR-DEV-ONLY");

	// -- Create the app_db/app_user with postgres user.

	let root_db = new_db_pool(PG_DEV_POSTGRES_URL).await?;
	pexec(&root_db, SQL_RECREATE_DB).await?;
	drop(root_db);

	let app_db = new_db_pool(PG_DEV_APP_URL).await?;

	let mut paths: Vec<PathBuf> = fs::read_dir(SQL_DIR)?
		.filter_map(|entry| entry.ok().map(|e| e.path()))
		.collect();

	for path in &mut paths {
		if let Some(path) = path.to_str() {
			let path = path.replace("\\", "/");

			if path.ends_with(".sql") && path != SQL_RECREATE_DB {
				pexec(&app_db, &path).await?;
			}
		}
	}

	// -- Init model layer
	let mm = ModelManager::new().await?;
	let ctx = Ctx::root_ctx();

	// -- Set demo1 pwd
	let demo1_user: User = UserBmc::first_by_username(&ctx, &mm, "demo1")
		.await?
		.unwrap();

	UserBmc::update_pwd(&ctx, &mm, demo1_user.id, DEMO_PWD).await?;
	info!("{:<12} - init_dev_db ", "FOR-DEV-ONLY");

	Ok(())
}

async fn pexec(db: &Db, file: &str) -> Result<(), sqlx::Error> {
	info!("{:<12} - pexec: {file}", "FOR-DEV-ONLY");

	let content = fs::read_to_string(file)?;

	let queries = content.split(';');

	for query in queries {
		sqlx::query(query).execute(db).await?;
	}

	Ok(())
}

async fn new_db_pool(db_con_url: &str) -> Result<Db, sqlx::Error> {
	PgPoolOptions::new()
		.max_connections(1)
		.acquire_timeout(Duration::from_millis(500))
		.connect(db_con_url)
		.await
}
