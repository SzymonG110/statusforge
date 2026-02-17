use crate::AppState;
use serde_json::Value;

pub async fn ingest_log(_state: &AppState, _project_id: &str, _body: Value) -> Result<(), crate::ApiError> {
    Ok(())
}
