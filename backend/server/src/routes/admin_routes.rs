use crate::handlers::admin_handlers::regiter_visible_id_post;
use axum::{
    routing::{delete, get, post, put},
    Router,
};

pub async fn admin_routes() -> Router {
    let item_routes =
        Router::new().route("/item/visible-id/register", post(regiter_visible_id_post));
    Router::new().merge(Router::new().nest("/admin", item_routes))
}
