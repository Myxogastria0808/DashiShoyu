use crate::routes::index;
use axum::{extract::DefaultBodyLimit, http::Method, routing::get, Extension, Router};
use cloudflare_r2_rs::r2;
use dotenvy::dotenv;
use once_cell::sync::OnceCell;
use sea_orm::{self, DatabaseConnection, DbErr};
use std::env;
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod handlers;
mod routes;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let _ = api().await;
}

//axum
async fn api() -> Result<(), DbErr> {
    // connect db
    let db: DatabaseConnection = server::connect_db().await?;
    // connect r2
    let r2_manager: r2::R2Manager = server::connect_r2().await;
    // r2 URL
    let r2_url: String = server::get_r2_url().await;
    // MeiliSearch
    let meilisearch_client: meilisearch_sdk::client::Client = server::connect_meilisearch().await;
    //meilisearch_url
    let meilisearch_url: String = server::get_meilisearch_url().await;
    //connect neo4j
    let graph: neo4rs::Graph = server::connect_neo4j().await;
    //CORS
    let cors: CorsLayer = CorsLayer::new()
        .allow_methods([Method::POST, Method::GET, Method::DELETE, Method::PUT])
        .allow_origin(Any);
    //port
    dotenv().expect(".env file not found.");
    static SERVER_PORT: OnceCell<String> = OnceCell::new();
    let _ = SERVER_PORT.set(env::var("SERVER_PORT").expect("KEY not found in .env file."));
    //Router
    let app = Router::new()
        .route("/", get(ping))
        .merge(index::root_routes().await)
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(Extension(db))
        .layer(Extension(r2_manager))
        .layer(Extension(r2_url))
        .layer(Extension(meilisearch_client))
        .layer(Extension(meilisearch_url))
        .layer(Extension(graph))
        .layer(cors)
        .layer(DefaultBodyLimit::max(1024 * 1024 * 100));
    //Server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();
    println!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

//* dummy *//
async fn ping() -> String {
    "pong!".to_string()
}

#[derive(OpenApi)]
#[openapi(
    info(title = "DashiShoyu"),
    servers((url = "http://0.0.0.0:5000")),
    tags(
        (name = "Health Check", description = "Health Checkのエンドポイント"),
        (name = "Item", description = "物品に関係するエンドポイント"),
        (name = "Object", description = "オブジェクトに関係するエンドポイント"),
    ),
    paths(
        handlers::health_check_handler::health_check_get,
        handlers::item_handlers::search_item_get,
        handlers::item_handlers::get_each_item_get,
        handlers::item_handlers::update_item_put,
        handlers::item_handlers::register_item_post,
        handlers::item_handlers::delete_item_delete,
        handlers::item_handlers::generate_visible_ids_post,
        handlers::item_handlers::generate_csv_get,
        handlers::object_handlers::search_object_get,
        handlers::object_handlers::get_each_object_get,
        handlers::object_handlers::get_object_with_tag_get,
        handlers::object_handlers::register_object_post,
        handlers::object_handlers::update_object_put,
        handlers::object_handlers::delete_object_delete,
    ),
    components(schemas(
        handlers::health_check_handler::HelthCheckResponse,
        server::MeiliSearchItemData,
        server::ItemData,
        server::ControlItemData,
        server::CsvItemData,
        server::ObjectData,
        server::MeiliSearchObjectData,
        server::DeleteItemData,
        server::RegisterObjectData,
        server::UpdateObjectData,
        ::entity::item::Record,
        ::entity::label::Color,
        ::entity::label::Model,
    ))
)]

struct ApiDoc;
