use ::entity::item::{self, Entity as Item};
use dotenvy::dotenv;
use neo4rs::{query, Graph, Path, Row};
use once_cell::sync::OnceCell;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use std::{collections::HashSet, env};

use crate::AppError;

pub async fn connect_neo4j() -> Graph {
    // Set environment variables
    // Declaration and initialization of static variable
    static NEO4J_BOLT_PORT: OnceCell<String> = OnceCell::new();
    static NEO4J_USER: OnceCell<String> = OnceCell::new();
    static NEO4J_PASSWORD: OnceCell<String> = OnceCell::new();
    // load .env file
    dotenv().expect(".env file not found.");
    // set Object value
    let _ = NEO4J_BOLT_PORT.set(env::var("NEO4J_BOLT_PORT").expect("KEY not found in .env file."));
    let _ = NEO4J_USER.set(env::var("NEO4J_USER").expect("KEY not found in .env file."));
    let _ = NEO4J_PASSWORD.set(env::var("NEO4J_PASSWORD").expect("KEY not found in .env file."));
    //インスタンスの作成
    Graph::new(
        format!(
            "neo4j:{}",
            // "localhost:{}",
            NEO4J_BOLT_PORT
                .get()
                .expect("Failed to get NEO4J_BOLT_PORT")
        ),
        NEO4J_USER.get().expect("Failed to get NEO4J_USER"),
        NEO4J_PASSWORD.get().expect("Failed to get NEO4J_PASSWORD"),
    )
    .await
    .expect("Cannot connect to Graph")
}

pub async fn search_path(graph: &Graph, oneself_id: i64) -> Result<Vec<i64>, AppError> {
    let mut result = graph
        .execute(query("MATCH path=({id:$id})-[*]->() RETURN path").param("id", oneself_id))
        .await
        .unwrap();
    let mut row: Result<Row, AppError> = match result.next().await.unwrap() {
        Some(row) => Ok(row),
        None => Err(AppError(anyhow::anyhow!("Row Object was not found."))),
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
                .expect("Cannot get value of 'item' (key name).")
        })
        .collect();
    Ok(ids)
}

pub struct ParentId {
    pub actual_parent_id: i64,
    pub is_actual_root: bool,
}

pub async fn search_parent_id(
    db: &DatabaseConnection,
    graph: &Graph,
    oneself_id: i64,
) -> Result<ParentId, AppError> {
    let mut result = graph
        .execute(query("MATCH path=({id:$id})-[:ItemTree]->() RETURN path").param("id", oneself_id))
        .await
        .unwrap();
    let row = match result.next().await.unwrap() {
        Some(row) => Ok(row),
        None => Err(AppError(anyhow::anyhow!("Row not found."))),
    };
    let id: Vec<i64> = row?
        .get::<Path>("path")
        .unwrap()
        .nodes()
        .into_iter()
        .map(|node| {
            node.get::<i64>("id")
                .expect("Cannot get value of 'item' (key name).")
        })
        .collect();
    let mut is_actual_root = false;
    if id.len() == 1 {
        return Err(AppError(anyhow::anyhow!("Root item cannot be changed.")));
    }
    let parent_item = Item::find_by_id::<i32>(id[1] as i32)
        .one(db)
        .await?
        .ok_or(AppError(anyhow::anyhow!("Root item was not found.")))?;
    if parent_item.visible_id == "ROOT" {
        is_actual_root = true;
    }
    Ok(ParentId {
        actual_parent_id: id[1],
        is_actual_root,
    })
}

pub async fn search_descendants_ids_hashset(
    graph: &Graph,
    oneself_id: i64,
) -> Result<HashSet<i64>, AppError> {
    let mut result = graph
        .execute(query("MATCH path=()-[*]->({id:$id}) RETURN path").param("id", oneself_id))
        .await
        .unwrap();
    let mut descendants: HashSet<i64> = HashSet::new();
    loop {
        let row = match result.next().await.unwrap() {
            Some(row) => row,
            None => break,
        };
        let ids: Vec<i64> = row
            .get::<Path>("path")
            .unwrap()
            .nodes()
            .into_iter()
            .map(|node| {
                node.get::<i64>("id")
                    .expect("Cannot get value of 'item' (key name).")
            })
            .collect();
        for id in ids {
            descendants.insert(id);
        }
    }
    Ok(descendants)
}

