use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
	let hc = httpc_test::new_client("http://localhost:8080")?;

	hc.do_get("/index.html").await?.print().await?;

	hc.do_get("/home").await?.print().await?;

	Ok(())
}
