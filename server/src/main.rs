use crate::routes::item_routes;
use axum::{Extension, Router};
use cloudflare_r2_rs::r2;
use dotenvy::dotenv;
use once_cell::sync::OnceCell;
use sea_orm::*;
use std::env;

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
    //Router
    let app = Router::new()
        .merge(item_routes::item_routes().await)
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
