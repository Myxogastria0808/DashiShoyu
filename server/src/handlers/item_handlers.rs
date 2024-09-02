use ::entity::item::{self, Entity as Item};
use axum::{
    extract::{Path, Query},
    Extension, Json,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde_json::json;
use server::AppError;
use std::collections::HashMap;

pub async fn search_item_get(
    Query(param): Query<HashMap<String, String>>,
) -> Result<Json<Vec<server::MeilisearchItemData>>, AppError> {
    let keywords = match param.get("keywords") {
        Some(keywords) => keywords,
        None => "",
    };
    //connect meilisearch
    let client = server::connect_meilisearch().await;
    //get result data
    let result: Vec<server::MeilisearchItemData> = client
        .index("item")
        .search()
        .with_query(keywords)
        .execute()
        .await?
        .hits
        .into_iter()
        .map(|item| item.result)
        .collect();
    Ok(Json(result))
}

pub async fn get_each_item_get(
    Path(id): Path<i32>,
    Extension(db): Extension<DatabaseConnection>,
) -> Result<Json<server::MeilisearchItemData>, AppError> {
    let result = Item::find_by_id(id).one(&db).await?;
    match result {
        Some(item) => {
            let item = server::MeilisearchItemData {
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
            };
            Ok(Json(item))
        }
        None => Err(AppError(anyhow::anyhow!("Item not found"))),
    }
}

pub async fn update_item_put(
    Path(id): Path<i32>,
    Extension(db): Extension<DatabaseConnection>,
) -> String {
}

pub async fn delete_item_delete(
    Path(id): Path<i32>,
    Extension(db): Extension<DatabaseConnection>,
) -> Result<Json<server::MeilisearchItemData>, AppError> {
    let delete_item = Item::find_by_id(id).one(&db).await?;
    //削除対象のノードがあるか確認
    match delete_item {
        Some(delete_item) => {
            //最上位のノードの場合
            if item.parent_id == id {
                return Err(AppError(anyhow::anyhow!("Can't delete top item")));
            }
            let children_items = Item::find()
                .filter(item::Column::ParentId.eq(id))
                .all(&db)
                .await?;
            //最下層のノードの場合
            if children_items.is_empty() {
                Item::delete_by_id(id).exec(&db).await?;
                return Ok(Json(delete_item));
            }
            if let Some(parent_item) = Item::find_by_id(delete_item.parent_id).one(&db).await? {
                for child_item in children_items {
                    let mut child_item: item::ActiveModel = child_item.into();
                    child_item.parent_id = Set(parent_item.id);
                    child_item.parent_visible_id = Set(parent_item.visible_id.to_owned());
                    child_item.grand_parent_id = Set(parent_item.parent_id);
                    child_item.grand_parent_visible_id =
                        Set(parent_item.parent_visible_id.to_owned());
                    child_item.update(&db).await?;
                }
            }
            Ok(Json(delete_item))
        }
        None => Err(AppError(anyhow::anyhow!("Item not found"))),
    }
}

pub async fn register_item_post(Extension(db): Extension<DatabaseConnection>) -> <Json<server::MeilisearchItemData>, AppError> {
    
}
