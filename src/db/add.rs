use chrono::Utc;
use sqlx::{SqlitePool};

pub async fn add_user(
    pool: &SqlitePool,
    discord_id: String,
    account_id: String,
) -> Result<(), sqlx::Error> {
    let mut conn = pool.acquire().await.unwrap();
    let now = Utc::now().to_string();

    sqlx::query!(
        r#"
INSERT INTO users(discord_id, claimed, account_id)
VALUES( ?1, ?2, ?3 )"#,
        discord_id,
        now,
        account_id
    )
    .execute(&mut conn)
    .await?;

    Ok(())
}
