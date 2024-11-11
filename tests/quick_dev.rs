#![allow(unused)]
use anyhow::Result;

#[tokio::test]
async fn quick_dev() -> Result<()> {
  let hc = httpc_test::new_client("http://localhost:8080")?;
  hc.do_get("/hello").await?.print().await?;
  // let hc = httpc_test::new_client();
  // let resp = hc.get("http://localhost:8080/hello").send().await.unwrap();
  Ok(())
}
