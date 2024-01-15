use sqlx::PgPool;

pub async fn save(pool: &PgPool, account_id: i32) -> Result<String, sqlx::Error> {
    sqlx::query!(
        r#"INSERT INTO balance (account_id) VALUES ($1)"#,
        account_id
    )
    .execute(pool)
    .await?;

    Ok("balance created".to_string())
}
