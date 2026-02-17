use crate::AppState;
use supabase::database::OrderDirection;

use super::{Monitor, MonitorResult};

pub async fn list_by_project(state: &AppState, project_id: &str) -> Result<Vec<Monitor>, crate::ApiError> {
    let monitors = state
        .supabase
        .database()
        .from("monitors")
        .select("*")
        .eq("project_id", project_id)
        .execute::<Monitor>()
        .await
        .map_err(|e| {
            eprintln!("Database error: {:?}", e);
            crate::ApiError::InternalServerError
        })?;

    Ok(monitors)
}

pub async fn get_by_id(state: &AppState, id: &str) -> Result<Monitor, crate::ApiError> {
    let mut monitors = state
        .supabase
        .database()
        .from("monitors")
        .select("*")
        .eq("id", id)
        .execute::<Monitor>()
        .await
        .map_err(|e| {
            eprintln!("Database error: {:?}", e);
            crate::ApiError::NotFound
        })?;

    monitors.pop().ok_or(crate::ApiError::NotFound)
}

pub async fn create(
    state: &AppState,
    project_id: &str,
    name: &str,
    kind: &str,
    url: &str,
    keyword: Option<&str>,
    interval_seconds: i32,
    enabled: bool,
) -> Result<Monitor, crate::ApiError> {
    let mut data = serde_json::json!({
        "project_id": project_id,
        "name": name,
        "kind": kind,
        "url": url,
        "interval_seconds": interval_seconds,
        "enabled": enabled,
    });

    if let Some(kw) = keyword {
        data["keyword"] = serde_json::Value::String(kw.to_string());
    }

    let mut monitors = state
        .supabase
        .database()
        .insert("monitors")
        .values(data)
        .map_err(|e| {
            eprintln!("Serialization error: {:?}", e);
            crate::ApiError::InternalServerError
        })?
        .returning("*")
        .execute::<Monitor>()
        .await
        .map_err(|e| {
            eprintln!("Database error: {:?}", e);
            crate::ApiError::BadRequest(format!("Failed to create monitor: {:?}", e))
        })?;

    monitors.pop().ok_or(crate::ApiError::InternalServerError)
}

pub async fn update(
    state: &AppState,
    id: &str,
    name: Option<&str>,
    kind: Option<&str>,
    url: Option<&str>,
    keyword: Option<&str>,
    interval_seconds: Option<i32>,
    enabled: Option<bool>,
) -> Result<Monitor, crate::ApiError> {
    let mut data = serde_json::Map::new();
    if let Some(name) = name {
        data.insert("name".to_string(), serde_json::Value::String(name.to_string()));
    }
    if let Some(kind) = kind {
        data.insert("kind".to_string(), serde_json::Value::String(kind.to_string()));
    }
    if let Some(url) = url {
        data.insert("url".to_string(), serde_json::Value::String(url.to_string()));
    }
    if let Some(kw) = keyword {
        data.insert("keyword".to_string(), serde_json::Value::String(kw.to_string()));
    } else if keyword.is_some() {
        data.insert("keyword".to_string(), serde_json::Value::Null);
    }
    if let Some(interval) = interval_seconds {
        data.insert("interval_seconds".to_string(), serde_json::Value::Number(interval.into()));
    }
    if let Some(enabled) = enabled {
        data.insert("enabled".to_string(), serde_json::Value::Bool(enabled));
    }

    let mut monitors = state
        .supabase
        .database()
        .update("monitors")
        .set(serde_json::Value::Object(data))
        .map_err(|e| {
            eprintln!("Serialization error: {:?}", e);
            crate::ApiError::InternalServerError
        })?
        .eq("id", id)
        .returning("*")
        .execute::<Monitor>()
        .await
        .map_err(|e| {
            eprintln!("Database error: {:?}", e);
            crate::ApiError::NotFound
        })?;

    monitors.pop().ok_or(crate::ApiError::NotFound)
}

pub async fn delete(state: &AppState, id: &str) -> Result<(), crate::ApiError> {
    state
        .supabase
        .database()
        .delete("monitors")
        .eq("id", id)
        .execute::<serde_json::Value>()
        .await
        .map_err(|e| {
            eprintln!("Database error: {:?}", e);
            crate::ApiError::NotFound
        })?;

    Ok(())
}

pub async fn create_result(
    state: &AppState,
    monitor_id: &str,
    region: &str,
    status: &str,
    response_time_ms: Option<i32>,
    http_status: Option<i32>,
    ssl_valid: Option<bool>,
    ssl_expires_at: Option<&str>,
    error_message: Option<&str>,
) -> Result<MonitorResult, crate::ApiError> {
    let mut data = serde_json::json!({
        "monitor_id": monitor_id,
        "region": region,
        "status": status,
    });

    if let Some(rt) = response_time_ms {
        data["response_time_ms"] = serde_json::Value::Number(rt.into());
    }
    if let Some(hs) = http_status {
        data["http_status"] = serde_json::Value::Number(hs.into());
    }
    if let Some(sv) = ssl_valid {
        data["ssl_valid"] = serde_json::Value::Bool(sv);
    }
    if let Some(se) = ssl_expires_at {
        data["ssl_expires_at"] = serde_json::Value::String(se.to_string());
    }
    if let Some(em) = error_message {
        data["error_message"] = serde_json::Value::String(em.to_string());
    }

    let mut results = state
        .supabase
        .database()
        .insert("monitor_results")
        .values(data)
        .map_err(|e| {
            eprintln!("Serialization error: {:?}", e);
            crate::ApiError::InternalServerError
        })?
        .returning("*")
        .execute::<MonitorResult>()
        .await
        .map_err(|e| {
            eprintln!("Database error: {:?}", e);
            crate::ApiError::BadRequest(format!("Failed to create monitor result: {:?}", e))
        })?;

    results.pop().ok_or(crate::ApiError::InternalServerError)
}

pub async fn list_results(
    state: &AppState,
    monitor_id: &str,
    region: Option<&str>,
    status: Option<&str>,
    limit: Option<u32>,
    offset: Option<u32>,
) -> Result<Vec<MonitorResult>, crate::ApiError> {
    let mut query = state
        .supabase
        .database()
        .from("monitor_results")
        .select("*")
        .eq("monitor_id", monitor_id);

    if let Some(region) = region {
        query = query.eq("region", region);
    }
    if let Some(status) = status {
        query = query.eq("status", status);
    }

    query = query.order("created_at", OrderDirection::Descending);

    if let Some(limit) = limit {
        query = query.limit(limit.min(100));
    } else {
        query = query.limit(50);
    }

    if let Some(offset) = offset {
        query = query.offset(offset);
    }

    let results = query
        .execute::<MonitorResult>()
        .await
        .map_err(|e| {
            eprintln!("Database error: {:?}", e);
            crate::ApiError::InternalServerError
        })?;

    Ok(results)
}
