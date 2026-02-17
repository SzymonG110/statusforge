use crate::AppState;

use super::{Organization, repository, CreateOrganization, UpdateOrganization};

pub async fn list_organizations(state: &AppState, user_id: &str) -> Result<Vec<Organization>, crate::ApiError> {
    repository::list(state, user_id).await
}

pub async fn get_organization(state: &AppState, id: &str) -> Result<Organization, crate::ApiError> {
    repository::get_by_id(state, id).await
}

pub async fn create_organization(
    state: &AppState,
    data: CreateOrganization,
    user_id: &str,
) -> Result<Organization, crate::ApiError> {
    if data.name.trim().is_empty() {
        return Err(crate::ApiError::BadRequest("Organization name cannot be empty".to_string()));
    }
    repository::create(state, &data.name, user_id).await
}

pub async fn update_organization(
    state: &AppState,
    id: &str,
    data: UpdateOrganization,
) -> Result<Organization, crate::ApiError> {
    if let Some(ref name) = data.name {
        if name.trim().is_empty() {
            return Err(crate::ApiError::BadRequest("Organization name cannot be empty".to_string()));
        }
    }
    repository::update(state, id, data.name.as_deref()).await
}

pub async fn delete_organization(state: &AppState, id: &str) -> Result<(), crate::ApiError> {
    repository::delete(state, id).await
}
