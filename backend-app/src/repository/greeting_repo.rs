use sqlx::PgPool;
use sqlx::Row;
use tracing::info;

pub async fn get_greeting(pool: &PgPool) -> anyhow::Result<String> {
    let row = sqlx::query("SELECT message FROM greetings LIMIT 1")
        .fetch_one(pool)
        .await?;

    let message: String = row.try_get("message")?;
    info!(%message, "Greeting fetched from database");
    Ok(message)
}
