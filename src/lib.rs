use serde::Deserialize;
use sqlx::SqlitePool;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Model {
    id: String,
    #[serde(rename = "providerID")]
    provider_id: String,
    variant: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, sqlx::FromRow)]
pub struct RowUsageSession {
    id: String,
    project_id: String,
    slug: String,
    title: String,
    cost: f64,
    tokens_input: i64,
    tokens_output: i64,
    tokens_reasoning: i64,
    tokens_cache_read: i64,
    tokens_cache_write: i64,
    model: String,
    time_created: i64,
    time_updated: i64,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct UsageSessionData {
    session: RowUsageSession,
    model: Model,
}

pub struct CollectClient {
    pool: Option<SqlitePool>,
}

impl CollectClient {
    pub fn new() -> Self {
        CollectClient { pool: None }
    }

    pub fn get_db_path() -> String {
        let home = dirs::home_dir().unwrap();
        let db_path = home
            .join(".local")
            .join("share")
            .join("opencode")
            .join("opencode.db");
        db_path.to_str().unwrap().to_string()
    }

    pub async fn open_pool(&mut self) -> anyhow::Result<()> {
        let db_path = Self::get_db_path();
        let pool = SqlitePool::connect(&db_path).await?;
        self.pool = Some(pool);
        Ok(())
    }

    pub async fn get_usage_sessions(
        &self,
        min_updated_at: Option<i64>,
    ) -> anyhow::Result<Vec<UsageSessionData>> {
        let time_updated = match min_updated_at {
            Some(value) => value,
            None => 0, // Default to 0 if no value is provided
        };
        let pool = match &self.pool {
            Some(pool) => pool,
            None => {
                anyhow::bail!("Database pool is not initialized. Call open_pool() first.");
            }
        };

        let sessions = sqlx::query_as::<_, RowUsageSession>(
            r#"
        SELECT
            id,
            project_id,
            slug,
            title,
            cost,
            tokens_input,
            tokens_output,
            tokens_reasoning,
            tokens_cache_read,
            tokens_cache_write,
            model,
            time_created,
            time_updated
        FROM session
        WHERE time_updated > ?1
        ORDER BY time_created DESC
        "#,
        )
        .bind(time_updated)
        .fetch_all(pool)
        .await?;

        let extended_sessions = sessions
            .into_iter()
            .map(|session| {
                let model_result = serde_json::from_str::<Model>(&session.model);
                let model = match model_result {
                    Ok(model) => model,
                    Err(e) => {
                        eprintln!(
                            "Failed to deserialize model for session {}: {}",
                            session.id, e
                        );
                        Model {
                            id: "unknown".to_string(),
                            provider_id: "unknown".to_string(),
                            variant: None,
                        }
                    }
                };

                UsageSessionData { session, model }
            })
            .collect();

        Ok(extended_sessions)
    }
}
