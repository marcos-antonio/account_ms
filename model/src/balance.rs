use chrono::NaiveDateTime;

#[derive(derive_builder::Builder, Clone, Debug, Default)]
pub struct Balance {
    pub id: i32,
    pub account_id: i32,
    pub balance: f32,
    pub created_date: NaiveDateTime,
    pub updated_date: Option<NaiveDateTime>,
}
