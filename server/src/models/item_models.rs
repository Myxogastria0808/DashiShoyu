use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MeilisearchItemData {
    pub id: i32,
    pub visible_id: String,
    pub parent_id: i32,
    pub parent_visible_id: String,
    pub grand_parent_id: i32,
    pub grand_parent_visible_id: String,
    pub name: String,
    pub product_number: String,
    pub photo_url: String,
    pub record: String,
    pub color: String,
    pub description: String,
    pub year_purchased: Option<i32>,
    pub connector: Value,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
