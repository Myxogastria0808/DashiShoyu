use ::entity::{
    object::{self, Entity as Object},
    object_tag_junction::{self, Entity as ObjectTagJunction},
    tag::{self, Entity as Tag},
};
use axum::{
    extract::{Path, Query},
    Extension, Json,
};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use server::AppError;
use std::collections::HashMap;
use uuid::Uuid;

//* search with meilisearch *//
#[axum::debug_handler]
#[utoipa::path(
    get,
    path = "/api/object/search",
    params(("keywords", Query, description = "set search word")),
    responses(
        (status = 200, description = "OK", body = Vec<MeiliSearchObjectData>),
    ),
    tag = "Object",
)]
pub async fn search_object_get(
    Query(param): Query<HashMap<String, String>>,
    Extension(meilisearch_client): Extension<meilisearch_sdk::client::Client>,
) -> Result<Json<Vec<server::MeiliSearchObjectData>>, AppError> {
    let keywords = match param.get("keywords") {
        Some(keywords) => keywords,
        None => "",
    };
    //get result data
    let result: Vec<server::MeiliSearchObjectData> = meilisearch_client
        .index("object")
        .search()
        .with_query(keywords)
        .execute()
        .await?
        .hits
        .into_iter()
        .map(|object| object.result)
        .collect();
    Ok(Json(result))
}

///* get one object *//
#[axum::debug_handler]
#[utoipa::path(
    get,
    path = "/api/object/get/{id}",
    params(("id", Path, description = "set get item id")),
    responses(
        (status = 200, description = "OK", body = ObjectData),
    ),
    tag = "Object",
)]
pub async fn get_each_object_get(
    Path(id): Path<i32>,
    Extension(db): Extension<DatabaseConnection>,
) -> Result<Json<server::ObjectData>, AppError> {
    let object_model = Object::find_by_id(id)
        .one(&db)
        .await?
        .ok_or(AppError(anyhow::anyhow!("Object was not found.")))?;
    let tag_ids = ObjectTagJunction::find()
        .filter(object_tag_junction::Column::ObjectId.eq(id))
        .all(&db)
        .await?;
    let tag_names = {
        let mut tag_names = Vec::new();
        for tag_id in tag_ids {
            let tag_name = Tag::find_by_id(tag_id.tag_id)
                .one(&db)
                .await?
                .ok_or(AppError(anyhow::anyhow!("Tag was not found.")))?
                .name;
            tag_names.push(tag_name);
        }
        tag_names
    };
    let object = server::ObjectData {
        id: object_model.id,
        name: object_model.name,
        photo_url: object_model.photo_url,
        mime_type: object_model.mime_type,
        license: object_model.license,
        tag: tag_names,
        description: object_model.description,
        created_at: object_model.created_at.to_string(),
        updated_at: object_model.updated_at.to_string(),
    };
    Ok(Json(object))
}

//* search object with tag *//
#[axum::debug_handler]
#[utoipa::path(
    get,
    path = "/api/object/tag/{tag}",
    params(("tag" = String, Path, description = "set object tag")),
    responses(
        (status = 200, description = "OK", body = Vec<ObjectData>),
    ),
    tag = "Object",
)]
pub async fn get_object_with_tag_get(
    Path(tag): Path<String>,
    Extension(db): Extension<DatabaseConnection>,
) -> Result<Json<Vec<server::ObjectData>>, AppError> {
    let tag_id = Tag::find()
        .filter(tag::Column::Name.eq(tag))
        .one(&db)
        .await?
        .ok_or(AppError(anyhow::anyhow!("Tag was not found.")))?
        .id;
    let object_ids = ObjectTagJunction::find()
        .filter(object_tag_junction::Column::TagId.eq(tag_id))
        .all(&db)
        .await?
        .iter()
        .map(|object_tag_junction| object_tag_junction.object_id)
        .collect::<Vec<i32>>();
    let mut object_models: Vec<server::ObjectData> = Vec::new();
    for object_id in object_ids {
        let object_model = Object::find_by_id(object_id)
            .one(&db)
            .await?
            .ok_or(AppError(anyhow::anyhow!("Object was not found.")))?;
        let tag_ids = ObjectTagJunction::find()
            .filter(object_tag_junction::Column::ObjectId.eq(object_id))
            .all(&db)
            .await?
            .iter()
            .map(|object_tag_junction| object_tag_junction.tag_id)
            .collect::<Vec<i32>>();
        let mut tag_names: Vec<String> = Vec::new();
        for tag_id in tag_ids {
            let tag_name = Tag::find_by_id(tag_id)
                .one(&db)
                .await?
                .ok_or(AppError(anyhow::anyhow!("Tag was not found.")))?
                .name;
            tag_names.push(tag_name);
        }
        object_models.push(server::ObjectData {
            id: object_model.id,
            name: object_model.name,
            photo_url: object_model.photo_url,
            mime_type: object_model.mime_type,
            license: object_model.license,
            tag: tag_names,
            description: object_model.description,
            created_at: object_model.created_at.to_string(),
            updated_at: object_model.updated_at.to_string(),
        });
    }
    Ok(Json(object_models))
}

