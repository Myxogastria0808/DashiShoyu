use axum::http::StatusCode;
use axum::Json;
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
pub async fn health_check_get() -> Result<Json<HelthCheckResponse>, AppError> {
    Ok(Json(HelthCheckResponse {
        status_code: format!("{}", StatusCode::OK),
        message: "DashiShoyu server is running.".to_string(),
    }))
}
