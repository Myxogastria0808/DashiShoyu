use dotenvy::dotenv;
use meilisearch_sdk::client::Client;
use once_cell::sync::OnceCell;
use std::env;

pub async fn connect_meilisearch() -> Client {
    // Set environment variables
    // Declaration and initialization of static variable
    static MEILI_URL: OnceCell<String> = OnceCell::new();
    static ADMIN_API_KEY: OnceCell<String> = OnceCell::new();
    // load .env file
    dotenv().expect(".env file not found.");
    // set Object value
    let _ = MEILI_URL.set(env::var("MEILI_URL").expect("KEY not found in .env file."));
    let _ = ADMIN_API_KEY.set(env::var("ADMIN_API_KEY").expect("KEY not found in .env file."));
    //インスタンスの作成
    Client::new(
        MEILI_URL.get().expect("Failed to get MEILI_URL"),
        Some(ADMIN_API_KEY.get().expect("Failed to get ADMIN_API_KEY")),
    )
    .expect("Cannot connect to MeiliSearch")
}

pub async fn get_meilisearch_admin_api_key() -> String {
    static ADMIN_API_KEY: OnceCell<String> = OnceCell::new();
    dotenv().expect(".env file not found.");
    let _ = ADMIN_API_KEY.set(env::var("ADMIN_API_KEY").expect("KEY not found in .env file."));
    ADMIN_API_KEY
        .get()
        .expect("Failed to get ADMIN_API_KEY")
        .to_string()
}

pub async fn get_meilisearch_url() -> String {
    static MEILI_URL: OnceCell<String> = OnceCell::new();
    dotenv().expect(".env file not found.");
    let _ = MEILI_URL.set(env::var("MEILI_URL").expect("KEY not found in .env file."));
    MEILI_URL
        .get()
        .expect("Failed to get MEILI_URL")
        .to_string()
}
