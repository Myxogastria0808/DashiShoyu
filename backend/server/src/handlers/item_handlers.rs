use ::entity::{
    item::{self, Entity as Item},
    label::{self, Color, Entity as Label},
};
use axum::{
    extract::{Path, Query},
    Extension, Json,
};
use chrono::Utc;
use meilisearch_sdk::client::Client;
use neo4rs::Graph;
use rand::{distributions::Alphanumeric, Rng};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde_json::json;
use server::AppError;
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

//* search with meilisearch *//
#[axum::debug_handler]
#[utoipa::path(
    get,
    path = "/api/item/search",
    params(("keywords", Query, description = "set search word")),
    responses(
        (status = 200, description = "OK", body = Vec<MeiliSearchItemData>),
    ),
    tag = "Item",
)]
pub async fn search_item_get(
    Query(param): Query<HashMap<String, String>>,
    Extension(meilisearch_client): Extension<meilisearch_sdk::client::Client>,
) -> Result<Json<Vec<server::MeiliSearchItemData>>, AppError> {
    let keywords = match param.get("keywords") {
        Some(keywords) => keywords,
        None => "",
    };
    //get result data
    let result: Vec<server::MeiliSearchItemData> = meilisearch_client
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

//移動するんだよ！！！！！
////////////////////////////////////////////////////////////////////////////////////////
async fn get_item_data(
    id: i32,
    db: DatabaseConnection,
    graph: Graph,
) -> Result<Json<server::ItemData>, AppError> {
    let item_model: item::Model = Item::find_by_id(id)
        .one(&db)
        .await?
        .ok_or(AppError(anyhow::anyhow!("Item was not found.")))?;
    let label_model: label::Model = Label::find()
        .filter(label::Column::VisibleId.eq(item_model.visible_id.clone()))
        .one(&db)
        .await?
        .ok_or(AppError(anyhow::anyhow!("Parent label was not found.")))?;
    let path = server::search_path(&graph, item_model.id.into()).await?;
    if path.len() != 1 {
        let parent_item_model: item::Model = Item::find_by_id(path[1] as i32)
            .one(&db)
            .await?
            .ok_or(AppError(anyhow::anyhow!("Parent item was not found.")))?;
        let mut item_name_path: Vec<String> = Vec::new();
        let mut visible_id_path: Vec<String> = Vec::new();
        for id in &path {
            let item_path_model: item::Model = Item::find_by_id(*id as i32)
                .one(&db)
                .await?
                .ok_or(AppError(anyhow::anyhow!("Parent item was not found.")))?;
            item_name_path.push(item_path_model.name);
            visible_id_path.push(item_path_model.visible_id);
        }
        let item = server::ItemData {
            id: item_model.id,
            visible_id: item_model.visible_id,
            parent_visible_id: parent_item_model.visible_id,
            name: item_model.name,
            product_number: item_model.product_number,
            photo_url: item_model.photo_url,
            record: item_model.record,
            color: label_model.color,
            description: item_model.description,
            year_purchased: item_model.year_purchased,
            connector: serde_json::from_value(item_model.connector.clone())?,
            created_at: item_model.created_at.to_string(),
            updated_at: item_model.updated_at.to_string(),
            path,
            visible_id_path,
            item_name_path,
        };
        Ok(Json(item))
    } else {
        let item = server::ItemData {
            id: item_model.id,
            visible_id: item_model.visible_id.clone(),
            parent_visible_id: item_model.visible_id.clone(),
            name: item_model.name.clone(),
            product_number: item_model.product_number,
            photo_url: item_model.photo_url,
            record: item_model.record,
            color: label_model.color,
            description: item_model.description,
            year_purchased: item_model.year_purchased,
            connector: serde_json::from_value(item_model.connector.clone())?,
            created_at: item_model.created_at.to_string(),
            updated_at: item_model.updated_at.to_string(),
            path,
            visible_id_path: vec![item_model.visible_id],
            item_name_path: vec![item_model.name],
        };
        Ok(Json(item))
    }
}

//* get one item *//
#[axum::debug_handler]
#[utoipa::path(
    get,
    path = "/api/item/get/{id}",
    params(("id", Path, description = "set get item id")),
    responses(
        (status = 200, description = "OK", body = ItemData),
    ),
    tag = "Item",
)]
pub async fn get_each_item_get(
    Path(id): Path<i32>,
    Extension(db): Extension<DatabaseConnection>,
    Extension(graph): Extension<Graph>,
) -> Result<Json<server::ItemData>, AppError> {
    let item_data = get_item_data(id, db, graph).await?;
    Ok(item_data)
}

