use model::individual::Individual;
use sqlx::PgPool;

pub async fn save(pool: &PgPool, item: &Individual) -> Result<Individual, sqlx::Error> {
    sqlx::query_as!(
        Individual,
        r#"INSERT INTO individual
				(name, document)
				VALUES ($1, $2)
				RETURNING id, name, document, created_date, updated_date"#,
        item.name,
        item.document,
    )
    .fetch_one(pool)
    .await
}
