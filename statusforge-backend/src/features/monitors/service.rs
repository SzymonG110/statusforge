use crate::AppState;

use super::{Monitor, MonitorResult, repository, CreateMonitor, UpdateMonitor, CreateMonitorResult, ListMonitorResultsQuery};

pub async fn list_monitors(state: &AppState, project_id: &str) -> Result<Vec<Monitor>, crate::ApiError> {
    repository::list_by_project(state, project_id).await
}

pub async fn get_monitor(state: &AppState, id: &str) -> Result<Monitor, crate::ApiError> {
    repository::get_by_id(state, id).await
}

pub async fn create_monitor(
    state: &AppState,
    project_id: &str,
    data: CreateMonitor,
) -> Result<Monitor, crate::ApiError> {
    let valid_kinds = ["http", "https", "ssl", "keyword"];
    if !valid_kinds.contains(&data.kind.as_str()) {
        return Err(crate::ApiError::BadRequest(
            format!("Invalid kind: {}. Must be one of: {:?}", data.kind, valid_kinds)
        ));
    }

    if data.name.trim().is_empty() {
        return Err(crate::ApiError::BadRequest("Monitor name cannot be empty".to_string()));
    }

    if data.url.trim().is_empty() {
        return Err(crate::ApiError::BadRequest("Monitor URL cannot be empty".to_string()));
    }

    if data.kind == "keyword" && data.keyword.is_none() {
        return Err(crate::ApiError::BadRequest("Keyword is required for keyword monitoring".to_string()));
    }

    let interval = data.interval_seconds.unwrap_or(300);
    if interval < 60 {
        return Err(crate::ApiError::BadRequest("Interval must be at least 60 seconds".to_string()));
    }

    repository::create(
        state,
        project_id,
        &data.name,
        &data.kind,
        &data.url,
        data.keyword.as_deref(),
        interval,
        data.enabled.unwrap_or(true),
    ).await
}

pub async fn update_monitor(
    state: &AppState,
    id: &str,
    data: UpdateMonitor,
) -> Result<Monitor, crate::ApiError> {
    if let Some(ref name) = data.name {
        if name.trim().is_empty() {
            return Err(crate::ApiError::BadRequest("Monitor name cannot be empty".to_string()));
        }
    }

    if let Some(ref url) = data.url {
        if url.trim().is_empty() {
            return Err(crate::ApiError::BadRequest("Monitor URL cannot be empty".to_string()));
        }
    }

    if let Some(ref kind) = data.kind {
        let valid_kinds = ["http", "https", "ssl", "keyword"];
        if !valid_kinds.contains(kind.as_str()) {
            return Err(crate::ApiError::BadRequest(
                format!("Invalid kind: {}. Must be one of: {:?}", kind, valid_kinds)
            ));
        }
    }

    if let Some(interval) = data.interval_seconds {
        if interval < 60 {
            return Err(crate::ApiError::BadRequest("Interval must be at least 60 seconds".to_string()));
        }
    }

    repository::update(
        state,
        id,
        data.name.as_deref(),
        data.kind.as_deref(),
        data.url.as_deref(),
        data.keyword.as_deref(),
        data.interval_seconds,
        data.enabled,
    ).await
}

pub async fn delete_monitor(state: &AppState, id: &str) -> Result<(), crate::ApiError> {
    repository::delete(state, id).await
}

pub async fn create_monitor_result(
    state: &AppState,
    monitor_id: &str,
    data: CreateMonitorResult,
) -> Result<MonitorResult, crate::ApiError> {
    let valid_regions = ["EU", "US", "ASIA"];
    if !valid_regions.contains(&data.region.as_str()) {
        return Err(crate::ApiError::BadRequest(
            format!("Invalid region: {}. Must be one of: {:?}", data.region, valid_regions)
        ));
    }

    let valid_statuses = ["up", "down", "degraded"];
    if !valid_statuses.contains(&data.status.as_str()) {
        return Err(crate::ApiError::BadRequest(
            format!("Invalid status: {}. Must be one of: {:?}", data.status, valid_statuses)
        ));
    }

    repository::create_result(
        state,
        monitor_id,
        &data.region,
        &data.status,
        data.response_time_ms,
        data.http_status,
        data.ssl_valid,
        data.ssl_expires_at.as_deref(),
        data.error_message.as_deref(),
    ).await
}

pub async fn list_monitor_results(
    state: &AppState,
    monitor_id: &str,
    query: ListMonitorResultsQuery,
) -> Result<Vec<MonitorResult>, crate::ApiError> {
    repository::list_results(
        state,
        monitor_id,
        query.region.as_deref(),
        query.status.as_deref(),
        query.limit,
        query.offset,
    ).await
}

pub async fn run_monitor_check(
    state: &AppState,
    monitor_id: &str,
    region: Option<&str>,
) -> Result<MonitorResult, crate::ApiError> {
    let monitor = repository::get_by_id(state, monitor_id).await?;

    if !monitor.enabled {
        return Err(crate::ApiError::BadRequest("Monitor is disabled".to_string()));
    }

    let check_region = region.unwrap_or("EU");
    let valid_regions = ["EU", "US", "ASIA"];
    if !valid_regions.contains(&check_region) {
        return Err(crate::ApiError::BadRequest(
            format!("Invalid region: {}. Must be one of: {:?}", check_region, valid_regions)
        ));
    }

    let payload = serde_json::json!({
        "monitor_id": monitor.id,
        "project_id": monitor.project_id,
        "name": monitor.name,
        "kind": monitor.kind,
        "url": monitor.url,
        "keyword": monitor.keyword,
        "region": check_region,
    });

    let function_name = "monitor-check";
    let response = state
        .supabase
        .functions()
        .invoke(function_name, Some(payload))
        .await
        .map_err(|e| {
            eprintln!("Edge Function error: {:?}", e);
            crate::ApiError::InternalServerError
        })?;

    let result_data: CreateMonitorResult = serde_json::from_value(response)
        .map_err(|e| {
            eprintln!("Failed to parse Edge Function response: {:?}", e);
            crate::ApiError::InternalServerError
        })?;

    create_monitor_result(state, monitor_id, result_data).await
}
