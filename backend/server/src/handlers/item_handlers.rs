use ::entity::{
    grand_parent_label_junction::{self, Entity as GrandParentLabelJunction},
    item::{self, Entity as Item, Record},
    label::{self, Entity as Label},
    parent_label_junction::{self, Entity as ParentLabelJunction},
};
use axum::{
    extract::{Multipart, Path, Query},
    Extension, Json,
};
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, LoaderTrait, QueryFilter, Set,
};
use serde_json::json;
use server::AppError;
use std::collections::HashMap;

//search with Meilisearch
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

//get one item
pub async fn get_each_item_get(
    Path(id): Path<i32>,
    Extension(db): Extension<DatabaseConnection>,
) -> Result<Json<server::ItemData>, AppError> {
    let item_model = Item::find_by_id(id)
        .one(&db)
        .await?
        .ok_or(AppError(anyhow::anyhow!("Item not found in Item Table")))?;
    //visible_id
    let label_model: label::Model = Label::find_by_id(item_model.label_id)
        .one(&db)
        .await?
        .ok_or(AppError(anyhow::anyhow!(
            "Item was not found in Label Table."
        )))?;

    let parent_label_junction_model: parent_label_junction::Model =
        ParentLabelJunction::find_by_id(item_model.parent_label_id)
            .one(&db)
            .await?
            .ok_or(AppError(anyhow::anyhow!(
                "Parent item was not found in Parent Label Junction Table."
            )))?;
    //parent_visible_id
    let parent_label_model: label::Model = Label::find_by_id(parent_label_junction_model.label_id)
        .one(&db)
        .await?
        .ok_or(AppError(anyhow::anyhow!(
            "Parent item was not found in Label Table"
        )))?;

    let grand_parent_label_junction_model: grand_parent_label_junction::Model =
        GrandParentLabelJunction::find_by_id(item_model.grand_parent_label_id)
            .one(&db)
            .await?
            .ok_or(AppError(anyhow::anyhow!(
                "Item was not found in Grand Parent Label Junction Table."
            )))?;
    //grand_parent_visible_id
    let grand_parent_label_model: label::Model =
        Label::find_by_id(grand_parent_label_junction_model.label_id)
            .one(&db)
            .await?
            .ok_or(AppError(anyhow::anyhow!(
                "Item was not found in Label Table."
            )))?;

    //get item path
    let mut path_item_name: Vec<String> = Vec::new();
    let mut path: Vec<String> = Vec::new();
    let mut current_item = label_model.visible_id.clone();
    let mut parent_item = parent_label_model.visible_id.clone();
    loop {
        if label_model.visible_id == parent_label_model.visible_id {
            //ルートのitemの場合
            path.push(label_model.visible_id.clone());
            let item_model = Item::find()
                .filter(item::Column::LabelId.eq(label_model.id))
                .one(&db)
                .await?
                .ok_or(AppError(anyhow::anyhow!(
                    "Item was not found in Item Table."
                )))?;
            path_item_name.push(item_model.name.clone());
            break;
        } else if parent_label_model.visible_id == grand_parent_label_model.visible_id {
            //親がルートの場合
            path.push(label_model.visible_id.clone());
            path.push(parent_label_model.visible_id.clone());
            let item_model = Item::find()
                .filter(item::Column::LabelId.eq(label_model.id))
                .one(&db)
                .await?
                .ok_or(AppError(anyhow::anyhow!(
                    "Item was not found in Item Table."
                )))?;
            path_item_name.push(item_model.name.clone());
            let parent_item_model = Item::find()
                .filter(item::Column::LabelId.eq(parent_label_model.id))
                .one(&db)
                .await?
                .ok_or(AppError(anyhow::anyhow!(
                    "Item was not found in Item Table."
                )))?;
            path_item_name.push(parent_item_model.name.clone());
            break;
        } else if current_item == parent_item {
            //それ以外の場合の終了条件
            let current_item_label_model: label::Model = Label::find()
                .filter(label::Column::VisibleId.eq(&current_item))
                .one(&db)
                .await?
                .ok_or(AppError(anyhow::anyhow!(
                    "Item was not found in Item Table."
                )))?;
            let current_item_model: item::Model = Item::find()
                .filter(item::Column::LabelId.eq(current_item_label_model.id))
                .one(&db)
                .await?
                .ok_or(AppError(anyhow::anyhow!(
                    "Item was not found in Item Table."
                )))?;
            path_item_name.push(current_item_model.name.clone());
            path.push(current_item);
            break;
        }
        let current_item_label_model: label::Model = Label::find()
            .filter(label::Column::VisibleId.eq(&current_item))
            .one(&db)
            .await?
            .ok_or(AppError(anyhow::anyhow!(
                "Item was not found in Item Table."
            )))?;
        let current_item_model: item::Model = Item::find()
            .filter(item::Column::LabelId.eq(current_item_label_model.id))
            .one(&db)
            .await?
            .ok_or(AppError(anyhow::anyhow!(
                "Item was not found in Item Table."
            )))?;
        path_item_name.push(current_item_model.name.clone());
        path.push(current_item);
        // current_item, parent_itemの更新
        //current_itemの更新
        current_item = parent_item.clone();
        //parent_current_itemの更新
        let old_parent_label_model: label::Model = Label::find()
            .filter(label::Column::VisibleId.eq(&parent_item))
            .one(&db)
            .await?
            .ok_or(AppError(anyhow::anyhow!(
                "Item was not found in Label Table."
            )))?;
        let old_parent_item_model: item::Model = Item::find()
            .filter(item::Column::LabelId.eq(old_parent_label_model.id))
            .one(&db)
            .await?
            .ok_or(AppError(anyhow::anyhow!(
                "Item was not found in Item Table."
            )))?;
        let new_parent_label_junction_model: parent_label_junction::Model =
            ParentLabelJunction::find()
                .filter(parent_label_junction::Column::Id.eq(old_parent_item_model.parent_label_id))
                .one(&db)
                .await?
                .ok_or(AppError(anyhow::anyhow!(
                    "Item was not found in Parent Label Junction Table."
                )))?;
        let new_parent_label_model: label::Model = Label::find()
            .filter(label::Column::Id.eq(new_parent_label_junction_model.label_id))
            .one(&db)
            .await?
            .ok_or(AppError(anyhow::anyhow!(
                "Item was not found in Label Table."
            )))?;
        parent_item = new_parent_label_model.visible_id.clone();
    }

    let item = server::ItemData {
        id: item_model.id,
        visible_id: label_model.visible_id,
        parent_visible_id: parent_label_model.visible_id,
        grand_parent_visible_id: grand_parent_label_model.visible_id,
        name: item_model.name,
        product_number: item_model.product_number,
        photo_url: item_model.photo_url,
        record: item_model.record,
        color: label_model.color,
        description: item_model.description,
        year_purchased: item_model.year_purchased,
        connector: item_model.connector,
        created_at: item_model.created_at,
        updated_at: item_model.updated_at,
        path: json!(path),
        path_item_name: json!(path_item_name),
    };

    Ok(Json(item))
}

