#![allow(unused)]

use anyhow::Result;
use httpc_test::{Response}; // Import the ResponseExt trait

#[tokio::test]
async fn dev() -> Result<()>{
    let hc = httpc_test::new_client("http://localhost:8000")?;
    hc.do_get("/hello").await.expect("REASON").print().await?;
    Ok(())
}