//* update *//
#[axum::debug_handler]
#[utoipa::path(
    put,
    path = "/api/item/update/{id}",
    params(("id", Path, description = "set update item id")),
    request_body(content = ControlItemData, description = "set update item data"),
    responses(
        (status = 200, description = "OK", body = ItemData),
    ),
    tag = "Item",
)]
pub async fn update_item_put(
    Path(id): Path<i32>,
    Extension(db): Extension<DatabaseConnection>,
    Extension(graph): Extension<Graph>,
    Extension(meilisearch_client): Extension<Client>,
    Json(update_data): Json<server::ControlItemData>,
) -> Result<Json<server::ItemData>, AppError> {
    /////////////////////////////////////////////////////////////
    // //validate_flag
    // let mut validate_flag = server::ControlItemFieldCountFlags::initialize();
    // //connectorのvector
    // let mut result_connector_vec: Vec<String> = Vec::new();
    // let mut update_data = server::ControlItemData {
    //     visible_id: "".to_string(),
    //     parent_visible_id: "".to_string(),
    //     name: "".to_string(),
    //     product_number: "".to_string(),
    //     record: Record::Qr,
    //     description: "".to_string(),
    //     year_purchased: None,
    //     connector: result_connector_vec.clone(),
    // };
    // while let Some(field) = multipart.next_field().await? {
    //     let field_name = field.name().unwrap().to_string();
    //     println!("field name: {}", field_name);
    //     //connector
    //     if field_name.starts_with("connector") {
    //         let connector = field.text().await?;
    //         println!("connector: {}", connector);
    //         result_connector_vec.push(connector);
    //         continue;
    //     }
    //     match field_name.as_str() {
    //         "visible_id" => {
    //             let visible_id = field.text().await?;
    //             println!("visible_id: {}", visible_id);
    //             validate_flag.increment_visible_id(&visible_id);
    //             update_data.visible_id = visible_id;
    //         }
    //         "parent_visible_id" => {
    //             let parent_visible_id = field.text().await?;
    //             println!("parent_visible_id: {}", parent_visible_id);
    //             validate_flag.increment_parent_visible_id(&parent_visible_id);
    //             update_data.parent_visible_id = parent_visible_id;
    //         }
    //         "name" => {
    //             let name = field.text().await?;
    //             println!("name: {}", name);
    //             validate_flag.increment_name(&name);
    //             update_data.name = name;
    //         }
    //         "product_number" => {
    //             let product_number = field.text().await?;
    //             println!("product_number: {}", product_number);
    //             validate_flag.increment_product_number();
    //             update_data.product_number = product_number;
    //         }
    //         "record" => {
    //             let record = field.text().await?;
    //             println!("record: {}", record);
    //             validate_flag.increment_record(&record);
    //             update_data.record = match record.as_str() {
    //                 "Qr" => Record::Qr,
    //                 "Barcode" => Record::Barcode,
    //                 "Nothing" => Record::Nothing,
    //                 _ => panic!("Record type validation was failed"),
    //             };
    //         }
    //         "description" => {
    //             let description = field.text().await?;
    //             println!("description: {}", description);
    //             validate_flag.increment_description();
    //             update_data.description = description;
    //         }
    //         "year_purchased" => {
    //             let year_purchased = field.text().await?;
    //             println!("year_purchased: {}", year_purchased);
    //             validate_flag.increment_year_purchased();
    //             if year_purchased.is_empty() {
    //                 update_data.year_purchased = None;
    //             } else {
    //                 update_data.year_purchased = Some(year_purchased.parse::<i32>()?);
    //             }
    //         }
    //         _ => {
    //             println!("other");
    //             validate_flag.increment_invalid_field_name();
    //         }
    //     }
    // }
    // //validate_check
    // validate_flag.check_is_ok()?;
    // //update connector
    // update_data.connector = result_connector_vec;
    /////////////////////////////////////////////////////////////

    //update neo4j
    'neo4j_break: {
        let parent_id_struct = server::search_parent_id(&db, &graph, id.into()).await?;
        let old_parent_id = parent_id_struct.actual_parent_id;
        let is_actual_root = parent_id_struct.is_actual_root;
        //rootの早期
        if is_actual_root {
            break 'neo4j_break;
        }
        //親物品が変わっているかどうかの確認
        let new_parent_id: i64 = Item::find()
            .filter(item::Column::VisibleId.eq(update_data.parent_visible_id))
            .one(&db)
            .await?
            .ok_or(AppError(anyhow::anyhow!("Parent item was not found.")))?
            .id
            .into();
        if old_parent_id == new_parent_id {
            break 'neo4j_break;
        }
        //リレーションを張り替える処理
        let mut descendants = server::search_descendants_ids_hashset(&graph, id.into()).await?;
        //子孫のノードが親のノードになっていないかの確認
        let is_not_duplication = descendants.insert(new_parent_id);
        if !is_not_duplication {
            return Err(AppError(anyhow::anyhow!(
                "Cannot change relation to descendants"
            )));
        }
        //リレーションの張り替え
        server::reconnect_new_parent_item(&graph, old_parent_id, new_parent_id, id.into()).await?;
    }
    //update Item table
    {
        let item_model: item::Model = Item::find_by_id(id)
            .one(&db)
            .await?
            .ok_or(AppError(anyhow::anyhow!("Item was not found.")))?;
        let mut item_active_model: item::ActiveModel = item_model.into();
        item_active_model.visible_id = Set(update_data.visible_id);
        item_active_model.name = Set(update_data.name);
        item_active_model.product_number = Set(update_data.product_number);
        item_active_model.record = Set(update_data.record);
        item_active_model.description = Set(update_data.description);
        item_active_model.year_purchased = Set(update_data.year_purchased);
        item_active_model.connector = Set(json!(update_data.connector));
        item_active_model.updated_at = Set(Utc::now().naive_local());
        item_active_model.update(&db).await?;
    }
    //update meilisearch
    {
        //更新後のitem_model
        let item_model = Item::find_by_id(id)
            .one(&db)
            .await?
            .ok_or(AppError(anyhow::anyhow!("Item was not found.")))?;
        //更新後のitem_modelに対するlabel_model
        let label_model = Label::find()
            .filter(label::Column::VisibleId.eq(item_model.visible_id.clone()))
            .one(&db)
            .await?
            .ok_or(AppError(anyhow::anyhow!("Label was not found.")))?;
        let meilisearch_data = server::MeiliSearchItemData {
            id,
            visible_id: item_model.visible_id,
            name: item_model.name,
            product_number: item_model.product_number,
            photo_url: item_model.photo_url,
            record: item_model.record,
            color: label_model.color,
            description: item_model.description,
            year_purchased: item_model.year_purchased,
            connector: serde_json::from_value(item_model.connector.clone())?,
            created_at: item_model.created_at.to_string(),
            updated_at: item_model.updated_at.to_string(),
        };
        meilisearch_client
            .index("item")
            .add_documents(&[meilisearch_data], Some("id"))
            .await?;
    }
    //更新後のitem_modelを返す
    let item_data = get_item_data(id, db, graph).await?;
    Ok(item_data)
}

