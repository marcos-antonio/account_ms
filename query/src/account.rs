use model::account::Account;
use sqlx::PgPool;

pub async fn save(pool: &PgPool, individual_id: i32) -> Result<Account, sqlx::Error> {
    sqlx::query_as!(
        Account,
        r#"INSERT INTO account (individual_id) VALUES ($1)
        returning id, individual_id, number, created_date, updated_date;"#,
        individual_id
    )
    .fetch_one(pool)
    .await
}
