use axum::{extract::State, http::StatusCode};
use registry::AppRegistry;

#[utoipa::path(get, path = "/health")]
pub async fn health_check() -> StatusCode {
    StatusCode::OK
}

#[utoipa::path(get, path = "/health/db")]
pub async fn health_check_db(State(registry): State<AppRegistry>) -> StatusCode {
    if registry.health_check_repository().check_db().await {
        StatusCode::OK
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}
