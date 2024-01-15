use chrono::NaiveDateTime;

#[derive(derive_builder::Builder, Clone, Debug, Default)]
pub struct BalanceTransaction {
    pub id: i32,
    pub balance_id: i32,
    pub value: f32,
    pub created_date: NaiveDateTime,
    pub updated_date: Option<NaiveDateTime>,
}
