use oc_collect::CollectClient;

#[tokio::main]
#[test]
async fn it_fetches_usage_sessions() -> anyhow::Result<()> {
    let mut collect_client = CollectClient::new();
    collect_client.open_pool().await?;
    let _ = collect_client
        .get_usage_sessions(Some(1783688487402))
        .await?;

    Ok(())
}
