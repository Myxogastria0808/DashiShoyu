use crate::AppError;
use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct MeiliSearchObjectData {
    pub id: i32,
    pub name: String,
    pub photo_url: String,
    pub mime_type: String,
    pub license: String,
    pub tag: Vec<String>,
    pub description: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Clone, ToSchema)]
pub struct ObjectData {
    pub id: i32,
    pub name: String,
    pub photo_url: String,
    pub mime_type: String,
    pub license: String,
    pub tag: Vec<String>,
    pub description: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct RegisterObjectData {
    pub name: String,
    pub mime_type: String,
    pub license: String,
    pub tag: Vec<String>,
    pub description: String,
}

// #[derive(Debug)]
// pub struct RegisterObjectFieldCountFlags {
//     name: u8,
//     name_length: u8,
//     mime_type: u8,
//     mime_type_length: u8,
//     license: u8,
//     description: u8,
//     invalid_field_name: u8,
// }

// impl RegisterObjectFieldCountFlags {
//     pub fn initialize() -> Self {
//         Self {
//             name: 0,
//             name_length: 0,
//             mime_type: 0,
//             mime_type_length: 0,
//             license: 0,
//             description: 0,
//             invalid_field_name: 0,
//         }
//     }
//     pub fn increment_name_and_name_length(&mut self, name: &str) {
//         self.name += 1;
//         if !name.is_empty() {
//             self.name_length += 1;
//         }
//     }
//     pub fn increment_mime_type_and_mime_type_length(&mut self, mime_type: &str) {
//         self.mime_type += 1;
//         if !mime_type.is_empty() {
//             self.mime_type_length += 1;
//         }
//     }
//     pub fn increment_license(&mut self) {
//         self.license += 1;
//     }
//     pub fn increment_description(&mut self) {
//         self.description += 1;
//     }
//     pub fn increment_invalid_field_name(&mut self) {
//         self.invalid_field_name += 1;
//     }
//     pub fn check_is_ok(&self) -> Result<(), AppError> {
//         if self.name == 1
//             && self.name_length == 1
//             && self.mime_type == 1
//             && self.mime_type_length == 1
//             && self.license == 1
//             && self.description == 1
//             && self.invalid_field_name == 0
//         {
//             Ok(())
//         } else {
//             Err(AppError(anyhow::anyhow!("Invalid form-data input.")))
//         }
//     }
// }

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct UpdateObjectData {
    pub name: String,
    pub license: String,
    pub tag: Vec<String>,
    pub description: String,
}

// #[derive(Debug)]
// pub struct UpdateObjectFieldCountFlags {
//     name: u32,
//     name_length: u32,
//     license: u32,
//     description: u32,
//     invalid_field_name: u32,
// }

// impl UpdateObjectFieldCountFlags {
//     pub fn initialize() -> Self {
//         Self {
//             name: 0,
//             name_length: 0,
//             license: 0,
//             description: 0,
//             invalid_field_name: 0,
//         }
//     }
//     pub fn increment_name_and_name_length(&mut self, name: &str) {
//         self.name += 1;
//         if !name.is_empty() {
//             self.name_length += 1;
//         }
//     }
//     pub fn increment_license(&mut self) {
//         self.license += 1;
//     }
//     pub fn increment_description(&mut self) {
//         self.description += 1;
//     }
//     pub fn increment_invalid_field_name(&mut self) {
//         self.invalid_field_name += 1;
//     }
//     pub fn check_is_ok(&self) -> Result<(), AppError> {
//         if self.name == 1
//             && self.name_length == 1
//             && self.license == 1
//             && self.description == 1
//             && self.invalid_field_name == 0
//         {
//             Ok(())
//         } else {
//             Err(AppError(anyhow::anyhow!("Invalid field name.")))
//         }
//     }
// }

#[derive(Debug, Serialize, Clone, ToSchema)]
pub struct DeleteObjectData {
    status_code: String,
    message: String,
}

impl DeleteObjectData {
    pub fn generate() -> Json<Self> {
        Json(Self {
            status_code: format!("{}", StatusCode::OK),
            message: "Item delete was successfull.".to_string(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct MimeType;

impl MimeType {
    pub fn to_extension(mime_type: String) -> Result<String, AppError> {
        match mime_type.as_str() {
            "image/jpeg" => Ok("jpg".to_string()),
            "image/png" => Ok("png".to_string()),
            "image/svg+xml" => Ok("svg".to_string()),
            "image/gif" => Ok("gif".to_string()),
            "image/bmp" => Ok("bmp".to_string()),
            "image/webp" => Ok("webp".to_string()),
            "application/postscript" => Ok("ai".to_string()),
            _ => Err(AppError(anyhow::anyhow!("Unsupported mime type."))),
        }
    }
}
