use crate::handlers::item_handlers::{
    delete_item_delete, generate_csv_get, generate_visible_ids_post, get_each_item_get,
    register_item_post, search_item_get, update_item_put,
};
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
        .route("/register", post(register_item_post))
        .route("/get/csv-data", get(generate_csv_get))
        .route(
            "/generate/visible-id/:number",
            post(generate_visible_ids_post),
        );
    let routes = Router::new().merge(item_routes);
    Router::new().merge(Router::new().nest("/item", routes))
}
