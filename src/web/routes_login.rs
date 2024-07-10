use crate::crypt::{pwd, EncryptContent};
use crate::ctx::Ctx;
use crate::model::user::{UserBmc, UserForLogin};
use crate::model::ModelManager;
use crate::web::{self, Error, Result};
use axum::extract::State;
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};
use tracing::debug;

pub fn routes(mm: ModelManager) -> Router {
	Router::new()
		.route("/api/login", post(api_login_handler))
		.with_state(mm)
}

pub fn home_routes() -> Router {
	Router::new().route("/home", get(api_home_handler))
}

async fn api_home_handler() -> Result<Json<Value>> {
	let body = Json(json!({
		"body": "Home access"
	}));

	Ok(body)
}

async fn api_login_handler(
	State(mm): State<ModelManager>,
	cookies: Cookies,
	payload: Json<LoginPayload>,
) -> Result<Json<Value>> {
	debug!(" {:<12} - api_login_handler", "HANDLER");

	// TODO: Implement real db/auth logic.
	let LoginPayload {
		username,
		pwd: pwd_clear,
	} = payload.0;
	let root_ctx = Ctx::root_ctx();

	let user: UserForLogin = UserBmc::first_by_username(&root_ctx, &mm, username)
		.await?
		.ok_or(Error::LoginFailUsernameNotFound)?;

	let Some(pwd) = user.pwd else {
		return Err(Error::LoginFailPwdNotFound { id: user.id });
	};

	pwd::validation_pwd(
		&EncryptContent {
			salt: user.pwd_salt.to_string(),
			content: pwd_clear.to_string(),
		},
		pwd.as_ref(),
	)
	.map_err(|_| Error::LoginFailPwdNotMatching { id: user.id });

	// FIXME: Implement real auth-token generation/signature.
	cookies.add(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign"));

	// Create the success body.
	let body = Json(json!({
		"result": {
			"success": true
		}
	}));

	Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
	username: String,
	pwd: String,
}
