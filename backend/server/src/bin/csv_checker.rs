use ::entity::{item::Record, label::Color};
use csv::Error;
use regex::Regex;
use serde::{Deserialize, Serialize};
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

fn main() {
    match csv_checker() {
        Ok(data) => {
            println!("{:#?}", data);
            println!("Success!");
        }
        Err(e) => {
            eprintln!("[Error]: {}", e);
            process::exit(1);
        }
    }
}

fn parse_csv() -> Result<Vec<CsvItemData>, Box<Error>> {
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

fn check_csv(data: Vec<CsvItemData>) -> Result<Vec<InsertData>, Box<Error>> {
    //insert item data
    let mut result_vec: Vec<InsertData> = Vec::new();
    let mut visible_id_vec: Vec<(usize, String)> = Vec::new();
    for (index, item) in data.iter().enumerate() {
        println!("----------------------------------");
        println!(
            "Checking item csv line: {}, visible_id: {}",
            index + 2,
            item.visible_id
        );
        //visible_idに被りがないかチェック
        visible_id_vec.push((index, item.visible_id.clone()));
        for (i, visible_id) in visible_id_vec.iter() {
            for j in 0..*i {
                let item = &visible_id_vec[j];
                if *visible_id == item.1 {
                    panic!(
                        "Duplicate visible_id: {}, Duplicate visible_id line: {}, {}",
                        item.1,
                        j + 2,
                        index + 2
                    );
                }
            }
        }
        //visible_idが正しい形式かチェック
        //1. 4文字であるかチェック
        if 4 != item.visible_id.len() {
            panic!("Invalid visible_id: {}", item.visible_id);
        }
        //2. 英数字であるかチェック
        let re = Regex::new(r"^[A-Z0-9]{4}$").unwrap();
        if !re.is_match(&item.visible_id) {
            panic!("Invalid visible_id: {}", item.visible_id);
        }
        let item_data = InsertData {
            visible_id: item.visible_id.clone(),
            parent_visible_id: item.parent_visible_id.clone(),
            name: item.name.clone(),
            product_number: match item.product_number.clone() {
                Some(content) => content,
                None => "".to_string(),
            },
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
            description: match item.description.clone() {
                Some(content) => content,
                None => "".to_string(),
            },
            year_purchased: item.year_purchased,
            connector: vec![
                item.connector_1.clone().unwrap_or_default(),
                item.connector_2.clone().unwrap_or_default(),
                item.connector_3.clone().unwrap_or_default(),
                item.connector_4.clone().unwrap_or_default(),
                item.connector_5.clone().unwrap_or_default(),
                item.connector_6.clone().unwrap_or_default(),
            ],
        };
        result_vec.push(item_data);
        println!("Pass item csv line: {}", index + 2);
        println!("----------------------------------");
    }
    Ok(result_vec)
}

fn csv_checker() -> Result<Vec<InsertData>, Box<Error>> {
    let data = parse_csv()?;
    let data = check_csv(data)?;
    Ok(data)
}