// //* update *//
pub async fn update_item_put(
    Path(id): Path<i32>,
    Extension(db): Extension<DatabaseConnection>,
    Extension(meilisearch_client): Extension<meilisearch_sdk::client::Client>,
    Extension(reqwest_client): Extension<reqwest::Client>,
    Extension(meilisearch_admin_api_key): Extension<String>,
    Extension(meilisearch_url): Extension<String>,
    mut multipart: Multipart,
) -> Result<Json<server::MeiliSearchItemData>, AppError> {
    //parent_visible_idに変更があるかを確認するためのflag
    let mut is_chaged_parennt_visible_id_flag = false;
    //visible_idが変更されているかどうかの確認するためのflag
    let mut is_changed_visible_id_flag = false;
    //存在しないfield_nameがないか確認するためのflag
    let mut have_invalid_field_name_flag = false;
    //connectorのvector
    let mut result_connector_vec: Vec<String> = Vec::new();
    let mut update_data = server::ControlItemData {
        visible_id: "".to_string(),
        parent_visible_id: "".to_string(),
        name: "".to_string(),
        product_number: "".to_string(),
        record: Record::Qr,
        description: "".to_string(),
        year_purchased: None,
        connector: json!(result_connector_vec),
    };
    while let Some(field) = multipart.next_field().await? {
        let field_name = field.name().unwrap().to_string();
        println!("field name: {}", field_name);
        //connector
        if field_name.starts_with("connector") {
            let connector = field.text().await?;
            println!("connector: {}", connector);
            result_connector_vec.push(connector);
            continue;
        }
        match field_name.as_str() {
            "visible_id" => {
                let visible_id = field.text().await?;
                println!("visible_id: {}", visible_id);
                //とりあえず格納する
                update_data.visible_id = visible_id;
            }
            "parent_visible_id" => {
                let parent_id = field.text().await?;
                println!("parent_visible_id: {}", parent_id);
                //とりあえず格納する
                update_data.parent_visible_id = parent_id;
            }
            "name" => {
                let name = field.text().await?;
                println!("name: {}", name);
                update_data.name = name;
            }
            "product_number" => {
                let product_number = field.text().await?;
                println!("product_number: {}", product_number);
                update_data.product_number = product_number;
            }
            "record" => {
                let record = field.text().await?;
                println!("record: {}", record);
                //Recordに不正な値が入っている場合の早期リターン
                if record != "Qr" && record != "Barcode" && record != "Nothing" {
                    return Err(AppError(anyhow::anyhow!(
                        "Record type '{}' is invalid",
                        record
                    )));
                }
                update_data.record = match record.as_str() {
                    "Qr" => Record::Qr,
                    "Barcode" => Record::Barcode,
                    "Nothing" => Record::Nothing,
                    _ => panic!("Record type validation was failed"),
                };
            }
            "description" => {
                let description = field.text().await?;
                println!("description: {}", description);
                update_data.description = description;
            }
            "year_purchased" => {
                let year_purchased = field.text().await?;
                println!("year_purchased: {}", year_purchased);
                if year_purchased.is_empty() {
                    update_data.year_purchased = None;
                } else {
                    update_data.year_purchased = Some(year_purchased.parse::<i32>()?);
                }
            }
            _ => {
                println!("other");
                have_invalid_field_name_flag = true;
            }
        }
    }
    //存在しないfieldを取得した場合の早期リターン
    if have_invalid_field_name_flag {
        return Err(AppError(anyhow::anyhow!("Invalid field name")));
    }
    //更新対象の物品のデータ取得
    let update_item_model: item::Model = Item::find_by_id(id)
        .one(&db)
        .await?
        .ok_or(AppError(anyhow::anyhow!("Item not found")))?;
    //FormDataのvisible_idからLabel Tableのidを取得
    //※更新するvisible_idが存在するかどうかの確認
    let new_label_model: label::Model = Label::find()
        .filter(label::Column::VisibleId.eq(&update_data.visible_id))
        .one(&db)
        .await?
        .ok_or(AppError(anyhow::anyhow!("Invaild visible_id")))?;
    //visible_idが変化しているかどうかをチェック
    let old_label_model: label::Model = vec![update_item_model.clone()]
        .load_one(Label, &db)
        .await?
        .first()
        .cloned()
        .ok_or(AppError(anyhow::anyhow!("Visible Id was not found")))?
        .ok_or(AppError(anyhow::anyhow!("Visible Id was not found")))?;
    if old_label_model.visible_id != new_label_model.visible_id {
        //visible_idが存在するかどうかの確認
        Label::find()
            .filter(label::Column::VisibleId.eq(&update_data.visible_id))
            .one(&db)
            .await?
            .ok_or(AppError(anyhow::anyhow!("Invaild visible_id")))?;
        is_changed_visible_id_flag = true;
    }
    //parent_visible_idが変化しているかどうかをチェック
    let new_parent_label_model: label::Model = Label::find()
        .filter(label::Column::VisibleId.eq(&update_data.parent_visible_id))
        .one(&db)
        .await?
        .ok_or(AppError(anyhow::anyhow!("Invaild parent_visible_id")))?;
    let old_parent_label_junction_model: parent_label_junction::Model =
        vec![update_item_model.clone()]
            .load_one(ParentLabelJunction, &db)
            .await?
            .first()
            .cloned()
            .ok_or(AppError(anyhow::anyhow!("Parent Visible Id was not found")))?
            .ok_or(AppError(anyhow::anyhow!("Parent Visible Id was not found")))?;
    let old_parent_label_model: label::Model = vec![old_parent_label_junction_model.clone()]
        .load_one(Label, &db)
        .await?
        .first()
        .cloned()
        .ok_or(AppError(anyhow::anyhow!("Parent Visible Id was not found")))?
        .ok_or(AppError(anyhow::anyhow!("Parent Visible Id was not found")))?;
    if new_parent_label_model.visible_id != old_parent_label_model.visible_id {
        is_chaged_parennt_visible_id_flag = true;
    }
    //* validation */
    //parent_visible_idがその物品の子孫の子になっていないかのチェック
    let mut descendant_item_labels_vec: Vec<i32> = Vec::new();
    let mut children_item_labels_vec: Vec<i32> = Vec::new();
    let mut grandchild_item_labels_vec: Vec<i32> = Vec::new();
    let mut parent_label_junction_ids_vec = ParentLabelJunction::find()
        .filter(parent_label_junction::Column::LabelId.eq(update_item_model.label_id))
        .all(&db)
        .await?
        .iter()
        .map(|parent_label_junction| parent_label_junction.id)
        .collect::<Vec<i32>>();
    let mut grand_parent_label_junction_ids_vec = GrandParentLabelJunction::find()
        .filter(grand_parent_label_junction::Column::LabelId.eq(update_item_model.label_id))
        .all(&db)
        .await?
        .iter()
        .map(|grand_parent_label_junction| grand_parent_label_junction.id)
        .collect::<Vec<i32>>();
    for parent_label_id in parent_label_junction_ids_vec.clone() {
        let label_id = Item::find()
            .filter(item::Column::ParentLabelId.eq(parent_label_id))
            .one(&db)
            .await?
            .ok_or(AppError(anyhow::anyhow!("Item not found")))?
            .label_id;
        children_item_labels_vec.push(label_id);
        descendant_item_labels_vec.push(label_id);
    }
    for grand_parent_label_id in grand_parent_label_junction_ids_vec.clone() {
        let label_id = Item::find()
            .filter(item::Column::GrandParentLabelId.eq(grand_parent_label_id))
            .one(&db)
            .await?
            .ok_or(AppError(anyhow::anyhow!("Item not found")))?
            .label_id;
        grandchild_item_labels_vec.push(label_id);
        descendant_item_labels_vec.push(label_id);
    }
    //visible_id・parent_visible_idの変更がある場合に利用するvector
    let update_children_item_labels_vec = children_item_labels_vec.clone();
    let update_grandchild_item_labels_vec = grandchild_item_labels_vec.clone();
    ///////////////////////////////////////////////////////////////////////
    println!(
        "parent_label_junction_ids_vec: {:?}",
        parent_label_junction_ids_vec
    );
    println!(
        "grand_parent_label_junction_ids_vec: {:?}",
        grand_parent_label_junction_ids_vec
    );
    println!("children_item_labels_vec: {:?}", children_item_labels_vec);
    println!(
        "grandchild_item_labels_vec: {:?}",
        grandchild_item_labels_vec
    );
    ///////////////////////////////////////////////////////////////////////
    //更新対象の物品の全子孫のlabel_idを取得
    //descendant_item_labels_vecに全子孫のlabel_idを格納
    loop {
        if children_item_labels_vec.is_empty() || grandchild_item_labels_vec.is_empty() {
            descendant_item_labels_vec.append(&mut grandchild_item_labels_vec.clone());
            break;
        }
        //子孫の処理
        for descendant_item_label_id in descendant_item_labels_vec.clone() {
            parent_label_junction_ids_vec.append(
                &mut ParentLabelJunction::find()
                    .filter(parent_label_junction::Column::LabelId.eq(descendant_item_label_id))
                    .all(&db)
                    .await?
                    .iter()
                    .map(|parent_label_junction| parent_label_junction.id)
                    .collect::<Vec<i32>>(),
            );
            grand_parent_label_junction_ids_vec.append(
                &mut GrandParentLabelJunction::find()
                    .filter(
                        grand_parent_label_junction::Column::LabelId.eq(descendant_item_label_id),
                    )
                    .all(&db)
                    .await?
                    .iter()
                    .map(|grand_parent_label_junction| grand_parent_label_junction.id)
                    .collect::<Vec<i32>>(),
            );
            for parent_label_id in parent_label_junction_ids_vec.clone() {
                let label_id = Item::find()
                    .filter(item::Column::ParentLabelId.eq(parent_label_id))
                    .one(&db)
                    .await?
                    .ok_or(AppError(anyhow::anyhow!("Item not found")))?
                    .label_id;
                children_item_labels_vec.push(label_id);
                descendant_item_labels_vec.push(label_id);
                //meilisearchの更新 (parent_visible_idを更新)
            }
            for grand_parent_label_id in grand_parent_label_junction_ids_vec.clone() {
                let label_id = Item::find()
                    .filter(item::Column::GrandParentLabelId.eq(grand_parent_label_id))
                    .one(&db)
                    .await?
                    .ok_or(AppError(anyhow::anyhow!("Item not found")))?
                    .label_id;
                grandchild_item_labels_vec.push(label_id);
                descendant_item_labels_vec.push(label_id);
                //meilisearchの更新 (visible_idを更新)
            }
        }
        parent_label_junction_ids_vec = vec![];
        grand_parent_label_junction_ids_vec = vec![];
        children_item_labels_vec = vec![];
        grandchild_item_labels_vec = vec![];
    }
    //更新する物品IDの親物品が子孫の子になっていないかのチェック
    for label_id in descendant_item_labels_vec.clone() {
        let parent_label_id = Label::find()
            .filter(label::Column::VisibleId.eq(&update_data.parent_visible_id))
            .one(&db)
            .await?
            .ok_or(AppError(anyhow::anyhow!("Item not found")))?
            .id;
        if label_id == parent_label_id {
            return Err(AppError(anyhow::anyhow!(
                "The visible id ({}) is a descendant of the one of the descendant visible id ({})",
                &update_data.visible_id,
                Label::find()
                    .filter(label::Column::Id.eq(label_id))
                    .one(&db)
                    .await?
                    .ok_or(AppError(anyhow::anyhow!("Item not found")))?
                    .visible_id,
            )));
        }
    }
    //parent_visible_idが変更されている場合の処理
    if is_chaged_parennt_visible_id_flag {
        //子物品・孫物品の更新
        for label_id in update_children_item_labels_vec.clone() {
            //parent_visible_idが変更されている場合の処理 (parent_visible_idとvisibleid、またはparent_visible_idのみが変更されている場合の処理)
            let item_model: item::Model = Item::find()
                .filter(item::Column::LabelId.eq(label_id))
                .one(&db)
                .await?
                .ok_or(AppError(anyhow::anyhow!("Item not found")))?;
            let grand_parent_label_junction_model = vec![item_model.clone()]
                .load_one(GrandParentLabelJunction, &db)
                .await?
                .first()
                .cloned()
                .ok_or(AppError(anyhow::anyhow!(
                    "Grand parent label junction model was not found"
                )))?
                .ok_or(AppError(anyhow::anyhow!(
                    "Grand parent label junction model was not found"
                )))?;
            let mut grand_parent_label_junction_model: grand_parent_label_junction::ActiveModel =
                grand_parent_label_junction_model.into();
            grand_parent_label_junction_model.label_id = Set(new_parent_label_model.id);
            grand_parent_label_junction_model.update(&db).await?;
            //meilisearchの更新
            let url = format!("{}/indexes/item/documents/{}", meilisearch_url, id);
            let mut meilisearch_item_data = reqwest_client
                .get(&url)
                .bearer_auth(&meilisearch_admin_api_key)
                .send()
                .await?
                .json::<server::MeiliSearchItemData>()
                .await?;
            meilisearch_item_data.grand_parent_visible_id =
                new_parent_label_model.visible_id.clone();
            let item_meilisearch = meilisearch_client
                .index("item")
                .add_documents(&vec![meilisearch_item_data], Some("id"))
                .await
                .unwrap();
            println!(
                "[INFO]: update MeiliSearch data of a child item ({}) result:  {:#?}",
                new_label_model.visible_id.clone(),
                item_meilisearch
            );
        }
        let parent_label_model = Label::find()
            .filter(label::Column::VisibleId.eq(&update_data.parent_visible_id))
            .one(&db)
            .await?
            .ok_or(AppError(anyhow::anyhow!(
                "Parent Label model was not found"
            )))?;
        //parent_label_idの取得
        let parent_label_id = parent_label_model.id;
        let parent_item_model = Item::find()
            .filter(item::Column::LabelId.eq(parent_label_id))
            .one(&db)
            .await?
            .ok_or(AppError(anyhow::anyhow!("Parent Item model was not found")))?;
        //parent_idの取得
        let parent_id = parent_item_model.id;
        //grand_parent_label_idの取得
        let grand_parent_label_id = Label::find()
            .filter(label::Column::Id.eq(parent_item_model.parent_label_id))
            .one(&db)
            .await?
            .ok_or(AppError(anyhow::anyhow!(
                "Grand parent label id was not found"
            )))?
            .id;
        //grand_parent_idの取得
        let grand_parent_id = Item::find()
            .filter(item::Column::LabelId.eq(grand_parent_label_id))
            .one(&db)
            .await?
            .ok_or(AppError(anyhow::anyhow!(
                "Grand parent item model was not found"
            )))?
            .id;
        //visible_idが変更されている場合の処理 (parent_visible_idとvisibleidが変更されている場合の処理)
        if is_changed_visible_id_flag {
            //子物品・孫物品の更新
            for label_id in update_children_item_labels_vec.clone() {
                let item_model: item::Model = Item::find()
                    .filter(item::Column::LabelId.eq(label_id))
                    .one(&db)
                    .await?
                    .ok_or(AppError(anyhow::anyhow!("Item not found")))?;
                let parent_label_junction_model = vec![item_model.clone()]
                    .load_one(ParentLabelJunction, &db)
                    .await?
                    .first()
                    .cloned()
                    .ok_or(AppError(anyhow::anyhow!(
                        "Parent label junction model was not found"
                    )))?
                    .ok_or(AppError(anyhow::anyhow!(
                        "Parent label junction model was not found"
                    )))?;
                let mut parent_label_junction_model: parent_label_junction::ActiveModel =
                    parent_label_junction_model.into();
                parent_label_junction_model.label_id = Set(new_label_model.id);
                parent_label_junction_model.update(&db).await?;
                //meilisearchの更新
                let url = format!("{}/indexes/item/documents/{}", meilisearch_url, id);
                let mut meilisearch_item_data = reqwest_client
                    .get(&url)
                    .bearer_auth(&meilisearch_admin_api_key)
                    .send()
                    .await?
                    .json::<server::MeiliSearchItemData>()
                    .await?;
                meilisearch_item_data.parent_visible_id = new_parent_label_model.visible_id.clone();
                let item_meilisearch = meilisearch_client
                    .index("item")
                    .add_documents(&vec![meilisearch_item_data], Some("id"))
                    .await
                    .unwrap();
                println!(
                    "[INFO]: update MeiliSearch data of a child item ({}) result:  {:#?}",
                    new_label_model.visible_id.clone(),
                    item_meilisearch
                );
            }
            for label_id in update_grandchild_item_labels_vec.clone() {
                let item_model: item::Model = Item::find()
                    .filter(item::Column::LabelId.eq(label_id))
                    .one(&db)
                    .await?
                    .ok_or(AppError(anyhow::anyhow!("Item not found")))?;
                let grand_parent_label_junction_model = vec![item_model.clone()]
                    .load_one(GrandParentLabelJunction, &db)
                    .await?
                    .first()
                    .cloned()
                    .ok_or(AppError(anyhow::anyhow!(
                        "Grand parent label junction model was not found"
                    )))?
                    .ok_or(AppError(anyhow::anyhow!(
                        "Grand parent label junction model was not found"
                    )))?;
                let mut grand_parent_label_junction_model: grand_parent_label_junction::ActiveModel =
                    grand_parent_label_junction_model.into();
                grand_parent_label_junction_model.label_id = Set(new_label_model.id);
                grand_parent_label_junction_model.update(&db).await?;
                //meilisearchの更新
                let url = format!("{}/indexes/item/documents/{}", meilisearch_url, id);
                let mut meilisearch_item_data = reqwest_client
                    .get(&url)
                    .bearer_auth(&meilisearch_admin_api_key)
                    .send()
                    .await?
                    .json::<server::MeiliSearchItemData>()
                    .await?;
                meilisearch_item_data.grand_parent_visible_id =
                    new_parent_label_model.visible_id.clone();
                let item_meilisearch = meilisearch_client
                    .index("item")
                    .add_documents(&vec![meilisearch_item_data], Some("id"))
                    .await
                    .unwrap();
                println!(
                    "[INFO]: update MeiliSearch data of a child item ({}) result:  {:#?}",
                    new_label_model.visible_id.clone(),
                    item_meilisearch
                );
            }
            //update処理
            let mut update_item_model: item::ActiveModel = Item::find()
                .filter(item::Column::Id.eq(id))
                .one(&db)
                .await?
                .ok_or(AppError(anyhow::anyhow!("Item not found")))?
                .into();
            update_item_model.label_id = Set(new_label_model.id);
            update_item_model.parent_id = Set(parent_id);
            update_item_model.parent_label_id = Set(parent_label_id);
            update_item_model.grand_parent_id = Set(grand_parent_id);
            update_item_model.grand_parent_label_id = Set(grand_parent_label_id);
            update_item_model.name = Set(update_data.name);
            update_item_model.product_number = Set(update_data.product_number);
            update_item_model.record = Set(update_data.record);
            update_item_model.description = Set(update_data.description);
            update_item_model.year_purchased = Set(update_data.year_purchased);
            update_item_model.connector = Set(update_data.connector);
            update_item_model.updated_at = Set(Utc::now().naive_utc());
            update_item_model.update(&db).await?;
            //update meilisearch data
            let update_item_model = Item::find()
                .filter(item::Column::Id.eq(id))
                .one(&db)
                .await?
                .ok_or(AppError(anyhow::anyhow!("Item not found")))?;
            let update_label_model = Label::find()
                .filter(label::Column::Id.eq(update_item_model.label_id))
                .one(&db)
                .await?
                .ok_or(AppError(anyhow::anyhow!("Label not found")))?;
            let update_parent_label_model = Label::find()
                .filter(label::Column::Id.eq(update_item_model.parent_label_id))
                .one(&db)
                .await?
                .ok_or(AppError(anyhow::anyhow!("Label not found")))?;
            let update_grand_parent_label_model = Label::find()
                .filter(label::Column::Id.eq(update_item_model.grand_parent_label_id))
                .one(&db)
                .await?
                .ok_or(AppError(anyhow::anyhow!("Label not found")))?;
            let meilisearch_update_item: server::MeiliSearchItemData =
                server::MeiliSearchItemData {
                    id,
                    visible_id: update_label_model.visible_id,
                    parent_visible_id: update_parent_label_model.visible_id,
                    grand_parent_visible_id: update_grand_parent_label_model.visible_id,
                    name: update_item_model.name,
                    product_number: update_item_model.product_number,
                    photo_url: update_item_model.photo_url,
                    record: update_item_model.record,
                    color: update_label_model.color,
                    description: update_item_model.description,
                    year_purchased: update_item_model.year_purchased,
                    connector: update_item_model.connector,
                    created_at: update_item_model.created_at,
                    updated_at: update_item_model.updated_at,
                };
            let item_meilisearch = meilisearch_client
                .index("item")
                .add_documents(&vec![meilisearch_update_item.clone()], Some("id"))
                .await
                .unwrap();
            println!("[INFO]: MeiliSearch Result {:#?}", item_meilisearch);
            //return
            return Ok(Json(meilisearch_update_item));
        }
        //parent_visible_idが変更されている場合の処理 (parent_visible_idのみが変更されている場合の処理)
        //update処理
        let mut update_item_model: item::ActiveModel = Item::find()
            .filter(item::Column::Id.eq(id))
            .one(&db)
            .await?
            .ok_or(AppError(anyhow::anyhow!("Item not found")))?
            .into();
        update_item_model.parent_id = Set(parent_id);
        update_item_model.parent_label_id = Set(parent_label_id);
        update_item_model.grand_parent_id = Set(grand_parent_id);
        update_item_model.grand_parent_label_id = Set(grand_parent_label_id);
        update_item_model.name = Set(update_data.name);
        update_item_model.product_number = Set(update_data.product_number);
        update_item_model.record = Set(update_data.record);
        update_item_model.description = Set(update_data.description);
        update_item_model.year_purchased = Set(update_data.year_purchased);
        update_item_model.connector = Set(update_data.connector);
        update_item_model.updated_at = Set(Utc::now().naive_utc());
        update_item_model.update(&db).await?;
        //update meilisearch data
        let update_item_model = Item::find()
            .filter(item::Column::Id.eq(id))
            .one(&db)
            .await?
            .ok_or(AppError(anyhow::anyhow!("Item not found")))?;
        let update_label_model = Label::find()
            .filter(label::Column::Id.eq(update_item_model.label_id))
            .one(&db)
            .await?
            .ok_or(AppError(anyhow::anyhow!("Label not found")))?;
        let update_parent_label_model = Label::find()
            .filter(label::Column::Id.eq(update_item_model.parent_label_id))
            .one(&db)
            .await?
            .ok_or(AppError(anyhow::anyhow!("Label not found")))?;
        let update_grand_parent_label_model = Label::find()
            .filter(label::Column::Id.eq(update_item_model.grand_parent_label_id))
            .one(&db)
            .await?
            .ok_or(AppError(anyhow::anyhow!("Label not found")))?;
        let meilisearch_update_item: server::MeiliSearchItemData = server::MeiliSearchItemData {
            id,
            visible_id: update_label_model.visible_id,
            parent_visible_id: update_parent_label_model.visible_id,
            grand_parent_visible_id: update_grand_parent_label_model.visible_id,
            name: update_item_model.name,
            product_number: update_item_model.product_number,
            photo_url: update_item_model.photo_url,
            record: update_item_model.record,
            color: update_label_model.color,
            description: update_item_model.description,
            year_purchased: update_item_model.year_purchased,
            connector: update_item_model.connector,
            created_at: update_item_model.created_at,
            updated_at: update_item_model.updated_at,
        };
        let item_meilisearch = meilisearch_client
            .index("item")
            .add_documents(&vec![meilisearch_update_item.clone()], Some("id"))
            .await
            .unwrap();
        println!("[INFO]: MeiliSearch Result {:#?}", item_meilisearch);
        //return
        return Ok(Json(meilisearch_update_item));
    }
    //visible_idが変更されている場合の処理 (visible_idのみが変更されている場合の処理)
    if is_changed_visible_id_flag {
        //子物品・孫物品の更新
        for label_id in update_children_item_labels_vec.clone() {
            let item_model: item::Model = Item::find()
                .filter(item::Column::LabelId.eq(label_id))
                .one(&db)
                .await?
                .ok_or(AppError(anyhow::anyhow!("Item not found")))?;
            let parent_label_junction_model = vec![item_model.clone()]
                .load_one(ParentLabelJunction, &db)
                .await?
                .first()
                .cloned()
                .ok_or(AppError(anyhow::anyhow!(
                    "Grand parent label junction model was not found"
                )))?
                .ok_or(AppError(anyhow::anyhow!(
                    "Grand parent label junction model was not found"
                )))?;
            let mut parent_label_junction_model: parent_label_junction::ActiveModel =
                parent_label_junction_model.into();
            parent_label_junction_model.label_id = Set(new_label_model.id);
            parent_label_junction_model.update(&db).await?;
            //meilisearchの更新
            let url = format!("{}/indexes/item/documents/{}", meilisearch_url, id);
            let mut meilisearch_item_data = reqwest_client
                .get(&url)
                .bearer_auth(&meilisearch_admin_api_key)
                .send()
                .await?
                .json::<server::MeiliSearchItemData>()
                .await?;
            meilisearch_item_data.parent_visible_id = new_parent_label_model.visible_id.clone();
            let item_meilisearch = meilisearch_client
                .index("item")
                .add_documents(&vec![meilisearch_item_data], Some("id"))
                .await
                .unwrap();
            println!(
                "[INFO]: update MeiliSearch data of a child item ({}) result:  {:#?}",
                new_label_model.visible_id.clone(),
                item_meilisearch
            );
        }
        for label_id in update_grandchild_item_labels_vec.clone() {
            let item_model: item::Model = Item::find()
                .filter(item::Column::LabelId.eq(label_id))
                .one(&db)
                .await?
                .ok_or(AppError(anyhow::anyhow!("Item not found")))?;
            let grand_parent_label_junction_model = vec![item_model.clone()]
                .load_one(GrandParentLabelJunction, &db)
                .await?
                .first()
                .cloned()
                .ok_or(AppError(anyhow::anyhow!(
                    "Grand parent label junction model was not found"
                )))?
                .ok_or(AppError(anyhow::anyhow!(
                    "Grand parent label junction model was not found"
                )))?;
            let mut grand_parent_label_junction_model: grand_parent_label_junction::ActiveModel =
                grand_parent_label_junction_model.into();
            grand_parent_label_junction_model.label_id = Set(new_label_model.id);
            grand_parent_label_junction_model.update(&db).await?;
            //meilisearchの更新
            let url = format!("{}/indexes/item/documents/{}", meilisearch_url, id);
            let mut meilisearch_item_data = reqwest_client
                .get(&url)
                .bearer_auth(&meilisearch_admin_api_key)
                .send()
                .await?
                .json::<server::MeiliSearchItemData>()
                .await?;
            meilisearch_item_data.grand_parent_visible_id =
                new_parent_label_model.visible_id.clone();
            let item_meilisearch = meilisearch_client
                .index("item")
                .add_documents(&vec![meilisearch_item_data], Some("id"))
                .await
                .unwrap();
            println!(
                "[INFO]: update MeiliSearch data of a child item ({}) result:  {:#?}",
                new_label_model.visible_id.clone(),
                item_meilisearch
            );
        }
        //update処理
        let mut update_item_model: item::ActiveModel = Item::find()
            .filter(item::Column::Id.eq(id))
            .one(&db)
            .await?
            .ok_or(AppError(anyhow::anyhow!("Item not found")))?
            .into();
        update_item_model.label_id = Set(new_label_model.id);
        update_item_model.name = Set(update_data.name);
        update_item_model.product_number = Set(update_data.product_number);
        update_item_model.record = Set(update_data.record);
        update_item_model.description = Set(update_data.description);
        update_item_model.year_purchased = Set(update_data.year_purchased);
        update_item_model.connector = Set(update_data.connector);
        update_item_model.updated_at = Set(Utc::now().naive_utc());
        update_item_model.update(&db).await?;
        //update meilisearch data
        let update_item_model = Item::find()
            .filter(item::Column::Id.eq(id))
            .one(&db)
            .await?
            .ok_or(AppError(anyhow::anyhow!("Item not found")))?;
        let update_label_model = Label::find()
            .filter(label::Column::Id.eq(update_item_model.label_id))
            .one(&db)
            .await?
            .ok_or(AppError(anyhow::anyhow!("Label not found")))?;
        let update_parent_label_model = Label::find()
            .filter(label::Column::Id.eq(update_item_model.parent_label_id))
            .one(&db)
            .await?
            .ok_or(AppError(anyhow::anyhow!("Label not found")))?;
        let update_grand_parent_label_model = Label::find()
            .filter(label::Column::Id.eq(update_item_model.grand_parent_label_id))
            .one(&db)
            .await?
            .ok_or(AppError(anyhow::anyhow!("Label not found")))?;
        let meilisearch_update_item: server::MeiliSearchItemData = server::MeiliSearchItemData {
            id,
            visible_id: update_label_model.visible_id,
            parent_visible_id: update_parent_label_model.visible_id,
            grand_parent_visible_id: update_grand_parent_label_model.visible_id,
            name: update_item_model.name,
            product_number: update_item_model.product_number,
            photo_url: update_item_model.photo_url,
            record: update_item_model.record,
            color: update_label_model.color,
            description: update_item_model.description,
            year_purchased: update_item_model.year_purchased,
            connector: update_item_model.connector,
            created_at: update_item_model.created_at,
            updated_at: update_item_model.updated_at,
        };
        let item_meilisearch = meilisearch_client
            .index("item")
            .add_documents(&vec![meilisearch_update_item.clone()], Some("id"))
            .await
            .unwrap();
        println!("[INFO]: MeiliSearch Result {:#?}", item_meilisearch);
        //return
        return Ok(Json(meilisearch_update_item));
    }
    //visible_id・parent_visible_idが変更されていない場合の処理
    //update処理
    let mut update_item_model: item::ActiveModel = Item::find()
        .filter(item::Column::Id.eq(id))
        .one(&db)
        .await?
        .ok_or(AppError(anyhow::anyhow!("Item not found")))?
        .into();
    update_item_model.name = Set(update_data.name);
    update_item_model.product_number = Set(update_data.product_number);
    update_item_model.record = Set(update_data.record);
    update_item_model.description = Set(update_data.description);
    update_item_model.year_purchased = Set(update_data.year_purchased);
    update_item_model.connector = Set(update_data.connector);
    update_item_model.updated_at = Set(Utc::now().naive_utc());

    update_item_model.update(&db).await?;
    //update meilisearch data
    let update_item_model = Item::find()
        .filter(item::Column::Id.eq(id))
        .one(&db)
        .await?
        .ok_or(AppError(anyhow::anyhow!("Item not found")))?;
    let update_label_model = Label::find()
        .filter(label::Column::Id.eq(update_item_model.label_id))
        .one(&db)
        .await?
        .ok_or(AppError(anyhow::anyhow!("Label not found")))?;
    let update_parent_label_model = Label::find()
        .filter(label::Column::Id.eq(update_item_model.parent_label_id))
        .one(&db)
        .await?
        .ok_or(AppError(anyhow::anyhow!("Label not found")))?;
    let update_grand_parent_label_model = Label::find()
        .filter(label::Column::Id.eq(update_item_model.grand_parent_label_id))
        .one(&db)
        .await?
        .ok_or(AppError(anyhow::anyhow!("Label not found")))?;
    let meilisearch_update_item: server::MeiliSearchItemData = server::MeiliSearchItemData {
        id,
        visible_id: update_label_model.visible_id,
        parent_visible_id: update_parent_label_model.visible_id,
        grand_parent_visible_id: update_grand_parent_label_model.visible_id,
        name: update_item_model.name,
        product_number: update_item_model.product_number,
        photo_url: update_item_model.photo_url,
        record: update_item_model.record,
        color: update_label_model.color,
        description: update_item_model.description,
        year_purchased: update_item_model.year_purchased,
        connector: update_item_model.connector,
        created_at: update_item_model.created_at,
        updated_at: update_item_model.updated_at,
    };
    let item_meilisearch = meilisearch_client
        .index("item")
        .add_documents(&vec![meilisearch_update_item.clone()], Some("id"))
        .await
        .unwrap();
    println!("[INFO]: MeiliSearch Result {:#?}", item_meilisearch);
    //return
    Ok(Json(meilisearch_update_item))
}
// pub async fn update_item_put(
//     Path(id): Path<i32>,
//     Extension(db): Extension<DatabaseConnection>,
//     mut multipart: Multipart,
// ) -> Result<Json<server::MeiliSearchItemData>, AppError> {
//     //parent_visible_idに変更があるかを確認するためのflag
//     let mut is_chaged_parennt_visible_id_flag = false;
//     //visible_idが変更されているかどうかの確認するためのflag
//     let mut is_changed_visible_id_flag = false;
//     //存在しないfield_nameがないか確認するためのflag
//     let mut have_invalid_field_name_flag = false;
//     //connectorのvector
//     let mut result_connector_vec: Vec<String> = Vec::new();
//     //DBに突っ込むデータ
//     let mut update_data = server::ControlItemData {
//         visible_id: "".to_string(),
//         parent_id: 0,
//         parent_visible_id: "".to_string(),
//         grand_parent_id: 0,
//         grand_parent_visible_id: "".to_string(),
//         name: "".to_string(),
//         product_number: "".to_string(),
//         record: Record::Qr,
//         color: Color::Red,
//         description: "".to_string(),
//         year_purchased: None,
//         connector: json!(result_connector_vec),
//     };
//     while let Some(field) = multipart.next_field().await? {
//         let field_name = field.name().unwrap().to_string();
//         println!("field name: {}", field_name);
//         //connector
//         if field_name.starts_with("connector") {
//             let connector = field.text().await?;
//             println!("connector: {}", connector);
//             result_connector_vec.push(connector);
//             continue;
//         }
//         match field_name.as_str() {
//             "visible_id" => {
//                 let visible_id = field.text().await?;
//                 println!("visible_id: {}", visible_id);
//                 //とりあえず格納する
//                 update_data.visible_id = visible_id;
//             }
//             "parent_visible_id" => {
//                 let parent_id = field.text().await?;
//                 println!("parent_visible_id: {}", parent_id);
//                 //とりあえず格納する
//                 update_data.parent_visible_id = parent_id;
//             }
//             "name" => {
//                 let name = field.text().await?;
//                 println!("name: {}", name);
//                 update_data.name = name;
//             }
//             "product_number" => {
//                 let product_number = field.text().await?;
//                 println!("product_number: {}", product_number);
//                 update_data.product_number = product_number;
//             }
//             "record" => {
//                 let record = field.text().await?;
//                 println!("record: {}", record);
//                 //Recordに不正な値が入っている場合の早期リターン
//                 if record != "Qr" && record != "Barcode" && record != "Nothing" {
//                     return Err(AppError(anyhow::anyhow!(
//                         "Record type '{}' is invalid",
//                         record
//                     )));
//                 }
//                 update_data.record = match record.as_str() {
//                     "Qr" => Record::Qr,
//                     "Barcode" => Record::Barcode,
//                     "Nothing" => Record::Nothing,
//                     _ => panic!("Record type validation was failed"),
//                 };
//             }
//             "color" => {
//                 let color = field.text().await?;
//                 println!("color: {}", color);
//                 //Colorに不正な値が入っている場合の早期リターン
//                 if color != "Red"
//                     && color != "Orange"
//                     && color != "Brown"
//                     && color != "SkyBlue"
//                     && color != "Blue"
//                     && color != "Green"
//                     && color != "Yellow"
//                     && color != "Purple"
//                     && color != "Pink"
//                 {
//                     return Err(AppError(anyhow::anyhow!(
//                         "Color type '{}' is invalid",
//                         color
//                     )));
//                 }
//                 update_data.color = match color.as_str() {
//                     "Red" => Color::Red,
//                     "Orange" => Color::Orange,
//                     "Brown" => Color::Brown,
//                     "SkyBlue" => Color::SkyBlue,
//                     "Blue" => Color::Blue,
//                     "Green" => Color::Green,
//                     "Yellow" => Color::Yellow,
//                     "Purple" => Color::Purple,
//                     "Pink" => Color::Pink,
//                     _ => panic!("Color type validation was failed"),
//                 };
//             }
//             "description" => {
//                 let description = field.text().await?;
//                 println!("description: {}", description);
//                 update_data.description = description;
//             }
//             "year_purchased" => {
//                 let year_purchased = field.text().await?;
//                 println!("year_purchased: {}", year_purchased);
//                 if year_purchased.is_empty() {
//                     update_data.year_purchased = None;
//                 } else {
//                     update_data.year_purchased = Some(year_purchased.parse::<i32>()?);
//                 }
//             }
//             _ => {
//                 println!("other");
//                 have_invalid_field_name_flag = true;
//             }
//         }
//     }
//     //存在しないfieldを取得した場合の早期リターン
//     if have_invalid_field_name_flag {
//         return Err(AppError(anyhow::anyhow!("Invalid field name")));
//     }
//     //connectorをupdate_dataに格納
//     update_data.connector = json!(result_connector_vec);
//     //木構造に整合性の乱れがないかチェック
//     //0. visible_idが変更されているかどうかの確認
//     //更新対象の物品が存在するか確認するflag
//     let mut is_exist_update_item_flag = false;
//     let current_update_item_state = Item::find_by_id(id).one(&db).await?;
//     match &current_update_item_state {
//         Some(current_update_item_state) => {
//             //1. visible_idが重複していないかチェック
//             if current_update_item_state.visible_id != update_data.visible_id {
//                 //物品ID: 変更
//                 is_changed_visible_id_flag = true;
//                 //visible_idが変更されている場合の被りがないかのチェック
//                 let all_items = Item::find().all(&db).await?;
//                 for item in all_items {
//                     if update_data.visible_id == item.visible_id {
//                         return Err(AppError(anyhow::anyhow!(
//                             "The visible id ({}) is already used",
//                             item.visible_id
//                         )));
//                     }
//                 }
//             }
//         }
//         None => is_exist_update_item_flag = true,
//     }
//     //更新対象の物品が存在しない場合の早期リターン
//     if is_exist_update_item_flag {
//         return Err(AppError(anyhow::anyhow!("Item not found")));
//     }
//     //3. 物品IDが物品IDを子孫に持つ全ての物品IDの子要素になっていないかのチェックをする
//     //親物品IDのチェックをする
//     //3.1. 親物品IDが変更されているかどうかの確認
//     match &current_update_item_state {
//         Some(current_update_item_state) => {
//             //ここで 3.1. のチェック
//             if current_update_item_state.parent_visible_id != update_data.parent_visible_id {
//                 //親物品ID: 変更
//                 is_chaged_parennt_visible_id_flag = true;
//                 //3.2. 物品IDが物品IDを子孫に持つ全ての物品IDの子要素になっていないかのチェックをする
//                 let mut descendants_items_vec: Vec<String> = Vec::new();
//                 let mut count = 0;
//                 let mut descendants_items: Vec<Model> = Vec::new();
//                 let mut new_children_items: Vec<Model> = Vec::new();
//                 let mut new_descendants_items: Vec<Model> = Vec::new();
//                 loop {
//                     if count == 0 {
//                         let children_items = Item::find()
//                             .filter(
//                                 item::Column::ParentVisibleId
//                                     .eq(&current_update_item_state.visible_id),
//                             )
//                             .all(&db)
//                             .await?;
//                         descendants_items = Item::find()
//                             .filter(
//                                 item::Column::GrandParentVisibleId
//                                     .eq(&current_update_item_state.visible_id),
//                             )
//                             .all(&db)
//                             .await?;
//                         for children_item in children_items {
//                             descendants_items_vec.push(children_item.visible_id);
//                         }
//                         for descendant_item in descendants_items.clone() {
//                             descendants_items_vec.push(descendant_item.visible_id);
//                         }
//                         count += 1;
//                     } else if count == 1 {
//                         for item in descendants_items.clone() {
//                             new_children_items = Item::find()
//                                 .filter(item::Column::ParentVisibleId.eq(&item.visible_id))
//                                 .all(&db)
//                                 .await?;
//                             new_descendants_items = Item::find()
//                                 .filter(item::Column::GrandParentVisibleId.eq(&item.visible_id))
//                                 .all(&db)
//                                 .await?;
//                             for new_children_item in new_children_items.clone() {
//                                 descendants_items_vec.push(new_children_item.visible_id);
//                             }
//                             for new_descendant_item in new_descendants_items.clone() {
//                                 descendants_items_vec.push(new_descendant_item.visible_id);
//                             }
//                         }
//                         descendants_items = new_descendants_items.clone();
//                         if new_children_items.is_empty() || new_descendants_items.is_empty() {
//                             break;
//                         }
//                     } else {
//                         break;
//                     }
//                 }
//                 for descendants_item in descendants_items_vec {
//                     if descendants_item == update_data.parent_visible_id {
//                         return Err(AppError(anyhow::anyhow!(
//                             "The visible id ({}) is a descendant of the one of the descendant visible id ({})",
//                             update_data.visible_id,
//                             current_update_item_state.visible_id
//                         )));
//                     }
//                 }
//             }
//         }
//         None => is_exist_update_item_flag = true,
//     }
//     //更新対象の物品が存在しない場合の早期リターン
//     if is_exist_update_item_flag {
//         return Err(AppError(anyhow::anyhow!("Item not found")));
//     }
//     //parent_id, grand_parent_idの取得
//     match Item::find()
//         .filter(item::Column::VisibleId.eq(&update_data.parent_visible_id))
//         .one(&db)
//         .await?
//     {
//         Some(item) => {
//             update_data.parent_id = item.id;
//             update_data.grand_parent_id = item.parent_id;
//             match Item::find()
//                 .filter(item::Column::Id.eq(item.parent_id))
//                 .one(&db)
//                 .await?
//             {
//                 Some(item) => {
//                     update_data.grand_parent_visible_id = item.visible_id;
//                 }
//                 None => is_exist_update_item_flag = true,
//             }
//         }
//         None => is_exist_update_item_flag = true,
//     };
//     //DBにデータの更新を反映する
//     println!("Validation Passed!");
//     //対象物品の更新
//     match current_update_item_state.clone() {
//         Some(item) => {
//             let mut item: ActiveModel = item.into();
//             item.visible_id = Set(update_data.visible_id.clone());
//             item.parent_id = Set(update_data.parent_id);
//             item.parent_visible_id = Set(update_data.parent_visible_id.clone());
//             item.grand_parent_id = Set(update_data.grand_parent_id);
//             item.grand_parent_visible_id = Set(update_data.grand_parent_visible_id.clone());
//             item.name = Set(update_data.name.clone());
//             item.product_number = Set(update_data.product_number.clone());
//             item.record = Set(update_data.record.clone());
//             item.color = Set(update_data.color.clone());
//             item.description = Set(update_data.description.clone());
//             item.year_purchased = Set(update_data.year_purchased);
//             item.connector = Set(update_data.connector.clone());
//             item.updated_at = Set(Utc::now().naive_local());
//             item.update(&db).await?;
//         }
//         None => is_exist_update_item_flag = true,
//     }
//     //更新対象の物品が存在しない場合の早期リターン
//     if is_exist_update_item_flag {
//         return Err(AppError(anyhow::anyhow!("Item not found")));
//     }
//     //子孫物品の更新
//     //物品ID: 変更
//     if let Some(current_update_item_state) = current_update_item_state {
//         if is_changed_visible_id_flag {
//             let children_items = Item::find()
//                 .filter(item::Column::ParentVisibleId.eq(&current_update_item_state.visible_id))
//                 .all(&db)
//                 .await?;
//             let descendants_items = Item::find()
//                 .filter(
//                     item::Column::GrandParentVisibleId.eq(&current_update_item_state.visible_id),
//                 )
//                 .all(&db)
//                 .await?;
//             for child_item in children_items {
//                 let mut child_item: ActiveModel = child_item.into();
//                 child_item.parent_visible_id = Set(update_data.visible_id.clone());
//                 child_item.update(&db).await?;
//             }
//             for descendant_item in descendants_items {
//                 let mut descendant_item: ActiveModel = descendant_item.into();
//                 descendant_item.grand_parent_visible_id = Set(update_data.visible_id.clone());
//                 descendant_item.update(&db).await?;
//             }
//         }
//         //親物品ID: 変更
//         if is_chaged_parennt_visible_id_flag {
//             let children_items = Item::find()
//                 .filter(item::Column::ParentVisibleId.eq(&current_update_item_state.visible_id))
//                 .all(&db)
//                 .await?;
//             for child_item in children_items {
//                 let mut child_item: ActiveModel = child_item.into();
//                 child_item.grand_parent_visible_id = Set(update_data.parent_visible_id.clone());
//                 child_item.update(&db).await?;
//             }
//         }
//     }
//     let updated_item = Item::find_by_id(id).one(&db).await?;
//     match updated_item {
//         Some(item) => {
//             let item = server::MeiliSearchItemData {
//                 id: item.id,
//                 visible_id: item.visible_id,
//                 parent_id: item.parent_id,
//                 parent_visible_id: item.parent_visible_id,
//                 grand_parent_id: item.grand_parent_id,
//                 grand_parent_visible_id: item.grand_parent_visible_id,
//                 name: item.name,
//                 product_number: item.product_number,
//                 photo_url: item.photo_url,
//                 record: item.record,
//                 color: item.color,
//                 description: item.description,
//                 year_purchased: item.year_purchased,
//                 connector: item.connector,
//                 created_at: item.created_at,
//                 updated_at: item.updated_at,
//             };
//             //meiliSearchにデータを更新する
//             let update_item_vec: Vec<MeiliSearchItemData> = vec![item.clone()];
//             let client = server::connect_meilisearch().await;
//             let item_meilisearch = client
//                 .index("item")
//                 .add_documents(&update_item_vec, Some("id"))
//                 .await?;
//             println!("MeiliSearch Result");
//             println!("{:?}", item_meilisearch);
//             Ok(Json(item))
//         }
//         None => Err(AppError(anyhow::anyhow!("Item not found"))),
//     }
// }

