use chrono::NaiveDateTime;

#[derive(derive_builder::Builder, Clone, Debug, Default)]
pub struct Account {
    pub id: i32,
    pub individual_id: i32,
    pub number: i32,
    pub created_date: NaiveDateTime,
    pub updated_date: Option<NaiveDateTime>,
}
