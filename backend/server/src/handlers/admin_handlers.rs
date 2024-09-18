use ::entity::{
    item::{self, Entity as Item, Record},
    label::{self, Color, Entity as Label},
};
use axum::{
    extract::{Multipart, Path, Query},
    Extension, Json,
};
use chrono::Utc;
use cloudflare_r2_rs::r2::R2Manager;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use serde_json::json;
use server::AppError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LabelModelData {
    visible_id: String,
    color: String,
}

pub async fn regiter_visible_id_post(
    Extension(db): Extension<DatabaseConnection>,
    label_model_data: Json<LabelModelData>,
) -> Result<(), AppError> {
    let _color_error_flag = false;
    let mut result_color = Color::Red;
    match label_model_data.color.as_str() {
        "red" => {
            result_color = Color::Red;
        }
        "orange" => {
            result_color = Color::Blue;
        }
        "brown" => {
            result_color = Color::Brown;
        }
        "skyblue" => {
            result_color = Color::SkyBlue;
        }
        "blue" => {
            result_color = Color::Blue;
        }
        "green" => {
            result_color = Color::Green;
        }
        "yellow" => {
            result_color = Color::Yellow;
        }
        "purple" => {
            result_color = Color::Purple;
        }
        "pink" => {
            result_color = Color::Pink;
        }
        _ => {
            let _color_error_flag = true;
        }
    }
    if _color_error_flag {
        return Err(AppError(anyhow::anyhow!("Invalid Color Name")));
    }
    let label_model = label::ActiveModel {
        visible_id: Set(label_model_data.visible_id.clone()),
        color: Set(result_color),
        ..Default::default()
    };
    let inserted_label_data = Label::insert(label_model).exec(&db).await?;
    println!(
        "[INFO]: Register Visible Id Result (admin end point): {:#?}",
        inserted_label_data
    );
    Ok(())
}
