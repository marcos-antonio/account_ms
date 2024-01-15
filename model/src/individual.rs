use chrono::NaiveDateTime;

#[derive(derive_builder::Builder, sqlx::FromRow, Clone, Debug, Default)]
pub struct Individual {
    #[builder(default)]
    pub id: i32,

    #[builder(default)]
    pub name: String,

    #[builder(default)]
    pub document: String,

    #[builder(default)]
    pub created_date: NaiveDateTime,

    #[builder(default)]
    pub updated_date: Option<NaiveDateTime>,
}
