use crate::handlers::csv_handlers::csv_get;
use crate::handlers::item_handlers::{
    delete_item_delete, get_each_item_get, register_item_post, search_item_get, update_item_put,
};
use crate::handlers::validate_visible_id_handlers::validate_visible_id_post;
use axum::{
    routing::{delete, get, post, put},
    Router,
};

pub async fn item_routes() -> Router {
    let csv_routes = Router::new().route("/csv", get(csv_get));
    let validate_visible_id_routes =
        Router::new().route("/validation", post(validate_visible_id_post));
    let item_routes = Router::new()
        .route("/search", get(search_item_get))
        .route("/get/:id", get(get_each_item_get))
        .route("/update/:id", put(update_item_put))
        .route("/delete/:id", delete(delete_item_delete))
        .route("/register", post(register_item_post));
    let routes = Router::new()
        .merge(csv_routes)
        .merge(validate_visible_id_routes)
        .merge(item_routes);
    Router::new().merge(Router::new().nest("/item", routes))
}
