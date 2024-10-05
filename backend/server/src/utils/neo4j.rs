use dotenvy::dotenv;
use neo4rs::{query, DeError, Graph, Path, Row};
use once_cell::sync::OnceCell;
use std::env;

use crate::AppError;

pub async fn connect_neo4j() -> Graph {
    // Set environment variables
    // Declaration and initialization of static variable
    static NEO4J_BOLT_URL: OnceCell<String> = OnceCell::new();
    static NEO4J_USER: OnceCell<String> = OnceCell::new();
    static NEO4J_PASSWORD: OnceCell<String> = OnceCell::new();
    // load .env file
    dotenv().expect(".env file not found.");
    // set Object value
    let _ = NEO4J_BOLT_URL.set(env::var("NEO4J_BOLT_URL").expect("KEY not found in .env file."));
    let _ = NEO4J_USER.set(env::var("NEO4J_USER").expect("KEY not found in .env file."));
    let _ = NEO4J_PASSWORD.set(env::var("NEO4J_PASSWORD").expect("KEY not found in .env file."));
    //インスタンスの作成
    Graph::new(&*NEO4J_BOLT_URL.get().expect("Failed to get NEO4J_BOLT_URL"), NEO4J_USER.get().expect("Failed to get NEO4J_USER"), NEO4J_PASSWORD.get().expect("Failed to get NEO4J_PASSWORD")).await.expect("Cannot connect to Graph")
}

pub async fn search_path(graph: Graph, descendant_node_id: i64) -> Result<Vec<i64>, AppError> {
    let mut result = graph
        .execute(query("MATCH path=({id:$id})-[*]->() RETURN path").param("id", descendant_node_id))
        .await.unwrap();
    let mut row: Result<Row, AppError> = match result.next().await.unwrap() {
        Some(row) => Ok(row),
        None => Err(AppError(anyhow::anyhow!("[ERROR]: Row Object was not found."))),
    };
    loop {
        row = match result.next().await.unwrap() {
            Some(row) => Ok(row),
            None => break,
        }
    }
    let ids: Vec<i64> = row?
        .get::<Path>("path")
        .unwrap()
        .nodes()
        .into_iter()
        .map(|node| {
            node.get::<i64>("id")
                .expect("[ERROR]: Cannot get value of 'item' (key name).")
        })
        .collect();
    Ok(ids)
}