use crate::AppState;

use super::Project;

pub async fn list_by_org(_state: &AppState, _organization_id: &str) -> Result<Vec<Project>, crate::ApiError> {
    Ok(Vec::new())
}
