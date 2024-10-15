use crate::handlers::object_handlers::{
    delete_object_delete, get_each_object_get, get_object_with_tag_get, register_object_post,
    search_object_get, update_object_put,
};
use axum::{
    routing::{delete, get, post, put},
    Router,
};

pub async fn object_routes() -> Router {
    let object_routes = Router::new()
        .route("/search", get(search_object_get))
        .route("/get/:id", get(get_each_object_get))
        .route("/tag/:tag", get(get_object_with_tag_get))
        .route("/register", post(register_object_post))
        .route("/update/:id", put(update_object_put))
        .route("/delete/:id", delete(delete_object_delete));
    let routes = Router::new().merge(object_routes);
    Router::new().merge(Router::new().nest("/object", routes))
}
