use super::{Error, Result};
use crate::crypt::{pwd, EncryptContent};
use crate::ctx::Ctx;
use serde::{Deserialize, Serialize};
use sqlb::{Fields, HasFields};
use sqlx::{postgres::PgRow, prelude::FromRow};
use tracing::debug;
use uuid::Uuid;

use super::{
	common::{self, DbBmc},
	ModelManager,
};

#[derive(Debug, Clone, Serialize, FromRow, Fields)]
pub struct User {
	pub id: i64,
	pub username: String,
}

// Entity for API Call
// (e.g., UserBmc::create argument)
#[derive(Deserialize)]
pub struct UserForCreate {
	pub username: String,
	pub pwd: String,
}

// Entity for insert method in this modules
// (e.g., inside UserBmc::create fn)
#[derive(Fields)]
struct UserForInsert {
	username: String,
}

#[derive(FromRow, Fields, Debug)]
pub struct UserForLogin {
	pub id: i64,
	pub username: String,

	pub pwd: Option<String>,
	pub pwd_salt: Uuid,
	pub token_salt: Uuid,
}

#[derive(Debug, FromRow, Fields)]
pub struct UserForAuth {
	pub id: i64,
	pub username: String,

	// -- token info
	pub token_salt: Uuid,
}

pub trait UserBy: for<'r> FromRow<'r, PgRow> + Unpin + Send + HasFields {}

impl UserBy for User {}
impl UserBy for UserForLogin {}
impl UserBy for UserForAuth {}

pub struct UserBmc;

impl DbBmc for UserBmc {
	const TABLE: &'static str = "user";
}

impl UserBmc {
	pub async fn get<E>(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<E>
	where
		E: UserBy,
	{
		common::get::<Self, _>(ctx, mm, id).await
	}

	pub async fn first_by_username<E>(
		ctx: &Ctx,
		mm: &ModelManager,
		username: String,
	) -> Result<Option<E>>
	where
		E: UserBy,
	{
		let user = sqlb::select()
			.table(Self::TABLE)
			.and_where("username", "=", username)
			.fetch_optional(mm.db())
			.await?;
		Ok(user)
	}

	pub async fn update_pwd(ctx: &Ctx, mm: &ModelManager, id: i64, raw_pwd: &str) -> Result<()> {
		debug!("{raw_pwd}");
		let user: UserForLogin = Self::get(ctx, mm, id).await?;
		let ec_content = EncryptContent {
			content: raw_pwd.to_string(),
			salt: user.token_salt.to_string(),
		};

		let pwd = pwd::encrypt_pwd(&ec_content)?;
		debug!("{pwd}");

		let row_effected = sqlb::update()
			.table(Self::TABLE)
			.and_where("id", "=", id)
			.data(vec![("pwd", pwd.to_string()).into()])
			.exec(mm.db())
			.await?;

		if row_effected == 0 {
			return Err(Error::EntityNotFound {
				entity: Self::TABLE,
				id,
			});
		}
		Ok(())
	}
}
