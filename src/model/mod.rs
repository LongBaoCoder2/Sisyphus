//! - The Model Layer normalizes the application's data type
//! 	structure and access.
//! - All application code data access must go through the model layer.
//! - The `ModelManager` holds the internal states/resources needed
//! 	by `ModelController` to access data.
//! 	(e.g., db_pool, S3 client, redis client).
//! - ModelController (e.g. `TaskBmc`, `ProjectBmc`) implement CRUD
//! 	and other data access methods on a given "entity"
//! 	(e.g., `Task`, `Project`)
//! 	(Bmc stands for Backend Model Controller)
//! - In frameworks like Axum, Tauri, `ModelManager`s are typiclly used as
//! 	App State
//! - `ModelManager` are designed to pass as an argument
//! 	to all `ModelController` functions.
//!  - So that we can protect the visibility of those resources only to the modern layer.

// region:    --- Modules
mod common;
mod error;
mod store;

pub mod task;
pub mod user;

use store::{new_db_pool, Db};

pub use self::error::{Error, Result};

// endregion: --- Modules

#[derive(Clone)]
pub struct ModelManager {
	db: Db,
}

impl ModelManager {
	pub async fn new() -> Result<Self> {
		let db = new_db_pool().await?;
		Ok(ModelManager { db })
	}

	// Return a reference to db pool connection
	// that only visible for model crate
	pub(in crate::model) fn db(&self) -> &Db {
		&self.db
	}
}
