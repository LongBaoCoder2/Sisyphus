mod error;

pub use self::error::{Error, Result};

use crate::config;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub type Db = Pool<Postgres>;

pub async fn new_db_pool() -> Result<Db> {
	PgPoolOptions::new()
		.max_connections(10)
		.connect(&config::config().DB_URL)
		.await
		.map_err(|ex| Error::FailedToCreatePool(ex.to_string()))
}
