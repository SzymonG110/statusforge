use crate::AppState;

use super::Organization;

pub async fn list(_state: &AppState, _user_id: &str) -> Result<Vec<Organization>, crate::ApiError> {
    Ok(Vec::new())
}
