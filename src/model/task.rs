use serde::{Deserialize, Serialize};
use sqlb::Fields;
use sqlx::prelude::FromRow;

use crate::ctx::Ctx;
use crate::model::error::{Error, Result};

use super::common::DbBmc;
use super::{common, ModelManager};

// Model: Task struct
#[derive(Clone, Debug, Serialize, FromRow, Fields)]
pub struct Task {
	pub id: i64,
	pub title: String,
}

// Task entity for creating method
#[derive(Deserialize, Fields)]
pub struct TaskForCreate {
	pub title: String,
}

// Task entity for updating method
#[derive(Deserialize, Fields)]
pub struct TaskForUpdate {
	pub title: Option<String>,
}

// Task Backend Model Controller
pub struct TaskBmc;

impl TaskBmc {
	// i64: id of entity
	pub async fn create(ctx: &Ctx, mm: &ModelManager, task_c: TaskForCreate) -> Result<i64> {
		common::create::<Self, TaskForCreate>(ctx, mm, task_c).await
	}

	pub async fn update(
		ctx: &Ctx,
		mm: &ModelManager,
		id: i64,
		task_u: TaskForUpdate,
	) -> Result<()> {
		common::update::<Self, TaskForUpdate>(ctx, mm, id, task_u).await
	}

	pub async fn list(ctx: &Ctx, mm: &ModelManager) -> Result<Vec<Task>> {
		common::list::<Self, _>(ctx, mm).await
	}

	pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Task> {
		common::get::<Self, _>(ctx, mm, id).await
	}

	pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
		common::delete::<Self, Task>(ctx, mm, id).await
	}
}

// Impl Trait Dbmc for Task model
impl DbBmc for TaskBmc {
	const TABLE: &'static str = "task";
}

#[cfg(test)]
mod tests {
	#![allow(unused)]
	use crate::_dev_utils;

	use super::*;
	use anyhow::Result;
	use serial_test::serial;

	#[serial]
	#[tokio::test]
	async fn test_create_ok() -> Result<()> {
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let fx_title = "test_create_ok title";
		let task_c = TaskForCreate {
			title: fx_title.to_string(),
		};

		let id = TaskBmc::create(&ctx, &mm, task_c).await.unwrap();

		let task = TaskBmc::get(&ctx, &mm, id).await.unwrap();
		assert_eq!(task.title, fx_title);

		TaskBmc::delete(&ctx, &mm, id).await.unwrap();

		Ok(())
	}

	#[serial]
	#[tokio::test]
	async fn test_get_err_not_found() -> Result<()> {
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let id = 100;

		let result = TaskBmc::get(&ctx, &mm, id).await;
		assert!(matches!(
			result,
			Err(Error::EntityNotFound { entity: "task", id })
		));

		Ok(())
	}

	#[serial]
	#[tokio::test]
	async fn test_delete_err_not_found() -> Result<()> {
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let id = 100;

		let result = TaskBmc::delete(&ctx, &mm, id).await;
		assert!(matches!(
			result,
			Err(Error::EntityNotFound { entity: "task", id })
		));

		Ok(())
	}

	#[serial]
	#[tokio::test]
	async fn test_list_ok() -> Result<()> {
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let list = ["Sau 1", "Sau 2"];
		let list_task = _dev_utils::seed_test(&ctx, &mm, &list).await?;

		let result = TaskBmc::list(&ctx, &mm).await?;
		assert!(matches!(list_task, result));

		Ok(())
	}

	#[serial]
	#[tokio::test]
	async fn test_update_err_not_found() -> Result<()> {
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let id = 100;
		let task_u = TaskForUpdate {
			title: Some("Sau".to_string()),
		};

		let result = TaskBmc::update(&ctx, &mm, id, task_u).await;
		assert!(matches!(
			result,
			Err(Error::EntityNotFound { entity: "task", id })
		));

		Ok(())
	}

	#[serial]
	#[tokio::test]
	async fn test_update_ok() -> Result<()> {
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let task_c = TaskForCreate {
			title: "Sau".to_string(),
		};
		let id = TaskBmc::create(&ctx, &mm, task_c).await?;

		let after_title = "Sau after edited";
		let task_u = TaskForUpdate {
			title: Some(after_title.to_string()),
		};
		TaskBmc::update(&ctx, &mm, id, task_u).await?;

		let task = TaskBmc::get(&ctx, &mm, id).await?;
		assert_eq!(after_title, task.title);

		Ok(())
	}
}