//* register *//
#[axum::debug_handler]
#[utoipa::path(
    post,
    path = "/api/item/register",
    request_body(content = ControlItemData, description = "set update item data"),
    responses(
        (status = 201, description = "Created", body = ItemData),
    ),
    tag = "Item",
)]
pub async fn register_item_post(
    Extension(db): Extension<DatabaseConnection>,
    Extension(graph): Extension<Graph>,
    Extension(meilisearch_client): Extension<Client>,
    Extension(r2_url): Extension<String>,
    Json(register_data): Json<server::ControlItemData>,
) -> Result<Json<server::ItemData>, AppError> {
    // //validate_flag
    // let mut validate_flag = server::ControlItemFieldCountFlags::initialize();
    // //connectorのvector
    // let mut result_connector_vec: Vec<String> = Vec::new();
    // let mut register_data = server::ControlItemData {
    //     visible_id: "".to_string(),
    //     parent_visible_id: "".to_string(),
    //     name: "".to_string(),
    //     product_number: "".to_string(),
    //     record: Record::Qr,
    //     description: "".to_string(),
    //     year_purchased: None,
    //     connector: result_connector_vec.clone(),
    // };
    // while let Some(field) = multipart.next_field().await? {
    //     let field_name = field.name().unwrap().to_string();
    //     println!("field name: {}", field_name);
    //     //connector
    //     if field_name.starts_with("connector") {
    //         let connector = field.text().await?;
    //         println!("connector: {}", connector);
    //         result_connector_vec.push(connector);
    //         continue;
    //     }
    //     match field_name.as_str() {
    //         "visible_id" => {
    //             let visible_id = field.text().await?;
    //             println!("visible_id: {}", visible_id);
    //             validate_flag.increment_visible_id(&visible_id);
    //             register_data.visible_id = visible_id;
    //         }
    //         "parent_visible_id" => {
    //             let parent_visible_id = field.text().await?;
    //             println!("parent_visible_id: {}", parent_visible_id);
    //             validate_flag.increment_parent_visible_id(&parent_visible_id);
    //             register_data.parent_visible_id = parent_visible_id;
    //         }
    //         "name" => {
    //             let name = field.text().await?;
    //             println!("name: {}", name);
    //             validate_flag.increment_name(&name);
    //             register_data.name = name;
    //         }
    //         "product_number" => {
    //             let product_number = field.text().await?;
    //             println!("product_number: {}", product_number);
    //             validate_flag.increment_product_number();
    //             register_data.product_number = product_number;
    //         }
    //         "record" => {
    //             let record = field.text().await?;
    //             println!("record: {}", record);
    //             //Recordに不正な値が入っている場合の早期リターン
    //             validate_flag.increment_record(&record);
    //             register_data.record = match record.as_str() {
    //                 "Qr" => Record::Qr,
    //                 "Barcode" => Record::Barcode,
    //                 "Nothing" => Record::Nothing,
    //                 _ => panic!("Record type validation was failed"),
    //             };
    //         }
    //         "description" => {
    //             let description = field.text().await?;
    //             println!("description: {}", description);
    //             validate_flag.increment_description();
    //             register_data.description = description;
    //         }
    //         "year_purchased" => {
    //             let year_purchased = field.text().await?;
    //             println!("year_purchased: {}", year_purchased);
    //             validate_flag.increment_year_purchased();
    //             if year_purchased.is_empty() {
    //                 register_data.year_purchased = None;
    //             } else {
    //                 register_data.year_purchased = Some(year_purchased.parse::<i32>()?);
    //             }
    //         }
    //         _ => {
    //             println!("other");
    //             validate_flag.increment_invalid_field_name();
    //         }
    //     }
    // }
    // //validate_check
    // validate_flag.check_is_ok()?;
    // //update connector
    // register_data.connector = result_connector_vec;
    ///////////////////////////////////////////////

    //insert to RDB
    let oneself_id = {
        let item_model = item::ActiveModel {
            visible_id: Set(register_data.visible_id.clone()),
            name: Set(register_data.name),
            product_number: Set(register_data.product_number),
            photo_url: Set(format!("{}/{}.webp", r2_url, Uuid::new_v4())),
            record: Set(register_data.record),
            description: Set(register_data.description),
            year_purchased: Set(register_data.year_purchased),
            connector: Set(json!(register_data.connector)),
            created_at: Set(Utc::now().naive_local()),
            updated_at: Set(Utc::now().naive_local()),
            ..Default::default()
        };
        let _ = Item::insert(item_model).exec(&db).await?;
        let item_model: item::Model = Item::find()
            .filter(item::Column::VisibleId.eq(register_data.visible_id))
            .one(&db)
            .await?
            .ok_or(AppError(anyhow::anyhow!("Item was not found.")))?;
        let oneself_id = item_model.id;
        let mut item_active_model: item::ActiveModel = item_model.into();
        item_active_model.photo_url = Set(format!("{}/{}.webp", r2_url, oneself_id));
        item_active_model.update(&db).await?;
        oneself_id
    };

    //insert to Neo4j
    {
        let parent_id = Item::find()
            .filter(item::Column::VisibleId.eq(register_data.parent_visible_id))
            .one(&db)
            .await?
            .ok_or(AppError(anyhow::anyhow!("Item was not found.")))?
            .id;
        let is_parent_exist = server::is_item_exits(&graph, parent_id.into()).await?;
        if !is_parent_exist {
            return Err(AppError(anyhow::anyhow!("Parent item was not found.")));
        }
        server::create_single_item(&graph, oneself_id.into()).await?;
        server::connect_items(&graph, parent_id.into(), oneself_id.into()).await?;
    }

    //insert to MeiliSearch
    {
        let item_model: item::Model = Item::find_by_id(oneself_id)
            .one(&db)
            .await?
            .ok_or(AppError(anyhow::anyhow!("Item was not found.")))?;
        let label_model: label::Model = Label::find()
            .filter(label::Column::VisibleId.eq(item_model.visible_id.clone()))
            .one(&db)
            .await?
            .ok_or(AppError(anyhow::anyhow!("Label was not found.")))?;
        let meilisearch_data = server::MeiliSearchItemData {
            id: item_model.id,
            visible_id: item_model.visible_id,
            name: item_model.name,
            product_number: item_model.product_number,
            photo_url: item_model.photo_url,
            record: item_model.record,
            color: label_model.color,
            description: item_model.description,
            year_purchased: item_model.year_purchased,
            connector: serde_json::from_value(item_model.connector)?,
            created_at: item_model.created_at.to_string(),
            updated_at: item_model.updated_at.to_string(),
        };
        let _ = meilisearch_client
            .index("item")
            .add_documents(&[meilisearch_data], Some("id"))
            .await
            .unwrap();
    }
    //return
    //更新後のitem_modelを返す
    let item_data = get_item_data(oneself_id, db, graph).await?;
    Ok(item_data)
}

