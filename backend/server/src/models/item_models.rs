use axum::{http::StatusCode, Json};
use entity::{item::Record, label::Color};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct MeiliSearchItemData {
    pub id: i32,
    pub visible_id: String,
    pub name: String,
    pub product_number: String,
    pub photo_url: String,
    pub record: Record,
    pub color: Color,
    pub description: String,
    pub year_purchased: Option<i32>,
    pub connector: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
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
    pub connector: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
    pub path: Vec<i64>,
    pub visible_id_path: Vec<String>,
    pub item_name_path: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct ControlItemData {
    pub visible_id: String,
    pub parent_visible_id: String,
    pub name: String,
    pub product_number: String,
    pub record: Record,
    pub description: String,
    pub year_purchased: Option<i32>,
    pub connector: Vec<String>,
}

// #[derive(Debug, Clone)]
// pub struct ControlItemFieldCountFlags {
//     visible_id: u8,
//     visible_id_length: u8,
//     parent_visible_id: u8,
//     parent_visible_id_length: u8,
//     name: u8,
//     name_length: u8,
//     product_number: u8,
//     record: u8,
//     record_pattern: u8,
//     description: u8,
//     year_purchased: u8,
//     invalid_field_name: u8,
// }

// impl ControlItemFieldCountFlags {
//     pub fn initialize() -> Self {
//         Self {
//             visible_id: 0,
//             visible_id_length: 0,
//             parent_visible_id: 0,
//             parent_visible_id_length: 0,
//             name: 0,
//             name_length: 0,
//             product_number: 0,
//             record: 0,
//             record_pattern: 0,
//             description: 0,
//             year_purchased: 0,
//             invalid_field_name: 0,
//         }
//     }
//     pub fn increment_visible_id(&mut self, visible_id: &str) {
//         self.visible_id += 1;
//         if !visible_id.is_empty() {
//             self.visible_id_length += 1;
//         }
//     }
//     pub fn increment_parent_visible_id(&mut self, parent_visible_id: &str) {
//         self.parent_visible_id += 1;
//         if !parent_visible_id.is_empty() {
//             self.parent_visible_id_length += 1;
//         }
//     }
//     pub fn increment_name(&mut self, name: &str) {
//         self.name += 1;
//         if !name.is_empty() {
//             self.name_length += 1;
//         }
//     }
//     pub fn increment_product_number(&mut self) {
//         self.product_number += 1;
//     }
//     pub fn increment_record(&mut self, record: &str) {
//         self.record += 1;
//         if record != "Qr" && record != "Barcode" && record != "Nothing" {
//             //record_patternは、0でないといけない
//             self.record_pattern += 1;
//         }
//     }
//     pub fn increment_description(&mut self) {
//         self.description += 1;
//     }
//     pub fn increment_year_purchased(&mut self) {
//         self.year_purchased += 1;
//     }
//     pub fn increment_invalid_field_name(&mut self) {
//         //invalid_field_nameは、0でないといけない
//         self.invalid_field_name += 1;
//     }
//     pub fn check_is_ok(&self) -> Result<(), AppError> {
//         if self.visible_id == 1
//             && self.visible_id_length == 1
//             && self.parent_visible_id == 1
//             && self.parent_visible_id_length == 1
//             && self.name == 1
//             && self.name_length == 1
//             && self.product_number == 1
//             && self.record == 1
//             && self.record_pattern == 0
//             && self.description == 1
//             && self.year_purchased == 1
//             && self.invalid_field_name == 0
//         {
//             Ok(())
//         } else {
//             Err(AppError(anyhow::anyhow!("Invalid form-data input.")))
//         }
//     }
// }

#[derive(Debug, Serialize, Clone, ToSchema)]
pub struct DeleteItemData {
    status_code: String,
    message: String,
}

impl DeleteItemData {
    pub fn generate() -> Json<Self> {
        Json(Self {
            status_code: format!("{}", StatusCode::OK),
            message: "Item delete was successfull.".to_string(),
        })
    }
}

#[derive(Debug, Serialize, Clone, ToSchema)]
pub struct CsvItemData {
    pub product_number: String,
    pub name: String,
    pub place: String,
    pub note: String,
}