//* register *//
#[axum::debug_handler]
#[utoipa::path(
    post,
    path = "/api/object/register",
    request_body(content = RegisterObjectData, description = "set update item data"),
    responses(
        (status = 201, description = "Created", body = ObjectData),
    ),
    tag = "Object",
)]
pub async fn register_object_post(
    Extension(db): Extension<DatabaseConnection>,
    Extension(r2_url): Extension<String>,
    Extension(meilisearch_client): Extension<meilisearch_sdk::client::Client>,
    Json(register_data): Json<server::RegisterObjectData>,
) -> Result<Json<server::ObjectData>, AppError> {
    //////////////////////////////////////////
    // //validate_flag
    // let mut validate_flag = server::RegisterObjectFieldCountFlags::initialize();
    // //connectorのvector
    // let mut result_tag_vec: Vec<String> = Vec::new();
    // let mut register_data = server::RegisterObjectData {
    //     name: "".to_string(),
    //     mime_type: "".to_string(),
    //     license: "".to_string(),
    //     tag: result_tag_vec.clone(),
    //     description: "".to_string(),
    // };
    // while let Some(field) = multipart.next_field().await? {
    //     let field_name = field.name().unwrap().to_string();
    //     println!("field name: {}", field_name);
    //     //connector
    //     if field_name.starts_with("tag") {
    //         let tag = field.text().await?;
    //         println!("tag: {}", tag);
    //         result_tag_vec.push(tag);
    //         continue;
    //     }
    //     match field_name.as_str() {
    //         "name" => {
    //             let name = field.text().await?;
    //             println!("name: {}", name);
    //             validate_flag.increment_name_and_name_length(&name);
    //             register_data.name = name;
    //         }
    //         "mime_type" => {
    //             let mime_type = field.text().await?;
    //             println!("mime_type: {}", mime_type);
    //             validate_flag.increment_mime_type_and_mime_type_length(&mime_type);
    //             register_data.mime_type = mime_type;
    //         }
    //         "license" => {
    //             let license = field.text().await?;
    //             println!("license: {}", license);
    //             validate_flag.increment_license();
    //             register_data.license = license;
    //         }
    //         "description" => {
    //             let description = field.text().await?;
    //             println!("description: {}", description);
    //             validate_flag.increment_description();
    //             register_data.description = description;
    //         }
    //         _ => {
    //             println!("other");
    //             validate_flag.increment_invalid_field_name();
    //         }
    //     }
    // }
    // //validate_check
    // validate_flag.check_is_ok()?;
    // //register tag_vec
    // register_data.tag = result_tag_vec.clone();
    // println!("{:#?}", register_data);
    //////////////////////////////////////////

    //insert object table
    let object_model = object::ActiveModel {
        name: Set(register_data.name),
        photo_url: Set(format!("{}/{}", r2_url, Uuid::new_v4())),
        mime_type: Set(register_data.mime_type.clone()),
        license: Set(register_data.license),
        description: Set(register_data.description),
        created_at: Set(Utc::now().naive_local()),
        updated_at: Set(Utc::now().naive_local()),
        ..Default::default()
    };
    let object_insert_result = Object::insert(object_model).exec(&db).await?;
    let object_model = Object::find_by_id(object_insert_result.last_insert_id)
        .one(&db)
        .await?
        .ok_or(AppError(anyhow::anyhow!("Object was not found.")))?;
    let mut object_active_model: object::ActiveModel = object_model.into();
    object_active_model.photo_url = Set(format!(
        "{}/obj-{}.{}",
        r2_url,
        object_insert_result.last_insert_id,
        server::MimeType::to_extension(register_data.mime_type)?
    ));
    object_active_model.update(&db).await?;
    //insert tag table
    if !register_data.tag.is_empty() {
        for tag_name in register_data.tag {
            //insert tag table
            let check_tag_is_exist = Tag::find()
                .filter(tag::Column::Name.eq(tag_name.clone()))
                .one(&db)
                .await?;
            match check_tag_is_exist {
                Some(_) => {}
                None => {
                    let tag_model = tag::ActiveModel {
                        name: Set(tag_name.clone()),
                        ..Default::default()
                    };
                    println!("{:#?}", tag_model);
                    let _ = Tag::insert(tag_model).exec(&db).await?;
                }
            }
            //get tag table
            let tag_id = Tag::find()
                .filter(tag::Column::Name.eq(tag_name))
                .one(&db)
                .await?
                .ok_or(AppError(anyhow::anyhow!("Tag was not found.")))?
                .id;
            //insert object_tag_junction table
            let object_tag_junction_model = object_tag_junction::ActiveModel {
                object_id: Set(object_insert_result.last_insert_id),
                tag_id: Set(tag_id),
                ..Default::default()
            };
            let _ = ObjectTagJunction::insert(object_tag_junction_model)
                .exec(&db)
                .await?;
        }
    }
    let object_model = Object::find_by_id(object_insert_result.last_insert_id)
        .one(&db)
        .await?
        .ok_or(AppError(anyhow::anyhow!("Object was not found.")))?;
    let tag_ids = ObjectTagJunction::find()
        .filter(object_tag_junction::Column::ObjectId.eq(object_model.id))
        .all(&db)
        .await?
        .iter()
        .map(|object_tag_junction| object_tag_junction.tag_id)
        .collect::<Vec<i32>>();
    let tag_names = {
        let mut tag_names = Vec::new();
        for tag_id in tag_ids {
            let tag_name = Tag::find_by_id(tag_id)
                .one(&db)
                .await?
                .ok_or(AppError(anyhow::anyhow!("Tag was not found.")))?
                .name;
            tag_names.push(tag_name);
        }
        tag_names
    };
    //insert meilisearch
    let meilisearch_object_data = server::MeiliSearchObjectData {
        id: object_model.id,
        name: object_model.name.clone(),
        photo_url: object_model.photo_url.clone(),
        mime_type: object_model.mime_type.clone(),
        license: object_model.license.clone(),
        tag: tag_names.clone(),
        description: object_model.description.clone(),
        created_at: object_model.created_at.to_string(),
        updated_at: object_model.updated_at.to_string(),
    };
    let _ = meilisearch_client
        .index("object")
        .add_documents(&[meilisearch_object_data], Some("id"))
        .await
        .unwrap();
    //return
    let object = server::ObjectData {
        id: object_model.id,
        name: object_model.name,
        photo_url: object_model.photo_url,
        mime_type: object_model.mime_type,
        license: object_model.license,
        tag: tag_names,
        description: object_model.description,
        created_at: object_model.created_at.to_string(),
        updated_at: object_model.updated_at.to_string(),
    };
    Ok(Json(object))
}

