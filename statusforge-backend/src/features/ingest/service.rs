use crate::AppState;

use super::{Log, repository, CreateLog, ListLogsQuery};

pub async fn ingest_log(
    state: &AppState,
    project_id: &str,
    data: CreateLog,
) -> Result<Log, crate::ApiError> {
    let valid_levels = ["debug", "info", "warn", "error", "fatal"];
    if !valid_levels.contains(&data.level.as_str()) {
        return Err(crate::ApiError::BadRequest(
            format!("Invalid level: {}. Must be one of: {:?}", data.level, valid_levels)
        ));
    }

    if data.message.trim().is_empty() {
        return Err(crate::ApiError::BadRequest("Message cannot be empty".to_string()));
    }

    repository::create(
        state,
        project_id,
        &data.level,
        &data.message,
        data.context.as_ref(),
        data.trace_id.as_deref(),
        data.source.as_deref(),
        data.environment.as_deref(),
    ).await
}

pub async fn list_logs(
    state: &AppState,
    project_id: &str,
    query: ListLogsQuery,
) -> Result<Vec<Log>, crate::ApiError> {
    repository::list(
        state,
        project_id,
        query.level.as_deref(),
        query.trace_id.as_deref(),
        query.source.as_deref(),
        query.environment.as_deref(),
        query.limit,
        query.offset,
    ).await
}