//* delete *//
#[axum::debug_handler]
#[utoipa::path(
    delete,
    path = "/api/item/delete/{id}",
    params(("id", Path, description = "set update item id")),
    responses(
        (status = 200, description = "OK", body = DeleteItemData),
    ),
    tag = "Item",
)]
pub async fn delete_item_delete(
    Path(id): Path<i32>,
    Extension(db): Extension<DatabaseConnection>,
    Extension(graph): Extension<Graph>,
    Extension(meilisearch_client): Extension<Client>,
) -> Result<Json<server::DeleteItemData>, AppError> {
    //neo4j
    {
        let parent_id_struxt = server::search_parent_id(&db, &graph, id.into()).await?;
        let actual_parent_id = parent_id_struxt.actual_parent_id;
        let is_actual_root = parent_id_struxt.is_actual_root;
        if is_actual_root {
            return Err(AppError(anyhow::anyhow!("Root Item cannot remove.")));
        }
        let children_ids = server::search_children_ids(&graph, id.into()).await?;
        for child_id in children_ids {
            server::connect_items(&graph, actual_parent_id, child_id).await?;
        }
        server::delete_item(&graph, id.into()).await?;
    }
    //RDB
    {
        let _ = Item::delete_by_id(id).exec(&db).await?;
    }
    //meilisearch
    {
        let _ = meilisearch_client
            .index("item")
            .delete_document(id)
            .await
            .unwrap();
    }
    Ok(server::DeleteItemData::generate())
}

