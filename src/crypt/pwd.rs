use crate::config::config;

use super::error::{Error, Result};
use super::{encrypt_into_b64u, EncryptContent};

pub fn encrypt_pwd(content: &EncryptContent) -> Result<String> {
	let key = &config().PWD_KEY;
	let result = encrypt_into_b64u(key, content)?;

	Ok(format!("#01#{result}"))
}

pub fn validation_pwd(content: &EncryptContent, ref_pwd: &str) -> Result<()> {
	let pwd = encrypt_pwd(content)?;

	if pwd != ref_pwd {
		return Err(Error::PwdNotMatching);
	}

	Ok(())
}
