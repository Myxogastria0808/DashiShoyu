use axum::Router;

use crate::routes::item_routes::item_routes;

pub async fn root_routes() -> Router {
    let routes = Router::new().merge(item_routes().await);
    Router::new().merge(Router::new().nest("/api", routes))
}