async fn search_descendants_ids_vec(graph: &Graph, oneself_id: i64) -> Result<Vec<i64>, AppError> {
    let mut result = graph
        .execute(query("MATCH path=()-[*]->({id:$id}) RETURN path").param("id", oneself_id))
        .await
        .unwrap();
    let mut descendants: Vec<i64> = Vec::new();
    loop {
        let row = match result.next().await.unwrap() {
            Some(row) => row,
            None => break,
        };
        let ids: Vec<i64> = row
            .get::<Path>("path")
            .unwrap()
            .nodes()
            .into_iter()
            .map(|node| {
                node.get::<i64>("id")
                    .expect("Cannot get value of 'item' (key name).")
            })
            .collect();
        for id in ids {
            descendants.push(id);
        }
    }
    Ok(descendants)
}

pub struct AllIds {
    pub actual_route_id: i32,
    pub descendants: Vec<i32>,
}

pub async fn get_all_ids(db: &DatabaseConnection, graph: &Graph) -> Result<Vec<AllIds>, AppError> {
    let root_item = Item::find()
        .filter(item::Column::VisibleId.eq("ROOT"))
        .one(db)
        .await?
        .ok_or(AppError(anyhow::anyhow!("Root item was not found.")))?;
    let actual_routes_ids = search_children_ids(graph, root_item.id.into()).await?;
    let all_ids = {
        let mut all_ids: Vec<AllIds> = Vec::new();
        for actual_route_id in actual_routes_ids {
            let descendants = search_descendants_ids_vec(graph, actual_route_id).await?;
            all_ids.push(AllIds {
                actual_route_id: actual_route_id as i32,
                descendants: descendants.into_iter().map(|id| id as i32).collect(),
            });
        }
        all_ids
    };
    Ok(all_ids)
}

pub async fn search_children_ids(graph: &Graph, oneself_id: i64) -> Result<Vec<i64>, AppError> {
    let mut result = graph
        .execute(
            query("MATCH path=()-[:ItemTree*1]->({id:$oneself_id}) RETURN path")
                .param("oneself_id", oneself_id),
        )
        .await
        .unwrap();
    let mut children: Vec<i64> = Vec::new();
    loop {
        let row = match result.next().await.unwrap() {
            Some(row) => row,
            None => break,
        };
        let mut ids: Vec<i64> = row
            .get::<Path>("path")
            .unwrap()
            .nodes()
            .into_iter()
            .map(|node| {
                node.get::<i64>("id")
                    .expect("Cannot get value of 'item' (key name).")
            })
            .collect();
        if ids.len() <= 2 {
            ids.pop();
            println!("ids: {:?}", ids);
            for id in ids {
                children.push(id);
            }
        }
    }
    Ok(children)
}

pub async fn reconnect_new_parent_item(
    graph: &Graph,
    old_parent_id: i64,
    new_parent_id: i64,
    oneself_id: i64,
) -> Result<(), AppError> {
    graph
        .run(query("MATCH ({id:$child_id})-[r:ItemTree]->({id:$old_parent_id}) DELETE r WITH r MATCH (child:Item {id:$child_id}) MATCH (parent:Item {id:$new_parent_id}) CREATE (child)-[:ItemTree]->(parent)")
        .param("old_parent_id", old_parent_id).param("new_parent_id", new_parent_id).param("child_id", oneself_id))
        .await
        .unwrap();
    Ok(())
}

pub async fn create_single_item(graph: &Graph, oneself_id: i64) -> Result<(), AppError> {
    graph
        .run(query("CREATE (item:Item {id: $id})").param("id", oneself_id))
        .await
        .unwrap();
    Ok(())
}

pub async fn delete_item(graph: &Graph, delete_id: i64) -> Result<(), AppError> {
    graph
        .run(
            query("MATCH (item:Item {id:$delete_id}) DETACH DELETE item")
                .param("delete_id", delete_id),
        )
        .await
        .unwrap();
    Ok(())
}

pub async fn connect_items(graph: &Graph, parent_id: i64, child_id: i64) -> Result<(), AppError> {
    graph
        .run(query("MATCH (child:Item {id: $child_id}) MATCH (parent:Item {id: $parent_id}) CREATE (child)-[relation:ItemTree]->(parent)")
        .param("parent_id", parent_id).param("child_id", child_id))
        .await.unwrap();
    Ok(())
}

pub async fn is_item_exits(graph: &Graph, parent_id: i64) -> Result<bool, AppError> {
    let mut parent_item = graph
        .execute(query("MATCH item=({id:$parent_id}) RETURN item").param("parent_id", parent_id))
        .await
        .unwrap();
    let result = parent_item.next().await.unwrap().is_some();
    Ok(result)
}
