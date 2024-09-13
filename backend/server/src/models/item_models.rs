use chrono::NaiveDateTime;
use entity::item::{self, Color, Record};
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
    pub record: Record,
    pub color: Color,
    pub description: String,
    pub year_purchased: Option<i32>,
    pub connector: Value,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<item::Model> for MeilisearchItemData {
    fn from(item: item::Model) -> Self {
        Self {
            id: item.id,
            visible_id: item.visible_id,
            parent_id: item.parent_id,
            parent_visible_id: item.parent_visible_id,
            grand_parent_id: item.grand_parent_id,
            grand_parent_visible_id: item.grand_parent_visible_id,
            name: item.name,
            product_number: item.product_number,
            photo_url: item.photo_url,
            record: item.record,
            color: item.color,
            description: item.description,
            year_purchased: item.year_purchased,
            connector: item.connector,
            created_at: item.created_at,
            updated_at: item.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ControlItemData {
    pub visible_id: String,
    pub parent_id: i32,
    pub parent_visible_id: String,
    pub grand_parent_id: i32,
    pub grand_parent_visible_id: String,
    pub name: String,
    pub product_number: String,
    pub record: Record,
    pub color: Color,
    pub description: String,
    pub year_purchased: Option<i32>,
    pub connector: Value,
}
