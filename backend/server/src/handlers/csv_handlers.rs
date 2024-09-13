use ::entity::item::{self, Color, Entity as Item, Record};
use axum::{
    extract::{Multipart, Path, Query},
    Extension, Json,
};
use chrono::Utc;
use cloudflare_r2_rs::r2::R2Manager;
use entity::item::Model;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::de;
use serde_json::json;
use server::AppError;
use std::{collections::HashMap, thread::current};

pub async fn csv_get(Extension(db): Extension<DatabaseConnection>) -> Result<(), AppError> {
    Ok(())
}