// pub async fn delete_item_delete(
//     Path(id): Path<i32>,
//     Extension(db): Extension<DatabaseConnection>,
// ) -> Result<Json<server::MeiliSearchItemData>, AppError> {
//     let delete_item = Item::find_by_id(id).one(&db).await?;
//     //削除対象のノードがあるか確認
//     match delete_item {
//         Some(delete_item) => {
//             //最上位のノードの場合
//             if delete_item.parent_id == id {
//                 return Err(AppError(anyhow::anyhow!("Can't delete top item")));
//             }
//             let children_items = Item::find()
//                 .filter(item::Column::ParentId.eq(id))
//                 .all(&db)
//                 .await?;
//             //最下層のノードの場合
//             if children_items.is_empty() {
//                 Item::delete_by_id(id).exec(&db).await?;
//                 let delete_item: server::MeiliSearchItemData = delete_item.into();
//                 return Ok(Json(delete_item));
//             }
//             if let Some(parent_item) = Item::find_by_id(delete_item.parent_id).one(&db).await? {
//                 for child_item in children_items {
//                     let mut child_item: item::ActiveModel = child_item.into();
//                     child_item.parent_id = Set(parent_item.id);
//                     child_item.parent_visible_id = Set(parent_item.visible_id.to_owned());
//                     child_item.grand_parent_id = Set(parent_item.parent_id);
//                     child_item.grand_parent_visible_id =
//                         Set(parent_item.parent_visible_id.to_owned());
//                     child_item.update(&db).await?;
//                 }
//             }
//             let delete_item: server::MeiliSearchItemData = delete_item.into();
//             Ok(Json(delete_item))
//         }
//         None => Err(AppError(anyhow::anyhow!("Item not found"))),
//     }
// }

