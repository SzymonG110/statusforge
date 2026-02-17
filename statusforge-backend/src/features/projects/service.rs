use crate::AppState;

use super::{Project, repository, CreateProject, UpdateProject};

pub async fn list_projects(state: &AppState, organization_id: &str) -> Result<Vec<Project>, crate::ApiError> {
    repository::list_by_org(state, organization_id).await
}

pub async fn get_project(state: &AppState, id: &str) -> Result<Project, crate::ApiError> {
    repository::get_by_id(state, id).await
}

pub async fn create_project(
    state: &AppState,
    organization_id: &str,
    data: CreateProject,
    user_id: &str,
) -> Result<Project, crate::ApiError> {
    if data.name.trim().is_empty() {
        return Err(crate::ApiError::BadRequest("Project name cannot be empty".to_string()));
    }
    repository::create(state, organization_id, &data.name, data.description.as_deref(), user_id).await
}

pub async fn update_project(
    state: &AppState,
    id: &str,
    data: UpdateProject,
) -> Result<Project, crate::ApiError> {
    if let Some(ref name) = data.name {
        if name.trim().is_empty() {
            return Err(crate::ApiError::BadRequest("Project name cannot be empty".to_string()));
        }
    }
    repository::update(state, id, data.name.as_deref(), data.description.as_deref()).await
}

pub async fn delete_project(state: &AppState, id: &str) -> Result<(), crate::ApiError> {
    repository::delete(state, id).await
}
