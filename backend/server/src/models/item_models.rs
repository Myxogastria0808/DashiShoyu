use chrono::NaiveDateTime;
use entity::{item::Record, label::Color};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MeiliSearchItemData {
    pub id: i32,
    pub visible_id: String,
    pub parent_visible_id: String,
    pub grand_parent_visible_id: String,
    pub name: String,
    pub product_number: String,
    pub photo_url: String,
    pub record: Record,
    pub color: Color,
    pub description: String,
    pub year_purchased: Option<i32>,
    pub connector: Value,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ItemData {
    pub id: i32,
    pub visible_id: String,
    pub parent_visible_id: String,
    pub name: String,
    pub product_number: String,
    pub photo_url: String,
    pub record: Record,
    pub color: Color,
    pub description: String,
    pub year_purchased: Option<i32>,
    pub connector: Value,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub path: Value,
    pub path_item_name: Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ControlItemData {
    pub visible_id: String,
    pub parent_visible_id: String,
    pub name: String,
    pub product_number: String,
    pub record: Record,
    pub description: String,
    pub year_purchased: Option<i32>,
    pub connector: Value,
}
