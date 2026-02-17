use crate::AppState;
use supabase::database::OrderDirection;

use super::Log;

pub async fn create(
    state: &AppState,
    project_id: &str,
    level: &str,
    message: &str,
    context: Option<&serde_json::Value>,
    trace_id: Option<&str>,
    source: Option<&str>,
    environment: Option<&str>,
) -> Result<Log, crate::ApiError> {
    let mut data = serde_json::json!({
        "project_id": project_id,
        "level": level,
        "message": message,
    });

    if let Some(ctx) = context {
        data["context"] = ctx.clone();
    }
    if let Some(tid) = trace_id {
        data["trace_id"] = serde_json::Value::String(tid.to_string());
    }
    if let Some(src) = source {
        data["source"] = serde_json::Value::String(src.to_string());
    }
    if let Some(env) = environment {
        data["environment"] = serde_json::Value::String(env.to_string());
    }

    let mut logs = state
        .supabase
        .database()
        .insert("logs")
        .values(data)
        .map_err(|e| {
            eprintln!("Serialization error: {:?}", e);
            crate::ApiError::InternalServerError
        })?
        .returning("*")
        .execute::<Log>()
        .await
        .map_err(|e| {
            eprintln!("Database error: {:?}", e);
            crate::ApiError::BadRequest(format!("Failed to create log: {:?}", e))
        })?;

    logs.pop().ok_or(crate::ApiError::InternalServerError)
}

pub async fn list(
    state: &AppState,
    project_id: &str,
    level: Option<&str>,
    trace_id: Option<&str>,
    source: Option<&str>,
    environment: Option<&str>,
    limit: Option<u32>,
    offset: Option<u32>,
) -> Result<Vec<Log>, crate::ApiError> {
    let mut query = state
        .supabase
        .database()
        .from("logs")
        .select("*")
        .eq("project_id", project_id);

    if let Some(level) = level {
        query = query.eq("level", level);
    }
    if let Some(trace_id) = trace_id {
        query = query.eq("trace_id", trace_id);
    }
    if let Some(source) = source {
        query = query.eq("source", source);
    }
    if let Some(environment) = environment {
        query = query.eq("environment", environment);
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

    let logs = query
        .execute::<Log>()
        .await
        .map_err(|e| {
            eprintln!("Database error: {:?}", e);
            crate::ApiError::InternalServerError
        })?;

    Ok(logs)
}
