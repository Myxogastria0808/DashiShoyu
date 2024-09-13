use cloudflare_r2_rs::r2::R2Manager;
use dotenvy::dotenv;
use once_cell::sync::OnceCell;
use std::env;
use std::error::Error;
use tokio::{fs::File, io::AsyncReadExt};

pub async fn connect_r2() -> R2Manager {
    // Set environment variables
    // Declaration and initialization of static variable
    static BUCKET_NAME: OnceCell<String> = OnceCell::new();
    static CLOUDFLARE_URI_ENDPOINT: OnceCell<String> = OnceCell::new();
    static API_TOKENS_ACCESS_KEY_ID: OnceCell<String> = OnceCell::new();
    static API_TOKENS_SECRET_ACCESS_KEY: OnceCell<String> = OnceCell::new();
    // load .env file
    dotenv().expect(".env file not found.");
    // set Object value
    let _ = BUCKET_NAME.set(env::var("BUCKET_NAME").expect("KEY not found in .env file."));
    let _ = CLOUDFLARE_URI_ENDPOINT
        .set(env::var("CLOUDFLARE_URI_ENDPOINT").expect("KEY not found in .env file."));
    let _ = API_TOKENS_ACCESS_KEY_ID
        .set(env::var("API_TOKENS_ACCESS_KEY_ID").expect("KEY not found in .env file."));
    let _ = API_TOKENS_SECRET_ACCESS_KEY
        .set(env::var("API_TOKENS_SECRET_ACCESS_KEY").expect("KEY not found in .env file."));
    //インスタンスの作成
    R2Manager::new(
        //Bucket Name
        BUCKET_NAME.get().unwrap(),
        //Cloudflare URI endpoint
        CLOUDFLARE_URI_ENDPOINT.get().unwrap(),
        //API Token's Access Key ID
        API_TOKENS_ACCESS_KEY_ID.get().unwrap(),
        //API Token's Secret Access Key
        API_TOKENS_SECRET_ACCESS_KEY.get().unwrap(),
    )
    .await
}

//get R2 URL
pub async fn get_r2_url() -> String {
    // Set environment variables
    // Declaration and initialization of static variable
    static R2_URL: OnceCell<String> = OnceCell::new();
    // load .env file
    dotenv().expect(".env file not found.");
    // set Object value
    let _ = R2_URL.set(env::var("R2_URL").expect("KEY not found in .env file."));
    R2_URL.get().expect("Cannot get R2_URL").to_string()
}

//upload
pub async fn upload_image_file(
    r2_manager: R2Manager,
    file_path: &str,
    file_name: &str,
    file_type: &str,
) -> Result<(), Box<dyn Error>> {
    let binary = convert_image_to_binary(file_path).await?;
    r2_manager
        .upload(file_name, &binary[..], Some("max-age=60"), Some(file_type))
        .await;
    Ok(())
}

async fn convert_image_to_binary(file_path: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    //read file
    let mut file = File::open(file_path).await?;
    //read binary
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).await?;
    Ok(buffer)
}

//get
pub async fn get_image(r2_manager: R2Manager, file_name: &str) -> Option<Vec<u8>> {
    r2_manager.get(file_name).await
}

//delete
pub async fn delete_image(r2_manager: R2Manager, file_name: &str) {
    r2_manager.delete(file_name).await;
}

//check_is_uploaded
pub async fn check_is_uploaded(r2_manager: R2Manager, file_name: &str) -> bool {
    r2_manager.get(file_name).await.is_some()
}
