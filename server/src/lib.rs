mod utils;
//db
pub use utils::db::connect_db;
//r2
pub use utils::r2::connect_r2;
pub use utils::r2::delete_image;
pub use utils::r2::get_image;
pub use utils::r2::get_r2_url;
pub use utils::r2::upload_image;
//image
pub use utils::image::convert_to_webp;
//meilisearch
pub use utils::meilisearch::connect_meilisearch;

//model
mod models;
pub use models::item_models::MeilisearchItemData;

//error
mod error;
pub use error::handler_error::AppError;