pub async fn register_item_post(
    Extension(db): Extension<DatabaseConnection>,
    mut multipart: Multipart,
) -> Result<(), AppError> {
    //parent_visible_idに変更があるかを確認するためのflag
    let mut is_chaged_parennt_visible_id_flag = false;
    //存在しないfield_nameがないか確認するためのflag
    let mut have_invalid_field_name_flag = false;
    //connectorのvector
    let mut result_connector_vec: Vec<String> = Vec::new();
    let mut update_data = server::ControlItemData {
        visible_id: "".to_string(),
        parent_visible_id: "".to_string(),
        name: "".to_string(),
        product_number: "".to_string(),
        record: Record::Qr,
        description: "".to_string(),
        year_purchased: None,
        connector: json!(result_connector_vec),
    };
    while let Some(field) = multipart.next_field().await? {
        let field_name = field.name().unwrap().to_string();
        println!("field name: {}", field_name);
        //connector
        if field_name.starts_with("connector") {
            let connector = field.text().await?;
            println!("connector: {}", connector);
            result_connector_vec.push(connector);
            continue;
        }
        match field_name.as_str() {
            "visible_id" => {
                let visible_id = field.text().await?;
                println!("visible_id: {}", visible_id);
                //とりあえず格納する
                update_data.visible_id = visible_id;
            }
            "parent_visible_id" => {
                let parent_id = field.text().await?;
                println!("parent_visible_id: {}", parent_id);
                //とりあえず格納する
                update_data.parent_visible_id = parent_id;
            }
            "name" => {
                let name = field.text().await?;
                println!("name: {}", name);
                update_data.name = name;
            }
            "product_number" => {
                let product_number = field.text().await?;
                println!("product_number: {}", product_number);
                update_data.product_number = product_number;
            }
            "record" => {
                let record = field.text().await?;
                println!("record: {}", record);
                //Recordに不正な値が入っている場合の早期リターン
                if record != "Qr" && record != "Barcode" && record != "Nothing" {
                    return Err(AppError(anyhow::anyhow!(
                        "Record type '{}' is invalid",
                        record
                    )));
                }
                update_data.record = match record.as_str() {
                    "Qr" => Record::Qr,
                    "Barcode" => Record::Barcode,
                    "Nothing" => Record::Nothing,
                    _ => panic!("Record type validation was failed"),
                };
            }
            "description" => {
                let description = field.text().await?;
                println!("description: {}", description);
                update_data.description = description;
            }
            "year_purchased" => {
                let year_purchased = field.text().await?;
                println!("year_purchased: {}", year_purchased);
                if year_purchased.is_empty() {
                    update_data.year_purchased = None;
                } else {
                    update_data.year_purchased = Some(year_purchased.parse::<i32>()?);
                }
            }
            _ => {
                println!("other");
                have_invalid_field_name_flag = true;
            }
        }
    }
    //存在しないfieldを取得した場合の早期リターン
    if have_invalid_field_name_flag {
        return Err(AppError(anyhow::anyhow!("Invalid field name")));
    }
    //parent_visible_idの存在と物品IDとして利用されているかのバリデーション
    let parent_label_model = Label::find()
        .filter(label::Column::VisibleId.eq(&update_data.parent_visible_id))
        .one(&db)
        .await?
        .ok_or(AppError(anyhow::anyhow!(
            "Parent label model was not found"
        )))?;
    let parent_item = Item::find()
        .filter(item::Column::LabelId.eq(parent_label_model.id))
        .one(&db)
        .await?
        .ok_or(AppError(anyhow::anyhow!("Parent item was not found")))?;

    Ok(())
}
// pub async fn register_item_post(
//     Extension(db): Extension<DatabaseConnection>,
//     Extension(r2_url): Extension<String>,
//     mut multipart: Multipart,
// ) -> Result<Json<MeiliSearchItemData>, AppError> {
//     println!("Register Item");
//     //存在しないfield_nameがないか確認するためのflag
//     let mut have_invalid_field_name_flag = false;
//     //connectorのvector
//     let mut result_connector_vec: Vec<String> = Vec::new();
//     //DBに突っ込むデータ
//     let mut register_data = server::ControlItemData {
//         visible_id: "".to_string(),
//         parent_id: 0,
//         parent_visible_id: "".to_string(),
//         grand_parent_id: 0,
//         grand_parent_visible_id: "".to_string(),
//         name: "".to_string(),
//         product_number: "".to_string(),
//         record: Record::Qr,
//         color: Color::Red,
//         description: "".to_string(),
//         year_purchased: None,
//         connector: json!(result_connector_vec),
//     };
//     while let Some(field) = multipart.next_field().await? {
//         let field_name = field.name().unwrap().to_string();
//         println!("field name: {}", field_name);
//         //connector
//         if field_name.starts_with("connector") {
//             let connector = field.text().await?;
//             println!("connector: {}", connector);
//             result_connector_vec.push(connector);
//             continue;
//         }
//         match field_name.as_str() {
//             "visible_id" => {
//                 let visible_id = field.text().await?;
//                 println!("visible_id: {}", visible_id);
//                 //とりあえず格納する
//                 register_data.visible_id = visible_id;
//             }
//             "parent_visible_id" => {
//                 let parent_id = field.text().await?;
//                 println!("parent_visible_id: {}", parent_id);
//                 //とりあえず格納する
//                 register_data.parent_visible_id = parent_id;
//             }
//             "name" => {
//                 let name = field.text().await?;
//                 println!("name: {}", name);
//                 register_data.name = name;
//             }
//             "product_number" => {
//                 let product_number = field.text().await?;
//                 println!("product_number: {}", product_number);
//                 register_data.product_number = product_number;
//             }
//             "record" => {
//                 let record = field.text().await?;
//                 println!("record: {}", record);
//                 //Recordに不正な値が入っている場合の早期リターン
//                 if record != "Qr" && record != "Barcode" && record != "Nothing" {
//                     return Err(AppError(anyhow::anyhow!(
//                         "Record type '{}' is invalid",
//                         record
//                     )));
//                 }
//                 register_data.record = match record.as_str() {
//                     "Qr" => Record::Qr,
//                     "Barcode" => Record::Barcode,
//                     "Nothing" => Record::Nothing,
//                     _ => panic!("Record type validation was failed"),
//                 };
//             }
//             "color" => {
//                 let color = field.text().await?;
//                 println!("color: {}", color);
//                 //Colorに不正な値が入っている場合の早期リターン
//                 if color != "Red"
//                     && color != "Orange"
//                     && color != "Brown"
//                     && color != "SkyBlue"
//                     && color != "Blue"
//                     && color != "Green"
//                     && color != "Yellow"
//                     && color != "Purple"
//                     && color != "Pink"
//                 {
//                     return Err(AppError(anyhow::anyhow!(
//                         "Color type '{}' is invalid",
//                         color
//                     )));
//                 }
//                 register_data.color = match color.as_str() {
//                     "Red" => Color::Red,
//                     "Orange" => Color::Orange,
//                     "Brown" => Color::Brown,
//                     "SkyBlue" => Color::SkyBlue,
//                     "Blue" => Color::Blue,
//                     "Green" => Color::Green,
//                     "Yellow" => Color::Yellow,
//                     "Purple" => Color::Purple,
//                     "Pink" => Color::Pink,
//                     _ => panic!("Color type validation was failed"),
//                 };
//             }
//             "description" => {
//                 let description = field.text().await?;
//                 println!("description: {}", description);
//                 register_data.description = description;
//             }
//             "year_purchased" => {
//                 let year_purchased = field.text().await?;
//                 println!("year_purchased: {}", year_purchased);
//                 if year_purchased.is_empty() {
//                     register_data.year_purchased = None;
//                 } else {
//                     register_data.year_purchased = Some(year_purchased.parse::<i32>()?);
//                 }
//             }
//             _ => {
//                 println!("other");
//                 have_invalid_field_name_flag = true;
//             }
//         }
//     }

