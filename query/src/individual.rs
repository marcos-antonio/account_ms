use error::AppResult;
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

pub async fn get_by_id(pool: &PgPool, id: i32) -> AppResult<Individual> {
    let individual = sqlx::query_as!(Individual, r#"SELECT * FROM individual WHERE id = $1"#, id)
        .fetch_one(pool)
        .await?;
    Ok(individual)
}
