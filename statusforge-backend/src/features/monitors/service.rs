use crate::AppState;

use super::Monitor;

pub async fn list_by_project(_state: &AppState, _project_id: &str) -> Result<Vec<Monitor>, crate::ApiError> {
    Ok(Vec::new())
}
