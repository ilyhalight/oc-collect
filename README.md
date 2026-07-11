# oc-collect

Small lib to collect data from OpenCode database.

## Usage

Install:

```bash
cargo install oc-collect
```

Usage:

```rs
use oc_collect::CollectClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut client = CollectClient::new();
    client.open_pool().await?;
    // Get all usage sessions updated after a specific timestamp (in milliseconds)
    // Use None for get all sessions
    let sessions = client.get_usage_sessions(Some(1783688487402)).await?;
    // Process the sessions...
    Ok(())
}
```

Inspired by [opencode-stats](https://github.com/Cateds/opencode-stats)
