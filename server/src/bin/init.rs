use ::entity::item::{self, Entity as Item};
use chrono::{NaiveDateTime, Utc};
use csv::Error;
use sea_orm::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{collections::HashMap, env, process};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ItemRawData {
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
    is_discarded: String,
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
    record: String,
    //Color
    color: String,
    description: String,
    year_purchased: Option<i32>,
    is_discarded: bool,
    //serde_json::Value
    connector: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ItemMeilisearchData {
    id: i32,
    visible_id: String,
    parent_id: i32,
    parent_visible_id: String,
    grand_parent_id: i32,
    grand_parent_visible_id: String,
    name: String,
    product_number: String,
    photo_url: String,
    record: String,
    color: String,
    description: String,
    year_purchased: Option<i32>,
    is_discarded: bool,
    connector: JsonValue,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    match make_item_data().await {
        Ok(data) => match insert_item_data(data).await {
            Ok(_) => {
                println!("\nSuccess!");
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        },
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}

async fn read_item_raw_data_csv() -> Result<Vec<ItemRawData>, Box<Error>> {
    let file_path = env::args_os()
        .nth(1)
        .expect("Give the relative path of the csv file as an argument.");
    let mut rdr = csv::Reader::from_path(file_path)?;
    let mut result_vec: Vec<ItemRawData> = Vec::new();
    for record in rdr.deserialize() {
        result_vec.push(record?);
    }
    Ok(result_vec)
}

async fn convert_to_item_data(data: Vec<ItemRawData>) -> Result<Vec<ItemData>, Box<Error>> {
    //upload image
    // get r2_manager
    let r2_manager = server::connect_r2().await;
    // get r2_url
    let r2_url = server::get_r2_url().await;
    for item in &data {
        println!("Now target: {}", item.visible_id);
        let input_file_path = format!("./src/bin/data/in/{}.jpg", item.visible_id);
        let output_file_path = format!("./src/bin/data/out/{}.webp", item.visible_id);
        let _ = server::convert_to_webp(&input_file_path, &output_file_path, 75.0);
        //upload image to R2
        let file_name = format!("{}.webp", item.visible_id);
        let _ = server::upload_image(
            r2_manager.clone(),
            &input_file_path,
            &file_name,
            "image/webp",
        )
        .await;
        println!("Upload was completed!: {}", item.visible_id);
    }
    println!("Upload files were completed!");
    //insert item data
    let hash_map: HashMap<String, String> = data
        .iter()
        .map(|item| (item.visible_id.clone(), item.parent_visible_id.clone()))
        .collect();
    Ok(data
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
            product_number: match item.product_number {
                Some(content) => content,
                None => "".to_string(),
            },
            photo_url: format!("{}/{}.webp", r2_url, item.visible_id),
            record: match item.record.as_str() {
                "qr" => "Qr".to_string(),
                "barcode" => "Barcode".to_string(),
                "nothing" => "Nothing".to_string(),
                _ => {
                    panic!("Invalid record: {}", item.record)
                }
            },
            color: match item.color.as_str() {
                "red" => "Red".to_string(),
                "orange" => "Orange".to_string(),
                "brown" => "Brown".to_string(),
                "skyblue" => "SkyBlue".to_string(),
                "blue" => "Blue".to_string(),
                "green" => "Green".to_string(),
                "yellow" => "Yellow".to_string(),
                "purple" => "Purple".to_string(),
                "pink" => "Pink".to_string(),
                _ => {
                    panic!("Invalid color: {}", item.color)
                }
            },
            description: match item.description {
                Some(content) => content,
                None => "".to_string(),
            },
            year_purchased: item.year_purchased,
            is_discarded: match item.is_discarded.as_str() {
                "TRUE" => true,
                "FALSE" => false,
                _ => {
                    panic!("Invalid is_discarded: {}", item.is_discarded)
                }
            },
            connector: vec![
                match item.connector_1 {
                    Some(content) => content,
                    None => "".to_string(),
                },
                match item.connector_2 {
                    Some(content) => content,
                    None => "".to_string(),
                },
                match item.connector_3 {
                    Some(content) => content,
                    None => "".to_string(),
                },
                match item.connector_4 {
                    Some(content) => content,
                    None => "".to_string(),
                },
                match item.connector_5 {
                    Some(content) => content,
                    None => "".to_string(),
                },
                match item.connector_6 {
                    Some(content) => content,
                    None => "".to_string(),
                },
            ],
        })
        .collect::<Vec<ItemData>>())
}

async fn make_item_data() -> Result<Vec<ItemData>, Box<Error>> {
    let data = read_item_raw_data_csv().await?;
    let data = convert_to_item_data(data).await?;
    Ok(data)
}

async fn insert_item_data_to_db(data: Vec<ItemData>) -> Result<(), DbErr> {
    //connect db
    let db: DatabaseConnection = server::connect_db().await?;
    let mut all_data: Vec<(String, i32)> = Vec::new();
    // Insert data
    for item in data.iter() {
        let item_model = item::ActiveModel {
            visible_id: Set(item.visible_id.clone()),
            parent_id: Set(0),
            parent_visible_id: Set(item.parent_visible_id.clone()),
            grand_parent_id: Set(0),
            grand_parent_visible_id: Set(item.grand_parent_visible_id.clone()),
            name: Set(item.name.clone()),
            product_number: Set(item.product_number.clone()),
            photo_url: Set(item.photo_url.clone()),
            record: Set(item.record.clone()),
            color: Set(item.color.clone()),
            description: Set(item.description.clone()),
            year_purchased: Set(item.year_purchased),
            is_discarded: Set(item.is_discarded),
            connector: Set(json!(item.connector.clone())),
            created_at: Set(Utc::now().naive_local()),
            updated_at: Set(Utc::now().naive_local()),
            ..Default::default()
        };
        let inserted_data: InsertResult<item::ActiveModel> =
            Item::insert(item_model).exec(&db).await?;
        let inserted_id = inserted_data.last_insert_id;
        all_data.push((item.visible_id.clone(), inserted_id));
    }
    let hash_map: HashMap<String, i32> = all_data.into_iter().collect();
    let all_data = Item::find().all(&db).await?;
    for item in all_data {
        let parent_visible_id = item.parent_visible_id.to_owned();
        let grand_parent_visible_id = item.grand_parent_visible_id.to_owned();
        let mut item: item::ActiveModel = item.into();
        item.parent_id = Set(match hash_map.get(&parent_visible_id) {
            Some(id) => *id,
            None => {
                panic!("Parent item is not found.");
            }
        });
        item.grand_parent_id = Set(match hash_map.get(&grand_parent_visible_id) {
            Some(id) => *id,
            None => {
                panic!("Parent item is not found.");
            }
        });
        item.update(&db).await?;
    }
    println!("Insert data to DB was completed!");
    Ok(())
}

async fn insert_item_data_to_meilisearch() -> Result<(), DbErr> {
    //connect db
    let db: DatabaseConnection = server::connect_db().await?;
    //get all db data
    let all_data = Item::find().all(&db).await?;
    let mut result_vec: Vec<ItemMeilisearchData> = Vec::new();
    for item in all_data {
        let item = ItemMeilisearchData {
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
            is_discarded: item.is_discarded,
            connector: item.connector,
            created_at: item.created_at,
            updated_at: item.updated_at,
        };
        result_vec.push(item);
    }
    //connect meilisearch
    let client = server::connect_meilisearch().await;
    let item_meilisearch = client
        .index("item")
        .add_documents(&result_vec, Some("id"))
        .await
        .unwrap();
    println!("\n[Meiliserch Result]");
    println!("{:#?}", item_meilisearch);
    println!("\nInsert data to MeiliSearch was completed!");
    Ok(())
}

async fn insert_item_data(data: Vec<ItemData>) -> Result<(), DbErr> {
    insert_item_data_to_db(data).await?;
    insert_item_data_to_meilisearch().await?;
    Ok(())
}
