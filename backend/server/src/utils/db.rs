use dotenvy::dotenv;
use once_cell::sync::OnceCell;
use sea_orm::{self, Database, DatabaseConnection, DbErr};
use std::env;

pub async fn connect_db() -> Result<DatabaseConnection, DbErr> {
    // Declaration and initialization of static variable
    static DATABASE_URL: OnceCell<String> = OnceCell::new();
    // load .env file
    dotenv().expect(".env file not found.");
    // set Object value
    let _ = DATABASE_URL.set(env::var("DATABASE_URL").expect("KEY not found in .env file."));
    // connnect database
    Database::connect(DATABASE_URL.get().expect("Failed to get API_URL")).await
}
