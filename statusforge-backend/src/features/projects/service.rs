use crate::AppState;

use super::{Project, repository};

pub async fn list_projects(state: &AppState, organization_id: &str) -> Result<Vec<Project>, crate::ApiError> {
    repository::list_by_org(state, organization_id).await
}
