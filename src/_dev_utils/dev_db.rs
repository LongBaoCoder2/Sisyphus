use std::{fs, path::PathBuf, time::Duration};

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tracing::{debug, info};

const PG_DEV_POSTGRES_URL: &str = "postgres://postgres:welcome@localhost/postgres";
const PG_DEV_APP_URL: &str = "postgres://app_user:dev_only_pwd@localhost/app_db";

const PG_DEV_RECREATE_PATH: &str = "src/sql/dev_init/00_recreated_db.sql";
const PG_DEV_INIT_DIR: &str = "src/sql/dev_init/";

type Db = Pool<Postgres>;

pub async fn init_dev_db() -> Result<(), Box<dyn std::error::Error>> {
	info!("{:<12} - init_dev_db()", "FOR-DEV-ONLY");

	// To deallocation db when go out of scope
	{
		let db = new_db_pool(&PG_DEV_POSTGRES_URL).await?;
		pexec(&db, &PG_DEV_RECREATE_PATH).await?;
	}

	let mut file_paths: Vec<PathBuf> = fs::read_dir(&PG_DEV_INIT_DIR)?
		.filter_map(|entry| entry.ok().map(|e| e.path()))
		.collect();
	// SQL command must be executed in order.
	file_paths.sort();

	let db = new_db_pool(&PG_DEV_APP_URL).await?;
	for path in file_paths {
		if let Some(path) = path.to_str() {
			let path = path.replace("\\", "/");

			if path.ends_with(".sql") && path != PG_DEV_RECREATE_PATH {
				pexec(&db, &path).await?;
			}
		}
	}

	Ok(())
}

async fn pexec(db: &Db, file_path: &str) -> Result<(), sqlx::Error> {
	let content = fs::read_to_string(file_path)?;

	let sqls: Vec<&str> = content.split(";").collect();
	for sql in sqls {
		sqlx::query(sql).execute(db).await?;
	}

	Ok(())
}

async fn new_db_pool(db_conn_url: &str) -> Result<Db, sqlx::Error> {
	PgPoolOptions::new()
		.max_connections(1)
		.acquire_timeout(Duration::from_millis(500))
		.connect(db_conn_url)
		.await
}
