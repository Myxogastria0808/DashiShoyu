use crate::handlers::health_check_handler::health_check_get;
use crate::routes::admin_routes::admin_routes;
use crate::routes::item_routes::item_routes;
use crate::routes::object_routes::object_routes;
use axum::routing::get;
use axum::Router;

pub async fn root_routes() -> Router {
    let routes = Router::new()
        .route("/", get(health_check_get))
        .merge(item_routes().await)
        .merge(admin_routes().await)
        .merge(object_routes().await);
    Router::new().merge(Router::new().nest("/api", routes))
}
