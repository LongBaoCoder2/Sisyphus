// Region: Module
mod error;
pub mod pwd;

pub use self::error::{Error, Result};
use hmac::{Hmac, Mac};
use sha2::Sha512;
use tracing::debug;

pub struct EncryptContent {
	pub content: String,
	pub salt: String,
}

pub fn encrypt_into_b64u(key: &[u8], encrypt_content: &EncryptContent) -> Result<String> {
	let EncryptContent { content, salt } = encrypt_content;

	// Create hmac from Key
	let mut hmac_sha512 = Hmac::<Sha512>::new_from_slice(key).map_err(|_| Error::KeyFailedHmac)?;

	// Update content
	hmac_sha512.update(content.as_bytes());
	hmac_sha512.update(salt.as_bytes());

	let hmac_result = hmac_sha512.finalize();
	let hmac_bytes = hmac_result.into_bytes();

	let result = base64_url::encode(&hmac_bytes);

	Ok(result)
}

#[cfg(test)]
mod crypt_test {
	use super::*;
	use rand::RngCore;

	#[test]
	fn test_encrypt_into_b64u_ok() -> Result<()> {
		let mut key = [0u8; 100];
		rand::thread_rng().fill_bytes(&mut key);
		let encrypt_content = EncryptContent {
			content: "test".to_string(),
			salt: "Sau".to_string(),
		};

		let fx_enc = encrypt_into_b64u(&key, &encrypt_content)?;
		let enc = encrypt_into_b64u(&key, &encrypt_content)?;

		assert_eq!(fx_enc, enc);

		Ok(())
	}
}
