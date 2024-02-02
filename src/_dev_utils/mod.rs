use tokio::sync::OnceCell;
use tracing::info;

use crate::{
	model::{
		self,
		task::{Task, TaskBmc, TaskForCreate},
		ModelManager,
	},
	Ctx,
};

mod dev_db;

pub async fn init_dev() {
	static INSTANCE: OnceCell<()> = OnceCell::const_new();

	INSTANCE
		.get_or_init(|| async {
			info!("{:<12} - init_dev_all()", "FOR-DEV-ONLY");

			dev_db::init_dev_db().await.unwrap();
		})
		.await;
}

pub async fn init_test() -> ModelManager {
	static INSTANCE: OnceCell<ModelManager> = OnceCell::const_new();

	let mm = INSTANCE
		.get_or_init(|| async {
			init_dev().await;
			ModelManager::new().await.unwrap()
		})
		.await;

	mm.clone()
}

pub async fn seed_tasks(
	ctx: &Ctx,
	mm: &ModelManager,
	titles: &[&str],
) -> model::Result<Vec<Task>> {
	let mut tasks = Vec::new();

	for title in titles {
		let id = TaskBmc::create(
			ctx,
			mm,
			TaskForCreate {
				title: title.to_string(),
			},
		)
		.await?;

		let task = TaskBmc::get(ctx, mm, id).await?;

		tasks.push(task);
	}

	Ok(tasks)
}
