use dotenvy::dotenv;
use once_cell::sync::OnceCell;
use sea_orm::{self, Database, DatabaseConnection, DbErr};
use std::env;

pub async fn connect_db() -> Result<DatabaseConnection, DbErr> {
    // Declaration and initialization of static variable
    static POSTGRES_USER: OnceCell<String> = OnceCell::new();
    static POSTGRES_PASSWORD: OnceCell<String> = OnceCell::new();
    static POSTGRES_PORT: OnceCell<String> = OnceCell::new();
    static POSTGRES_DB: OnceCell<String> = OnceCell::new();
    // load .env file
    dotenv().expect(".env file not found.");
    // set Object value
    let _ = POSTGRES_USER
        .set(env::var("POSTGRES_USER").expect("POSTGRES_USER not found in .env file."));
    let _ = POSTGRES_PASSWORD
        .set(env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD not found in .env file."));
    let _ = POSTGRES_PORT
        .set(env::var("POSTGRES_PORT").expect("POSTGRES_PORT not found in .env file."));
    let _ = POSTGRES_DB.set(env::var("POSTGRES_DB").expect("POSTGRES_DB not found in .env file."));
    // connnect database
    Database::connect(format!(
        "postgres://{}:{}@postgres:{}/{}",
        // "postgres://{}:{}@localhost:{}/{}",
        POSTGRES_USER.get().expect("Failed to get POSTGRES_USER"),
        POSTGRES_PASSWORD
            .get()
            .expect("Failed to get POSTGRES_PASSWORD"),
        POSTGRES_PORT.get().expect("Failed to get  POSTGRES_PORT"),
        POSTGRES_DB.get().expect("Failed to get POSTGRES_DB"),
    ))
    .await
}
