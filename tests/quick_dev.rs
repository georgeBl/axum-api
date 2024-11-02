use anyhow::Result;
use serde_json::{json, Value};

#[tokio::test]
async fn quick_dev() -> httpc_test::Result<()> {
    // Create a new httpc test client with a base URL (will be prefixed for all calls)
    // The client will have a cookie_store.
    let hc = httpc_test::new_client("http://localhost:8080")?;

    //// do_get, do_post, do_put, do_patch, do_delete return a httpc_test::Response

    // Simple do_get
    let res = hc.do_get("/hello").await?; // httpc_test::Response
    let status = res.status();
    // Pretty print the result (status, headers, response cookies, client cookies, body)
    res.print().await?;

    Ok(())
}
