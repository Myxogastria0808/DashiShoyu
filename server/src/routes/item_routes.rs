use crate::handlers::item_handlers::*;

use axum::{
    routing::{delete, get, post, put},
    Router,
};

pub async fn item_routes() -> Router {
    let item_routes = Router::new()
        .route("/search", get(search_item_get))
        .route("/get/:id", get(get_each_item_get))
        .route("/update/:id", put(update_item_put))
        .route("/delete/:id", delete(delete_item_delete))
        .route("/register", post(register_item_post));
    Router::new().merge(Router::new().nest("/item", item_routes))
}
