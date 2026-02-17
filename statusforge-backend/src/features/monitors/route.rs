use axum::{extract::Path, extract::Query, extract::State, routing::{delete, get, post, put}, Json, Router};
use serde_json::Value;

use crate::{AppState, ApiError};

use super::{service, CreateMonitor, UpdateMonitor, CreateMonitorResult, ListMonitorResultsQuery, CheckMonitorRequest};

async fn list_monitors(
    State(state): State<AppState>,
    Path(project_id): Path<String>,
) -> Result<Json<Value>, ApiError> {
    let monitors = service::list_monitors(&state, &project_id).await?;
    Ok(Json(serde_json::to_value(monitors).unwrap()))
}

async fn get_monitor(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Value>, ApiError> {
    let monitor = service::get_monitor(&state, &id).await?;
    Ok(Json(serde_json::to_value(monitor).unwrap()))
}

async fn create_monitor(
    State(state): State<AppState>,
    Path(project_id): Path<String>,
    Json(data): Json<CreateMonitor>,
) -> Result<Json<Value>, ApiError> {
    let monitor = service::create_monitor(&state, &project_id, data).await?;
    Ok(Json(serde_json::to_value(monitor).unwrap()))
}

async fn update_monitor(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(data): Json<UpdateMonitor>,
) -> Result<Json<Value>, ApiError> {
    let monitor = service::update_monitor(&state, &id, data).await?;
    Ok(Json(serde_json::to_value(monitor).unwrap()))
}

async fn delete_monitor(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Value>, ApiError> {
    service::delete_monitor(&state, &id).await?;
    Ok(Json(serde_json::json!({ "message": "Monitor deleted" })))
}

async fn create_monitor_result(
    State(state): State<AppState>,
    Path(monitor_id): Path<String>,
    Json(data): Json<CreateMonitorResult>,
) -> Result<Json<Value>, ApiError> {
    let result = service::create_monitor_result(&state, &monitor_id, data).await?;
    Ok(Json(serde_json::to_value(result).unwrap()))
}

async fn list_monitor_results(
    State(state): State<AppState>,
    Path(monitor_id): Path<String>,
    Query(query): Query<ListMonitorResultsQuery>,
) -> Result<Json<Value>, ApiError> {
    let results = service::list_monitor_results(&state, &monitor_id, query).await?;
    Ok(Json(serde_json::to_value(results).unwrap()))
}

async fn check_monitor(
    State(state): State<AppState>,
    Path(monitor_id): Path<String>,
    Query(query): Query<CheckMonitorRequest>,
) -> Result<Json<Value>, ApiError> {
    let result = service::run_monitor_check(&state, &monitor_id, query.region.as_deref()).await?;
    Ok(Json(serde_json::to_value(result).unwrap()))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/projects/:project_id/monitors", get(list_monitors).post(create_monitor))
        .route("/monitors/:id", get(get_monitor).put(update_monitor).delete(delete_monitor))
        .route("/monitors/:id/check", post(check_monitor))
        .route("/monitors/:monitor_id/results", post(create_monitor_result).get(list_monitor_results))
}
