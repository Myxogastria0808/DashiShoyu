use ::entity::{
    item::{self, Entity as Item, Record},
    label::{self, Color, Entity as Label},
    object::{self, Entity as Object},
    object_tag_junction::{self, Entity as ObjectTagJunction},
    tag::{self, Entity as Tag},
};
use chrono::Utc;
use csv::Error;
use neo4rs::query;
use sea_orm::{self, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{env, process};

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
struct InsertData {
    visible_id: String,
    parent_visible_id: String,
    name: String,
    product_number: String,
    photo_url: String,
    record: Record,
    color: Color,
    description: String,
    year_purchased: Option<i32>,
    connector: Vec<String>,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    match make_item_data().await {
        Ok(data) => match insert_item_data_to_db(data).await {
            Ok(_) => {
                println!("Success!");
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

async fn convert_to_item_data(data: Vec<CsvItemData>) -> Result<Vec<InsertData>, Box<Error>> {
    //insert item data
    let data = data
        .into_iter()
        .map(|item| InsertData {
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
        .collect::<Vec<InsertData>>();
    let mut connector_vec_list: Vec<Vec<String>> = Vec::new();
    let mut result_vec: Vec<InsertData> = Vec::new();
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
        let item_data = InsertData {
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

async fn make_item_data() -> Result<Vec<InsertData>, Box<Error>> {
    let data = read_item_raw_data_csv().await?;
    let data = convert_to_item_data(data).await?;
    Ok(data)
}

async fn insert_item_data_to_db(data: Vec<InsertData>) -> Result<(), DbErr> {
    //connect db
    let db: DatabaseConnection = server::connect_db().await?;
    //insert to Label table
    //gen root
    let root_label_model = label::ActiveModel {
        visible_id: Set("ROOT".to_string()),
        color: Set(Color::Purple),
    };
    let inserted_root_label_data = Label::insert(root_label_model).exec(&db).await?;
    println!(
        "[INFO]: Inserted to Label Table: {:?}",
        inserted_root_label_data
    );
    //decendants
    for item in &data {
        let label_model = label::ActiveModel {
            visible_id: Set(item.visible_id.clone()),
            color: Set(item.color.clone()),
        };
        let inserted_label_data = Label::insert(label_model).exec(&db).await?;
        println!("[INFO]: Inserted to Label Table: {:?}", inserted_label_data);
    }
    //r2
    let r2_manager = server::connect_r2().await;
    //r2 url
    let r2_url = server::get_r2_url().await;
    //meilisearch vector
    let mut meilisearch_item_vec: Vec<server::MeiliSearchItemData> = Vec::new();
    //insert to Item table
    //gen root
    let connector: Vec<String> = Vec::new();
    let root_item_model = item::ActiveModel {
        visible_id: Set("ROOT".to_string()),
        name: Set("筑波大学".to_string()),
        product_number: Set("ROOT".to_string()),
        photo_url: Set("".to_string()),
        record: Set(Record::Nothing),
        description: Set("https://www.tsukuba.ac.jp/".to_string()),
        year_purchased: Set(None),
        connector: Set(json!(connector)),
        created_at: Set(Utc::now().naive_local()),
        updated_at: Set(Utc::now().naive_local()),
        ..Default::default()
    };
    let inserted_root_item_data = Item::insert(root_item_model).exec(&db).await?;
    println!(
        "[INFO]: Inserted to Item Table: {:?}",
        inserted_root_item_data
    );
    //decendants
    for item in &data {
        let item_model = item::ActiveModel {
            visible_id: Set(item.visible_id.clone()),
            name: Set(item.name.clone()),
            product_number: Set(item.product_number.clone()),
            photo_url: Set(item.visible_id.clone()),
            record: Set(item.record.clone()),
            description: Set(item.description.clone()),
            year_purchased: Set(item.year_purchased),
            connector: Set(json!(item.connector)),
            created_at: Set(Utc::now().naive_local()),
            updated_at: Set(Utc::now().naive_local()),
            ..Default::default()
        };
        let inserted_item_data = Item::insert(item_model).exec(&db).await?;
        println!("[INFO]: Inserted to Item Table: {:?}", inserted_item_data);
        //upload image
        let input_file_path = format!("./src/bin/data/in/{}.jpg", item.visible_id);
        let output_file_path = format!(
            "./src/bin/data/out/{}.webp",
            inserted_item_data.last_insert_id
        );
        let _ = server::convert_to_webp(&input_file_path, &output_file_path, 75.0);
        let file_name = format!("{}.webp", inserted_item_data.last_insert_id);
        let _ = server::upload_image_file(
            r2_manager.clone(),
            &input_file_path,
            &file_name,
            "image/webp",
        )
        .await;
        let _ = server::get_image(r2_manager.clone(), &file_name)
            .await
            .expect("upload image was failed.");
        println!(
            "[INFO]: Uploading {}.webp was succeeded.",
            inserted_item_data.last_insert_id
        );
        let mut last_insreted_item: item::ActiveModel =
            Item::find_by_id(inserted_item_data.last_insert_id)
                .one(&db)
                .await?
                .unwrap_or_else(|| panic!("[ERROR]: Id is missing: {}", &item.visible_id))
                .into();
        last_insreted_item.photo_url = Set(format!(
            "{}/{}.webp",
            r2_url, inserted_item_data.last_insert_id
        ));
        let _ = Item::update(last_insreted_item).exec(&db).await;
        //insert to meilisearch
        let item_data = Item::find_by_id(inserted_item_data.last_insert_id)
            .one(&db)
            .await?
            .expect("[ERROR]: Item not found.");
        let label_data = Label::find()
            .filter(label::Column::VisibleId.eq(&item_data.visible_id))
            .one(&db)
            .await?
            .expect("[ERROR]: Label not found.");
        let meilisearch_item = server::MeiliSearchItemData {
            id: item_data.id,
            visible_id: item_data.visible_id,
            name: item_data.name,
            product_number: item_data.product_number,
            photo_url: item_data.photo_url,
            record: item_data.record,
            color: label_data.color,
            description: item_data.description,
            year_purchased: item_data.year_purchased,
            connector: serde_json::from_value::<Vec<String>>(item_data.connector).unwrap(),
            created_at: item_data.created_at.to_string(),
            updated_at: item_data.updated_at.to_string(),
        };
        meilisearch_item_vec.push(meilisearch_item);
    }
    //insert to Object table
    let object_model = object::ActiveModel {
        name: Set("つくば市の市内風景".to_string()),
        photo_url: Set(format!("{}/obj-1.jpg", r2_url)),
        mime_type: Set("image/jpeg".to_string()),
        license: Set("https://www.city.tsukuba.lg.jp/soshikikarasagasu/shichokoshitsukohosenryakuka/gyomuannai/6/1008071.html からダウンロードした画像を保存している。ライセンスは、URL先のものに従う。".to_string()),
        description: Set("つくば市のサイトにあった画像です。".to_string()),
        created_at: Set(Utc::now().naive_local()),
        updated_at: Set(Utc::now().naive_local()),
        ..Default::default()
    };
    let inserted_object_data = Object::insert(object_model).exec(&db).await?;
    println!(
        "[INFO]: Inserted to Object Table: {:?}",
        inserted_object_data
    );
    //insert to Tag table
    let tag_model_1 = tag::ActiveModel {
        name: Set("風景".to_string()),
        ..Default::default()
    };
    let inserted_tag_data_1 = Tag::insert(tag_model_1).exec(&db).await?;
    println!("[INFO]: Inserted to Tag Table: {:?}", inserted_tag_data_1);
    let tag_model_2 = tag::ActiveModel {
        name: Set("つくば".to_string()),
        ..Default::default()
    };
    let inserted_tag_data_2 = Tag::insert(tag_model_2).exec(&db).await?;
    println!("[INFO]: Inserted to Tag Table: {:?}", inserted_tag_data_2);
    let object_tag_model_1 = object_tag_junction::ActiveModel {
        object_id: Set(1),
        tag_id: Set(1),
        ..Default::default()
    };
    //insret to ObjectTagJunction table
    let inserted_object_tag_data_1 = ObjectTagJunction::insert(object_tag_model_1)
        .exec(&db)
        .await?;
    println!(
        "[INFO]: Inserted to ObjectTagJunction Table: {:?}",
        inserted_object_tag_data_1
    );
    let object_tag_model_2 = object_tag_junction::ActiveModel {
        object_id: Set(1),
        tag_id: Set(2),
        ..Default::default()
    };
    let inserted_object_tag_data_2 = ObjectTagJunction::insert(object_tag_model_2)
        .exec(&db)
        .await?;
    println!(
        "[INFO]: Inserted to ObjectTagJunction Table: {:?}",
        inserted_object_tag_data_2
    );
    println!("\n[INFO]: Insert data to Database was completed!\n");
    //insert initial object file
    let _ = server::upload_image_file(
        r2_manager.clone(),
        "./src/bin/object/object.jpg",
        "obj-1.jpg",
        "image/jpeg",
    )
    .await;
    let _ = server::get_image(r2_manager.clone(), "obj-1.jpg")
        .await
        .expect("upload image was failed.");
    println!("[INFO]: Uploading obj-1.jpg was succeeded.");
    println!("\n[INFO]: Upload image to R2 was completed!\n");
    //insert to meilisearch
    let client = server::connect_meilisearch().await;
    let item_meilisearch = client
        .index("item")
        .add_documents(&meilisearch_item_vec, Some("id"))
        .await
        .unwrap();
    println!(
        "[INFO]: MeiliSearch result of item\n{:#?}",
        item_meilisearch
    );
    //initialize object index (meilisearch)
    let object_model = Object::find_by_id(1)
        .one(&db)
        .await?
        .expect("[ERROR]: Object was not found.");
    let tag_ids = ObjectTagJunction::find()
        .filter(object_tag_junction::Column::ObjectId.eq(1))
        .all(&db)
        .await?
        .into_iter()
        .map(|tag| tag.tag_id)
        .collect::<Vec<i32>>();
    let tag_names = {
        let mut tag_names = Vec::new();
        for tag_id in tag_ids {
            let tag_name = Tag::find_by_id(tag_id)
                .one(&db)
                .await?
                .expect("[ERROR]: Tag was not found.")
                .name;
            tag_names.push(tag_name);
        }
        tag_names
    };
    let object_meilisearch = server::MeiliSearchObjectData {
        id: object_model.id,
        name: object_model.name,
        photo_url: object_model.photo_url,
        mime_type: object_model.mime_type,
        license: object_model.license,
        tag: tag_names,
        description: object_model.description,
        created_at: Utc::now().naive_local().to_string(),
        updated_at: Utc::now().naive_local().to_string(),
    };
    let object_meilisearch = client
        .index("object")
        .add_documents(&[object_meilisearch], Some("id"))
        .await
        .unwrap();
    println!(
        "[INFO]: MeiliSearch result of object\n{:#?}",
        object_meilisearch
    );
    println!("\n[INFO]: Insert data to MeiliSearch was completed!\n");
    //connect neo4j
    let graph = server::connect_neo4j().await;
    //create node to Neo4j
    //gen root
    let root_id = Item::find()
        .filter(item::Column::VisibleId.eq("ROOT".to_string()))
        .one(&db)
        .await?
        .unwrap_or_else(|| panic!("[ERROR]: Id is missing: {}", "ROOT"));
    graph
        .run(query("CREATE (item:Item {id: $id})").param("id", root_id.id))
        .await
        .unwrap_or_else(|_| {
            panic!(
                "[ERROR]: Cannot create node: {} ({})",
                root_id.id, root_id.visible_id
            )
        });
    println!(
        "[INFO]: Inserted to Neo4j: {} ({})",
        root_id.id, root_id.visible_id
    );
    //decendants
    for item in &data {
        let oneself_id = Item::find()
            .filter(item::Column::VisibleId.eq(&item.visible_id))
            .one(&db)
            .await?
            .unwrap_or_else(|| panic!("[ERROR]: Id is missing: {}", &item.visible_id));
        graph
            .run(query("CREATE (item:Item {id: $id})").param("id", oneself_id.id))
            .await
            .unwrap_or_else(|_| {
                panic!(
                    "[ERROR]: Cannot create node: {} ({})",
                    oneself_id.id, item.visible_id
                )
            });
        println!(
            "[INFO]: Inserted to Neo4j: {} ({})",
            oneself_id.id, item.visible_id
        );
    }
    //add relation to Neo4j
    let root_item = Item::find()
        .filter(item::Column::VisibleId.eq("ROOT".to_string()))
        .one(&db)
        .await?
        .unwrap_or_else(|| panic!("[ERROR]: Id is missing: {}", "ROOT"));
    graph
        .run(query("MATCH (child:Item {id: $child_id}) MATCH (parent:Item {id: $parent_id}) CREATE (child)-[relation:ItemTree]->(parent)")
        .param("parent_id", root_item.id).param("child_id", root_item.id))
        .await
        .unwrap_or_else(|_| panic!("[ERROR]: Cannot create relation:  ({})-[relation:ItemTree]->({}) ( ({})-[relation:ItemTree]->({}) )", root_item.id, root_item.id, root_item.visible_id, root_item.visible_id));
    for item in &data {
        //ルートのitemの処理 (実際のルートではないが、csv上ではルートとして扱っている)
        if item.visible_id == item.parent_visible_id {
            let oneself_id = Item::find()
                .filter(item::Column::VisibleId.eq(&item.visible_id))
                .one(&db)
                .await?
                .unwrap_or_else(|| panic!("[ERROR]: Id is missing: {}", &item.visible_id))
                .id;
            graph
                .run(query("MATCH (child:Item {id: $child_id}) MATCH (parent:Item {id: $parent_id}) CREATE (child)-[relation:ItemTree]->(parent)")
                .param("parent_id", root_item.id).param("child_id", oneself_id))
                .await
                .unwrap_or_else(|_| panic!("[ERROR]: Cannot create relation:  ({})-[relation:ItemTree]->({}) ( ({})-[relation:ItemTree]->({}) )", oneself_id, root_item.id, item.visible_id, root_item.visible_id));
            continue;
        }
        //それ以外の処理
        let oneself_id = Item::find()
            .filter(item::Column::VisibleId.eq(&item.visible_id))
            .one(&db)
            .await?
            .unwrap_or_else(|| panic!("[ERROR]: Id is missing: {}", &item.visible_id))
            .id;
        let parent_id = Item::find()
            .filter(item::Column::VisibleId.eq(&item.parent_visible_id))
            .one(&db)
            .await?
            .unwrap_or_else(|| panic!("[ERROR]: Parent Id is missing: {}", &item.visible_id))
            .id;
        graph
            .run(query("MATCH (child:Item {id: $child_id}) MATCH (parent:Item {id: $parent_id}) CREATE (child)-[relation:ItemTree]->(parent)")
            .param("parent_id", parent_id).param("child_id", oneself_id))
            .await
            .unwrap_or_else(|_| panic!("[ERROR]: Cannot create relation:  ({})-[relation:ItemTree]->({}) ( ({})-[relation:ItemTree]->({}) )", oneself_id, parent_id, item.visible_id, item.parent_visible_id));
        println!("[INFO]: Inserted to Neo4j: ({})-[relation:ItemTree]->({}) ( ({})-[relation:ItemTree]->({}) )", oneself_id, parent_id, item.visible_id, item.parent_visible_id);
    }
    println!("\n[INFO]: Insert data to Neo4j was completed!\n");
    Ok(())
}
