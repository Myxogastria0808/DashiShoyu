use axum::Json;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

pub struct AppError(pub anyhow::Error);

//anyhow::error => AppError への型変換
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

//AppError => axum::response::Response への型変換
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        Json(json!({
            "status_code": format!("{}", StatusCode::INTERNAL_SERVER_ERROR),
            "message": format!("Internal Server Error: {}", self.0),
        }))
        .into_response()
    }
}
