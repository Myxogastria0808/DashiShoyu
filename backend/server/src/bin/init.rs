use ::entity::{
    grand_parent_label_junction::{self, Entity as GrandParentLabelJunction},
    item::{self, Entity as Item, Record},
    label::{self, Color, Entity as Label},
    parent_label_junction::{self, Entity as ParentLabelJunction},
};
use chrono::Utc;
use csv::Error;
use sea_orm::{
    self, ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{collections::HashMap, env, process};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct CsvItemData {
    visible_id: String,
    parent_visible_id: String,
    name: String,
    #[serde(deserialize_with = "csv::invalid_option")]
    product_number: Option<String>,
    record: String,
    color: String,
    #[serde(deserialize_with = "csv::invalid_option")]
    description: Option<String>,
    #[serde(deserialize_with = "csv::invalid_option")]
    year_purchased: Option<i32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    connector_1: Option<String>,
    #[serde(deserialize_with = "csv::invalid_option")]
    connector_2: Option<String>,
    #[serde(deserialize_with = "csv::invalid_option")]
    connector_3: Option<String>,
    #[serde(deserialize_with = "csv::invalid_option")]
    connector_4: Option<String>,
    #[serde(deserialize_with = "csv::invalid_option")]
    connector_5: Option<String>,
    #[serde(deserialize_with = "csv::invalid_option")]
    connector_6: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ItemData {
    visible_id: String,
    parent_visible_id: String,
    grand_parent_visible_id: String,
    name: String,
    product_number: String,
    photo_url: String,
    //Record
    record: Record,
    //Color
    color: Color,
    description: String,
    year_purchased: Option<i32>,
    //serde_json::Value
    connector: Vec<String>,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    match make_item_data().await {
        Ok(data) => match insert_item_data_to_db(data).await {
            Ok(_) => {
                println!("\nSuccess!");
            }
            Err(e) => {
                eprintln!("[Error]: {}", e);
                process::exit(1);
            }
        },
        Err(e) => {
            eprintln!("[Error]: {}", e);
            process::exit(1);
        }
    }
}

async fn read_item_raw_data_csv() -> Result<Vec<CsvItemData>, Box<Error>> {
    let file_path = env::args_os()
        .nth(1)
        .expect("Give the relative path of the csv file as an argument.");
    let mut rdr = csv::Reader::from_path(file_path)?;
    let mut result_vec: Vec<CsvItemData> = Vec::new();
    for record in rdr.deserialize() {
        result_vec.push(record?);
    }
    Ok(result_vec)
}

async fn convert_to_item_data(data: Vec<CsvItemData>) -> Result<Vec<ItemData>, Box<Error>> {
    //insert item data
    let hash_map: HashMap<String, String> = data
        .iter()
        .map(|item| (item.visible_id.clone(), item.parent_visible_id.clone()))
        .collect();
    let data = data
        .into_iter()
        .map(|item| ItemData {
            grand_parent_visible_id: match hash_map.get(&item.parent_visible_id) {
                Some(s) => s.to_owned(),
                None => {
                    panic!("Parent is not found: {}", item.visible_id)
                }
            },
            visible_id: item.visible_id.clone(),
            parent_visible_id: item.parent_visible_id,
            name: item.name,
            product_number: item.product_number.unwrap_or_default(),
            photo_url: "".to_string(),
            record: match item.record.as_str() {
                "qr" => Record::Qr,
                "barcode" => Record::Barcode,
                "nothing" => Record::Nothing,
                _ => {
                    panic!("Invalid record: {}", item.record)
                }
            },
            color: match item.color.as_str() {
                "red" => Color::Red,
                "orange" => Color::Orange,
                "brown" => Color::Brown,
                "skyblue" => Color::SkyBlue,
                "blue" => Color::Blue,
                "green" => Color::Green,
                "yellow" => Color::Yellow,
                "purple" => Color::Purple,
                "pink" => Color::Pink,
                _ => {
                    panic!("Invalid color: {}", item.color)
                }
            },
            description: match item.description {
                Some(content) => content,
                None => "".to_string(),
            },
            year_purchased: item.year_purchased,
            connector: vec![
                item.connector_1.unwrap_or_default(),
                item.connector_2.unwrap_or_default(),
                item.connector_3.unwrap_or_default(),
                item.connector_4.unwrap_or_default(),
                item.connector_5.unwrap_or_default(),
                item.connector_6.unwrap_or_default(),
            ],
        })
        .collect::<Vec<ItemData>>();
    let mut connector_vec_list: Vec<Vec<String>> = Vec::new();
    let mut result_vec: Vec<ItemData> = Vec::new();
    for item in &data {
        let mut connector_vec: Vec<String> = Vec::new();
        for connector_item in item.connector.iter() {
            match connector_item.as_str() {
                "" => {}
                _ => {
                    connector_vec.push(connector_item.to_string().to_owned());
                }
            }
        }
        connector_vec_list.push(connector_vec);
    }
    for (index, item) in data.iter().enumerate() {
        let item_data = ItemData {
            grand_parent_visible_id: item.grand_parent_visible_id.clone(),
            visible_id: item.visible_id.clone(),
            parent_visible_id: item.parent_visible_id.clone(),
            name: item.name.clone(),
            product_number: item.product_number.clone(),
            photo_url: item.photo_url.clone(),
            record: item.record.clone(),
            color: item.color.clone(),
            description: item.description.clone(),
            year_purchased: item.year_purchased,
            connector: connector_vec_list[index].clone(),
        };
        result_vec.push(item_data);
    }
    Ok(result_vec)
}

async fn make_item_data() -> Result<Vec<ItemData>, Box<Error>> {
    let data = read_item_raw_data_csv().await?;
    let data = convert_to_item_data(data).await?;
    Ok(data)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct VisibleIds {
    id: i32,
    visible_id: String,
    label_id: i32,
    parent_visible_id: String,
    parent_label_id: i32,
    parent_label_junction_id: i32,
    grand_parent_visible_id: String,
    grand_parent_label_id: i32,
    grand_parent_label_junction_id: i32,
}

async fn insert_item_data_to_db(data: Vec<ItemData>) -> Result<(), DbErr> {
    //connect db
    let db: DatabaseConnection = server::connect_db().await?;
    let mut visible_ids_vec: Vec<VisibleIds> = Vec::new();
    for item in &data {
        // Insert label data
        let label_model = label::ActiveModel {
            visible_id: Set(item.visible_id.clone()),
            color: Set(item.color.clone()),
            ..Default::default()
        };
        let inserted_label_data = Label::insert(label_model).exec(&db).await?;
        let label_id = inserted_label_data.last_insert_id;
        visible_ids_vec.push(VisibleIds {
            id: 0,
            visible_id: item.visible_id.clone(),
            label_id,
            parent_visible_id: item.parent_visible_id.clone(),
            parent_label_id: 0,
            parent_label_junction_id: 0,
            grand_parent_visible_id: item.grand_parent_visible_id.clone(),
            grand_parent_label_id: 0,
            grand_parent_label_junction_id: 0,
        });
        println!("[INFO]: {:?}", inserted_label_data);
    }
    println!("[INFO]: insert to Label Table was completed!");
    for item in &data {
        let parent_visible_ids = visible_ids_vec
            .clone()
            .into_iter()
            .find(|visible_ids| item.parent_visible_id == visible_ids.visible_id)
            .expect("Parent visible id is not found.");
        let grand_parent_visible_ids = visible_ids_vec
            .clone()
            .into_iter()
            .find(|visible_ids| item.grand_parent_visible_id == visible_ids.visible_id)
            .expect("Parent visible id is not found.");
        let visible_ids_index = visible_ids_vec
            .clone()
            .into_iter()
            .position(|visible_ids| item.visible_id == visible_ids.visible_id)
            .expect("Visible id is not found.");
        visible_ids_vec[visible_ids_index].parent_label_id = parent_visible_ids.label_id;
        visible_ids_vec[visible_ids_index].grand_parent_label_id =
            grand_parent_visible_ids.label_id;
    }
    for (visible_ids_vec_index, visible_ids) in visible_ids_vec.clone().into_iter().enumerate() {
        let parent_label_junction_model = parent_label_junction::ActiveModel {
            label_id: Set(visible_ids.parent_label_id),
            ..Default::default()
        };
        let parent_label_junction_id = ParentLabelJunction::insert(parent_label_junction_model)
            .exec(&db)
            .await?
            .last_insert_id;
        let grand_parent_label_junction_model = grand_parent_label_junction::ActiveModel {
            label_id: Set(visible_ids.grand_parent_label_id),
            ..Default::default()
        };
        let grand_parent_label_junction_id =
            GrandParentLabelJunction::insert(grand_parent_label_junction_model)
                .exec(&db)
                .await?
                .last_insert_id;
        visible_ids_vec[visible_ids_vec_index].parent_label_junction_id = parent_label_junction_id;
        visible_ids_vec[visible_ids_vec_index].grand_parent_label_junction_id =
            grand_parent_label_junction_id;
    }
    for item in &data {
        let visible_ids = visible_ids_vec
            .clone()
            .into_iter()
            .find(|visible_ids| item.visible_id == visible_ids.visible_id)
            .expect("Visible id is not found.");
        let item_model = item::ActiveModel {
            label_id: Set(visible_ids.label_id),
            parent_id: Set(0), //初期値を0でセット
            parent_label_id: Set(visible_ids.parent_label_junction_id),
            grand_parent_id: Set(0), //初期値を0でセット
            grand_parent_label_id: Set(visible_ids.grand_parent_label_junction_id),
            name: Set(item.name.clone()),
            product_number: Set(item.product_number.clone()),
            photo_url: Set(item.photo_url.clone()),
            record: Set(item.record.clone()),
            description: Set(item.description.clone()),
            year_purchased: Set(item.year_purchased),
            connector: Set(json!(item.connector.clone())),
            created_at: Set(Utc::now().naive_local()),
            updated_at: Set(Utc::now().naive_local()),
            ..Default::default()
        };
        let inserted_item = Item::insert(item_model).exec(&db).await?;
        let inserted_item_id = inserted_item.last_insert_id;
        let visible_ids_index = visible_ids_vec
            .clone()
            .into_iter()
            .position(|visible_ids| item.visible_id == visible_ids.visible_id)
            .expect("Visible id is not found.");
        visible_ids_vec[visible_ids_index].id = inserted_item_id;
        //upload image
        let r2_manager = server::connect_r2().await;
        println!(
            "[INFO]: Now target {} ({}.webp)",
            visible_ids.visible_id, inserted_item_id
        );
        let input_file_path = format!("./src/bin/data/in/{}.jpg", visible_ids.visible_id);
        let output_file_path = format!("./src/bin/data/out/{}.webp", inserted_item_id);
        let _ = server::convert_to_webp(&input_file_path, &output_file_path, 75.0);
        let file_name = format!("{}.webp", inserted_item_id);
        let _ = server::upload_image_file(
            r2_manager.clone(),
            &input_file_path,
            &file_name,
            "image/webp",
        )
        .await;
        let _ = server::get_image(r2_manager, &file_name)
            .await
            .expect("upload image was failed.");
        println!(
            "[INFO]: Upload {} image file was completed!",
            visible_ids.visible_id
        );
        //update photo_url
        let mut update_item: item::ActiveModel = Item::find_by_id(inserted_item_id)
            .one(&db)
            .await?
            .expect("Register item was failed.")
            .into();
        update_item.photo_url = Set(format!(
            "{}/{}.webp",
            server::get_r2_url().await,
            inserted_item_id
        ));
        update_item.update(&db).await?;
    }
    for item in &data {
        let visible_ids = visible_ids_vec
            .clone()
            .into_iter()
            .find(|visible_ids| item.visible_id == visible_ids.visible_id)
            .expect("Visible id is not found.");
        let update_item = Item::find_by_id(visible_ids.id)
            .one(&db)
            .await?
            .expect("Register item was failed.");
        let mut update_item: item::ActiveModel = update_item.into();
        let parent_label = Label::find()
            .filter(label::Column::VisibleId.eq(visible_ids.parent_visible_id))
            .one(&db)
            .await?
            .expect("Parent label is not found.");
        let parent_item = Item::find()
            .filter(item::Column::LabelId.eq(parent_label.id))
            .one(&db)
            .await?
            .expect("Parent item is not found.");
        let grand_parent_label = Label::find()
            .filter(label::Column::VisibleId.eq(visible_ids.grand_parent_visible_id))
            .one(&db)
            .await?
            .expect("Parent label is not found.");
        let grand_parent_item = Item::find()
            .filter(item::Column::LabelId.eq(grand_parent_label.id))
            .one(&db)
            .await?
            .expect("Parent item is not found.");
        update_item.parent_id = Set(parent_item.id);
        update_item.grand_parent_id = Set(grand_parent_item.id);
        let result = update_item.update(&db).await?;
        println!("[INFO]: {:#?}", result);
    }
    println!("[INFO]: Insert data to DB was completed!");
    let mut meilisearch_item_vec: Vec<server::MeiliSearchItemData> = Vec::new();
    for item in data.clone() {
        let db_label_item = Label::find()
            .filter(label::Column::VisibleId.eq(&item.visible_id))
            .one(&db)
            .await?
            .expect("Item is not found.");
        let db_item = Item::find()
            .filter(item::Column::LabelId.eq(db_label_item.id))
            .one(&db)
            .await?
            .expect("Item is not found.");
        let meilisearch_item = server::MeiliSearchItemData {
            id: db_item.id,
            visible_id: item.visible_id.clone(),
            parent_visible_id: item.parent_visible_id.clone(),
            grand_parent_visible_id: item.grand_parent_visible_id.clone(),
            name: db_item.name,
            product_number: db_item.product_number,
            photo_url: db_item.photo_url,
            record: db_item.record,
            color: item.color,
            description: db_item.description,
            year_purchased: db_item.year_purchased,
            connector: db_item.connector,
            created_at: db_item.created_at,
            updated_at: db_item.updated_at,
        };
        meilisearch_item_vec.push(meilisearch_item);
    }
    let client = server::connect_meilisearch().await;
    let item_meilisearch = client
        .index("item")
        .add_documents(&meilisearch_item_vec, Some("id"))
        .await
        .unwrap();
    println!("[INFO]: MeiliSearch Result\n{:#?}", item_meilisearch);
    println!("\n[INFO]: Insert data to MeiliSearch was completed!");
    Ok(())
}
