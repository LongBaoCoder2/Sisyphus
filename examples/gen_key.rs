use rand::RngCore;

fn main() {
	let mut key = [0u8; 64];
	rand::thread_rng().fill_bytes(&mut key);
	println!("Generated key: {key:?}");

	let encoded_str = base64_url::encode(&key);
	println!("\nEncoded string: {encoded_str}");
}
