use ::entity::{item, item::Entity as Item};
use axum::{routing::get, Extension, Router};
use cloudflare_r2_rs::r2;
use dotenvy::dotenv;
use once_cell::sync::OnceCell;
use sea_orm::*;
use std::env;

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
    //Router
    let app = Router::new()
        .route("/", get(sample_handler))
        .layer(Extension(db))
        .layer(Extension(r2_manager))
        .layer(Extension(r2_url));
    //Server
    dotenv().expect(".env file not found.");
    static API_URL: OnceCell<String> = OnceCell::new();
    let _ = API_URL.set(env::var("API_URL").expect("KEY not found in .env file."));
    let listener = tokio::net::TcpListener::bind(API_URL.get().expect("Failed to get API_URL"))
        .await
        .unwrap();
    println!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

//Handler
async fn sample_handler() -> String {
    "Hello, World".to_string()
}
