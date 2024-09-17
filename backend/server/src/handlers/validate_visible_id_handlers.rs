use ::entity::{
    item::{self, Entity as Item, Record},
    label::Color,
};
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

pub async fn validate_visible_id_post(
    Path(id): Path<i32>, //生成するvisible_idの個数
    Extension(db): Extension<DatabaseConnection>,
) -> Result<(), AppError> {
    Ok(())
}
