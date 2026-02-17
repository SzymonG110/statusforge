use crate::AppState;

pub async fn validate_jwt(_state: &AppState, _token: &str) -> Result<String, crate::ApiError> {
    Err(crate::ApiError::NotFound)
}
