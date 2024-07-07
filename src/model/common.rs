use sqlb::HasFields;
use sqlx::{postgres::PgRow, FromRow, PgExecutor, Row};

use crate::ctx::Ctx;

use super::ModelManager;

use crate::model::{Error, Result};

pub trait DbBmc {
	const TABLE: &'static str;
}

pub async fn get<MC, E>(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<E>
where
	MC: DbBmc,
	E: for<'r> FromRow<'r, PgRow> + Unpin + Send,
	E: HasFields,
{
	let res = sqlb::select()
		.table(MC::TABLE)
		.and_where("id", "=", id)
		.columns(E::field_names())
		.fetch_optional(mm.db())
		.await?
		.ok_or(Error::EntityNotFound {
			entity: MC::TABLE,
			id,
		})?;

	Ok(res)
}

pub async fn list<MC, E>(_ctx: &Ctx, mm: &ModelManager) -> Result<Vec<E>>
where
	MC: DbBmc,
	E: for<'r> FromRow<'r, PgRow> + Unpin + Send,
	E: HasFields,
{
	let res = sqlb::select()
		.table(MC::TABLE)
		.columns(E::field_names())
		.order_by("id")
		.fetch_all(mm.db())
		.await?;

	Ok(res)
}

pub async fn create<MC, E>(_ctx: &Ctx, mm: &ModelManager, data: E) -> Result<i64>
where
	MC: DbBmc,
	E: HasFields,
{
	let fields = data.not_none_fields();

	let (id,) = sqlb::insert()
		.table(MC::TABLE)
		.data(fields)
		.returning(&["id"])
		.fetch_one::<_, (i64,)>(mm.db())
		.await?;

	Ok(id)
}

pub async fn delete<MC, E>(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()>
where
	MC: DbBmc,
	E: HasFields,
{
	let row_effected = sqlb::delete()
		.table(MC::TABLE)
		.and_where("id", "=", id)
		.exec(mm.db())
		.await?;

	if row_effected == 0 {
		return Err(Error::EntityNotFound {
			entity: MC::TABLE,
			id,
		});
	}

	Ok(())
}

pub async fn update<MC, E>(_ctx: &Ctx, mm: &ModelManager, id: i64, data: E) -> Result<()>
where
	MC: DbBmc,
	E: HasFields,
{
	let fields = data.not_none_fields();

	let row_effected = sqlb::update()
		.table(MC::TABLE)
		.data(fields)
		.and_where("id", "=", id)
		.exec(mm.db())
		.await?;

	if row_effected == 0 {
		return Err(Error::EntityNotFound {
			entity: MC::TABLE,
			id,
		});
	}

	Ok(())
}