//     //存在しないfieldを取得した場合の早期リターン
//     if have_invalid_field_name_flag {
//         return Err(AppError(anyhow::anyhow!("Invalid field name")));
//     }
//     //connectorをregister_dataに格納
//     register_data.connector = json!(result_connector_vec);
//     let all_items = Item::find().all(&db).await?;
//     //物品IDが重複していないかのチェック
//     let mut is_exist_parent_item_flag = 0;
//     for item in &all_items {
//         if register_data.visible_id == item.visible_id {
//             return Err(AppError(anyhow::anyhow!(
//                 "The visible id ({}) is already used",
//                 item.visible_id
//             )));
//         }
//         if register_data.parent_visible_id == item.visible_id {
//             is_exist_parent_item_flag += 1;
//         }
//     }
//     //親物品IDが存在するかどうかのチェック
//     if is_exist_parent_item_flag != 1 {
//         return Err(AppError(anyhow::anyhow!(
//             "The parent visible id ({}) is not found",
//             &register_data.parent_visible_id
//         )));
//     }
//     //parent_id, grand_parent_idの取得
//     //更新対象の物品が存在するか確認するflag
//     let mut is_exist_parent_item_flag = false;
//     match Item::find()
//         .filter(item::Column::VisibleId.eq(&register_data.parent_visible_id))
//         .one(&db)
//         .await?
//     {
//         Some(item) => {
//             register_data.parent_id = item.id;
//             register_data.grand_parent_id = item.parent_id;
//             match Item::find()
//                 .filter(item::Column::Id.eq(item.parent_id))
//                 .one(&db)
//                 .await?
//             {
//                 Some(item) => {
//                     register_data.grand_parent_visible_id = item.visible_id;
//                 }
//                 None => is_exist_parent_item_flag = true,
//             }
//         }
//         None => is_exist_parent_item_flag = true,
//     };
//     //更新対象の物品が存在しない場合の早期リターン
//     if is_exist_parent_item_flag {
//         return Err(AppError(anyhow::anyhow!("Parent item not found")));
//     }
//     //DBにデータを登録する
//     let item_model = item::ActiveModel {
//         visible_id: Set(register_data.visible_id.clone()),
//         parent_id: Set(register_data.parent_id),
//         parent_visible_id: Set(register_data.parent_visible_id.clone()),
//         grand_parent_id: Set(register_data.grand_parent_id),
//         grand_parent_visible_id: Set(register_data.grand_parent_visible_id.clone()),
//         name: Set(register_data.name.clone()),
//         product_number: Set(register_data.product_number.clone()),
//         photo_url: Set("".to_string()),
//         record: Set(register_data.record.clone()),
//         color: Set(register_data.color.clone()),
//         description: Set(register_data.description.clone()),
//         year_purchased: Set(register_data.year_purchased),
//         connector: Set(register_data.connector.clone()),
//         created_at: Set(Utc::now().naive_local()),
//         updated_at: Set(Utc::now().naive_local()),
//         ..Default::default()
//     };
//     Item::insert(item_model).exec(&db).await?;
//     //DBに登録したデータを取得する
//     let register_item = Item::find()
//         .filter(item::Column::VisibleId.eq(&register_data.visible_id))
//         .one(&db)
//         .await?;
//     if let Some(item_model) = register_item {
//         let mut item: ActiveModel = item_model.clone().into();
//         item.photo_url = Set(format!("{}/{}.webp", r2_url, item_model.id));
//         item.update(&db).await?;
//     } else {
//         return Err(AppError(anyhow::anyhow!("Item not found")));
//     }
//     let register_item = Item::find()
//         .filter(item::Column::VisibleId.eq(&register_data.visible_id))
//         .one(&db)
//         .await?;
//     if let Some(item_model) = register_item {
//         let item = server::MeiliSearchItemData {
//             id: item_model.id,
//             visible_id: item_model.visible_id,
//             parent_id: item_model.parent_id,
//             parent_visible_id: item_model.parent_visible_id,
//             grand_parent_id: item_model.grand_parent_id,
//             grand_parent_visible_id: item_model.grand_parent_visible_id,
//             name: item_model.name,
//             product_number: item_model.product_number,
//             photo_url: item_model.photo_url,
//             record: item_model.record,
//             color: item_model.color,
//             description: item_model.description,
//             year_purchased: item_model.year_purchased,
//             connector: item_model.connector,
//             created_at: item_model.created_at,
//             updated_at: item_model.updated_at,
//         };
//         let register_item_vec: Vec<MeiliSearchItemData> = vec![item.clone()];
//         let client = server::connect_meilisearch().await;
//         let item_meilisearch = client
//             .index("item")
//             .add_documents(&register_item_vec, Some("id"))
//             .await?;
//         println!("MeiliSearch Result");
//         println!("{:?}", item_meilisearch);
//         Ok(Json(item))
//     } else {
//         Err(AppError(anyhow::anyhow!("Item not found")))
//     }
// }