//* validate　visible　id *//
#[axum::debug_handler]
#[utoipa::path(
    post,
    path = "/api/item/generate/visible-id/{number}",
    params(("number", Path, description = "set generate item id amount")),
    responses(
        (status = 200, description = "OK", body = Vec<label::Model>),
    ),
    tag = "Item",
)]
pub async fn generate_visible_ids_post(
    Path(number): Path<i32>, //生成するvisible_idの個数
    Extension(db): Extension<DatabaseConnection>,
) -> Result<Json<Vec<label::Model>>, AppError> {
    let visivle_id_hashset = Label::find()
        .all(&db)
        .await?
        .into_iter()
        .map(|label| label.visible_id)
        .collect::<HashSet<String>>();
    let mut new_visible_id: String;
    let mut label_vec: Vec<label::Model> = Vec::new();
    for _ in 0..number {
        let color_index: usize = rand::random::<usize>() % Color::COLOR_PALETTE.len();
        new_visible_id = {
            let mut rng = rand::thread_rng();
            loop {
                let new_id = (0..5)
                    .map(|_| rng.sample(Alphanumeric).to_ascii_uppercase() as char)
                    .collect();
                if !visivle_id_hashset.contains(&new_id) {
                    break new_id;
                }
            }
        };
        label_vec.push(label::Model {
            visible_id: new_visible_id.clone(),
            color: Color::COLOR_PALETTE[color_index].clone(),
        });
        let label_model = label::ActiveModel {
            visible_id: Set(new_visible_id),
            color: Set(Color::COLOR_PALETTE[color_index].clone()),
        };
        let _ = Label::insert(label_model).exec(&db).await?;
    }
    Ok(Json(label_vec))
}

