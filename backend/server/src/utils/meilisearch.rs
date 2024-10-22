use dotenvy::dotenv;
use meilisearch_sdk::client::Client;
use once_cell::sync::OnceCell;
use std::env;

pub async fn connect_meilisearch() -> Client {
    // Set environment variables
    // Declaration and initialization of static variable
    static MEILI_PORT: OnceCell<String> = OnceCell::new();
    static MEILI_MASTER_KEY: OnceCell<String> = OnceCell::new();
    // load .env file
    dotenv().expect(".env file not found.");
    // set Object value
    let _ = MEILI_PORT.set(env::var("MEILI_PORT").expect("KEY not found in .env file."));
    let _ =
        MEILI_MASTER_KEY.set(env::var("MEILI_MASTER_KEY").expect("KEY not found in .env file."));
    //インスタンスの作成
    Client::new(
        format!(
            "http://meilisearch:{}",
            // "http://localhost:{}",
            MEILI_PORT.get().expect("Failed to get MEILI_PORT")
        ),
        Some(
            MEILI_MASTER_KEY
                .get()
                .expect("Failed to get MEILI_MASTER_KEY"),
        ),
    )
    .expect("Cannot connect to MeiliSearch")
}

pub async fn get_meilisearch_url() -> String {
    static MEILI_PORT: OnceCell<String> = OnceCell::new();
    dotenv().expect(".env file not found.");
    let _ = MEILI_PORT.set(env::var("MEILI_PORT").expect("KEY not found in .env file."));
    format!(
        "http://meilisearch:{}",
        // "http://localhost:{}",
        MEILI_PORT.get().expect("Failed to get MEILI_PORT")
    )
}
