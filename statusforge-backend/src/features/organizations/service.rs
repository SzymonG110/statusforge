use crate::AppState;

use super::{Organization, repository};

pub async fn list_organizations(state: &AppState, user_id: &str) -> Result<Vec<Organization>, crate::ApiError> {
    repository::list(state, user_id).await
}
