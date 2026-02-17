use axum::{extract::{Path, Query, State}, routing::{get, post}, Json, Router};
use serde_json::Value;

use crate::{AppState, ApiError};

use super::{service, CreateLog, ListLogsQuery};

async fn ingest_log(
    State(state): State<AppState>,
    Path(project_id): Path<String>,
    Json(data): Json<CreateLog>,
) -> Result<Json<Value>, ApiError> {
    let log = service::ingest_log(&state, &project_id, data).await?;
    Ok(Json(serde_json::to_value(log).unwrap()))
}

async fn list_logs(
    State(state): State<AppState>,
    Path(project_id): Path<String>,
    Query(query): Query<ListLogsQuery>,
) -> Result<Json<Value>, ApiError> {
    let logs = service::list_logs(&state, &project_id, query).await?;
    Ok(Json(serde_json::to_value(logs).unwrap()))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/projects/:project_id/logs", post(ingest_log).get(list_logs))
}