//* generate csv *//
#[axum::debug_handler]
#[utoipa::path(
    get,
    path = "/api/item/get/csv-data",
    responses(
        (status = 200, description = "OK", body = Vec<CsvItemData>),
    ),
    tag = "Item",
)]
pub async fn generate_csv_get(
    Extension(db): Extension<DatabaseConnection>,
    Extension(graph): Extension<Graph>,
) -> Result<Json<Vec<server::CsvItemData>>, AppError> {
    let mut csv_data_vec: Vec<server::CsvItemData> = Vec::new();
    let all_ids = server::get_all_ids(&db, &graph).await?;
    for ids in all_ids {
        let actual_root_id = ids.actual_route_id;
        let descendants_items = ids.descendants;
        let actual_root_item = Item::find_by_id(actual_root_id)
            .one(&db)
            .await?
            .ok_or(AppError(anyhow::anyhow!("Actual root item was not found.")))?;
        for descendant in descendants_items {
            let item = Item::find_by_id(descendant)
                .one(&db)
                .await?
                .ok_or(AppError(anyhow::anyhow!("Item was not found.")))?;
            let csv_data: server::CsvItemData = server::CsvItemData {
                product_number: item.product_number,
                name: item.name,
                place: actual_root_item.name.clone(),
                note: item.description,
            };
            csv_data_vec.push(csv_data);
        }
    }
    Ok(Json(csv_data_vec))
}
