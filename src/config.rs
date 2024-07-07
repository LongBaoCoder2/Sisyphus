use crate::error::{Error, Result};
use std::{env, str::FromStr, sync::OnceLock};

pub fn config() -> &'static Config {
	static INSTANCE: OnceLock<Config> = OnceLock::new();
	INSTANCE.get_or_init(|| {
		Config::load_from_env().unwrap_or_else(|ex| {
			panic!(
				"   MISSING ENV -- CANNOT LOAD CONFIGURATION OF APPLICATION. Error: {}",
				ex
			)
		})
	})
}

#[allow(non_snake_case)]
pub struct Config {
	// Key token
	pub PWD_KEY: Vec<u8>,

	pub TOKEN_KEY: Vec<u8>,

	pub TOKEN_DURATION: f64,

	// DB
	pub DB_URL: String,

	// html folder
	pub WEB_FOLDER: String,
}

impl Config {
	pub fn load_from_env() -> Result<Config> {
		Ok(Config {
			PWD_KEY: get_env_b64u_into_u8s("SERVICE_PWD_KEY").unwrap(),
			TOKEN_KEY: get_env_b64u_into_u8s("SERVICE_TOKEN_KEY").unwrap(),
			TOKEN_DURATION: get_from_parse("SERVICE_TOKEN_DURATION").unwrap(),
			DB_URL: get_env("SERVICE_DB_URL").unwrap(),
			WEB_FOLDER: get_env("SERVICE_WEB_FOLDER").unwrap(),
		})
	}
}

fn get_from_parse<T: FromStr>(name: &'static str) -> Result<T> {
	get_env(name)?
		.as_str()
		.parse::<T>()
		.map_err(|_| Error::ParseError(name))
}

fn get_env_b64u_into_u8s(name: &'static str) -> Result<Vec<u8>> {
	base64_url::decode(&get_env(name)?.as_bytes()).map_err(|_| Error::FailedToDecodeBase64Url(name))
}

fn get_env(name: &'static str) -> Result<String> {
	env::var(name).map_err(|_| Error::ConfigMissingError(name))
}
