use tokio::sync::OnceCell;

use tracing::{debug, info};

use crate::model::task::{Task, TaskBmc, TaskForCreate};
use crate::model::user::UserBmc;
use crate::model::{Error, Result};
use crate::{ctx::Ctx, model::ModelManager};

mod dev_db;

const PASSWORD_SAMPLE: &str = "sau";

pub async fn init_dev() {
	static INIT: OnceCell<()> = OnceCell::const_new();

	INIT.get_or_init(|| async {
		info!("{:<12} Initialize dev environment.", "FOR-DEV-ONLY");

		dev_db::init_dev_db()
			.await
			.unwrap_or_else(|ex| panic!("Cannot init dev environment! Error: {ex}"));

		let ctx = Ctx::root_ctx();
		let mm = ModelManager::new().await.unwrap();
		let user_id = 1000;

		let result = UserBmc::update_pwd(&ctx, &mm, user_id, &PASSWORD_SAMPLE).await;
		debug!("{result:?}");
	})
	.await;
}

pub async fn init_test() -> ModelManager {
	static INIT: OnceCell<ModelManager> = OnceCell::const_new();

	let mm = INIT
		.get_or_init(|| async {
			init_dev().await;

			ModelManager::new().await.unwrap()
		})
		.await;

	mm.clone()
}

pub async fn seed_test(ctx: &Ctx, mm: &ModelManager, list: &[&str]) -> Result<Vec<Task>> {
	let mut list_task = Vec::new();
	for task in list {
		let task = TaskForCreate {
			title: task.to_string(),
		};

		let id = TaskBmc::create(ctx, mm, task).await?;
		let task = TaskBmc::get(ctx, mm, id).await?;
		list_task.push(task);
	}

	Ok(list_task)
}