//* update *//
#[axum::debug_handler]
#[utoipa::path(
    put,
    path = "/api/object/update/{id}",
    params(("id", Path, description = "set update item id")),
    request_body(content = UpdateObjectData, description = "set update item data"),
    responses(
        (status = 200, description = "OK", body = ObjectData),
    ),
    tag = "Object",
)]
pub async fn update_object_put(
    Path(id): Path<i32>,
    Extension(db): Extension<DatabaseConnection>,
    Extension(meilisearch_client): Extension<meilisearch_sdk::client::Client>,
    Json(update_data): Json<server::UpdateObjectData>,
) -> Result<Json<server::ObjectData>, AppError> {
    ////////////////////////////////////////
    // //validate_flag
    // let mut validate_flag = server::UpdateObjectFieldCountFlags::initialize();
    // //connectorのvector
    // let mut result_tag_vec: Vec<String> = Vec::new();
    // let mut register_data = server::UpdateObjectData {
    //     name: "".to_string(),
    //     license: "".to_string(),
    //     tag: result_tag_vec.clone(),
    //     description: "".to_string(),
    // };
    // while let Some(field) = multipart.next_field().await? {
    //     let field_name = field.name().unwrap().to_string();
    //     println!("field name: {}", field_name);
    //     //connector
    //     if field_name.starts_with("tag") {
    //         let tag = field.text().await?;
    //         println!("tag: {}", tag);
    //         result_tag_vec.push(tag);
    //         continue;
    //     }
    //     match field_name.as_str() {
    //         "name" => {
    //             let name = field.text().await?;
    //             println!("name: {}", name);
    //             validate_flag.increment_name_and_name_length(&name);
    //             register_data.name = name;
    //         }
    //         "license" => {
    //             let license = field.text().await?;
    //             println!("license: {}", license);
    //             validate_flag.increment_license();
    //             register_data.license = license;
    //         }
    //         "description" => {
    //             let description = field.text().await?;
    //             println!("description: {}", description);
    //             validate_flag.increment_description();
    //             register_data.description = description;
    //         }
    //         _ => {
    //             println!("other");
    //             validate_flag.increment_invalid_field_name();
    //         }
    //     }
    // }
    // validate_flag.check_is_ok()?;
    // //register tag_vec
    // register_data.tag = result_tag_vec.clone();
    // println!("{:#?}", register_data);
    /////////////////////////////////////////
    //update object table
    let object_model = Object::find_by_id(id)
        .one(&db)
        .await?
        .ok_or(AppError(anyhow::anyhow!("Object was not found.")))?;
    let mut object_active_model: object::ActiveModel = object_model.into();
    object_active_model.name = Set(update_data.name);
    object_active_model.license = Set(update_data.license);
    object_active_model.description = Set(update_data.description);
    object_active_model.updated_at = Set(Utc::now().naive_local());
    object_active_model.update(&db).await?;
    //update tag table
    //clear object_tag_junction table
    let _ = ObjectTagJunction::delete_many()
        .filter(object_tag_junction::Column::ObjectId.eq(id))
        .exec(&db)
        .await?;
    if !update_data.tag.is_empty() {
        for tag_name in update_data.tag {
            //insert tag table
            let check_tag_is_exist = Tag::find()
                .filter(tag::Column::Name.eq(tag_name.clone()))
                .one(&db)
                .await?;
            match check_tag_is_exist {
                Some(_) => {}
                None => {
                    let tag_model = tag::ActiveModel {
                        name: Set(tag_name.clone()),
                        ..Default::default()
                    };
                    println!("{:#?}", tag_model);
                    let _ = Tag::insert(tag_model).exec(&db).await?;
                }
            }
            //get tag table
            let tag_id = Tag::find()
                .filter(tag::Column::Name.eq(tag_name))
                .one(&db)
                .await?
                .ok_or(AppError(anyhow::anyhow!("Tag was not found.")))?
                .id;
            //insert object_tag_junction table
            let object_tag_junction_model = object_tag_junction::ActiveModel {
                object_id: Set(id),
                tag_id: Set(tag_id),
                ..Default::default()
            };
            let _ = ObjectTagJunction::insert(object_tag_junction_model)
                .exec(&db)
                .await?;
        }
    }
    let object_model = Object::find_by_id(id)
        .one(&db)
        .await?
        .ok_or(AppError(anyhow::anyhow!("Object was not found.")))?;
    let tag_ids = ObjectTagJunction::find()
        .filter(object_tag_junction::Column::ObjectId.eq(object_model.id))
        .all(&db)
        .await?
        .iter()
        .map(|object_tag_junction| object_tag_junction.tag_id)
        .collect::<Vec<i32>>();
    let tag_names = {
        let mut tag_names = Vec::new();
        for tag_id in tag_ids {
            let tag_name = Tag::find_by_id(tag_id)
                .one(&db)
                .await?
                .ok_or(AppError(anyhow::anyhow!("Tag was not found.")))?
                .name;
            tag_names.push(tag_name);
        }
        tag_names
    };
    //insert meilisearch
    let meilisearch_object_data = server::MeiliSearchObjectData {
        id: object_model.id,
        name: object_model.name.clone(),
        photo_url: object_model.photo_url.clone(),
        mime_type: object_model.mime_type.clone(),
        license: object_model.license.clone(),
        tag: tag_names.clone(),
        description: object_model.description.clone(),
        created_at: object_model.created_at.to_string(),
        updated_at: object_model.updated_at.to_string(),
    };
    let _ = meilisearch_client
        .index("object")
        .add_documents(&[meilisearch_object_data], Some("id"))
        .await
        .unwrap();
    //return
    let object = server::ObjectData {
        id: object_model.id,
        name: object_model.name,
        photo_url: object_model.photo_url,
        mime_type: object_model.mime_type,
        license: object_model.license,
        tag: tag_names,
        description: object_model.description,
        created_at: object_model.created_at.to_string(),
        updated_at: object_model.updated_at.to_string(),
    };
    Ok(Json(object))
}

//* delete *//
#[utoipa::path(
    delete,
    path = "/api/object/delete/{id}",
    params(("id", Path, description = "set update item id")),
    responses(
        (status = 200, description = "OK", body = DeleteObjectData),
    ),
    tag = "Object",
)]
#[axum::debug_handler]
pub async fn delete_object_delete(
    Path(id): Path<i32>,
    Extension(db): Extension<DatabaseConnection>,
    Extension(meilisearch_client): Extension<meilisearch_sdk::client::Client>,
) -> Result<Json<server::DeleteObjectData>, AppError> {
    let _ = ObjectTagJunction::delete_many()
        .filter(object_tag_junction::Column::ObjectId.eq(id))
        .exec(&db)
        .await?;
    let _ = Object::delete_by_id(id).exec(&db).await?;
    let _ = meilisearch_client
        .index("object")
        .delete_document(id)
        .await
        .unwrap();
    Ok(server::DeleteObjectData::generate())
}
