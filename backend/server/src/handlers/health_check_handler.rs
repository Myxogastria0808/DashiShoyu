use axum::http::StatusCode;
use axum::{Extension, Json};
use neo4rs::Graph;
use sea_orm::DatabaseConnection;
use serde::Serialize;
use server::AppError;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct HelthCheckResponse {
    status_code: String,
    message: String,
}

#[utoipa::path(
    get,
    path = "/api",
    responses(
        (status = 200, description = "OK", body = HelthCheckResponse)
    ),
    tag = "Health Check",
)]
pub async fn health_check_get(
    Extension(db): Extension<DatabaseConnection>,
    Extension(graph): Extension<Graph>,
    Extension(meilisearch_client): Extension<meilisearch_sdk::client::Client>,
) -> Result<Json<HelthCheckResponse>, AppError> {
    //RDB
    db.ping().await?;
    //Graph DB
    let _ = server::search_path(&graph, 1).await?;
    //Meilisearch
    let _ = meilisearch_client.health().await?;
    Ok(Json(HelthCheckResponse {
        status_code: format!("{}", StatusCode::OK),
        message: "DashiShoyu server is running.".to_string(),
    }))
}
