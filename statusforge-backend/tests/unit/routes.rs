use axum::extract::Query;
use serde::Deserialize;
use statusforge_backend::ApiError;

#[derive(Deserialize)]
struct UserIdQuery {
    user_id: Option<String>,
}

async fn get_user_id(query: Option<Query<UserIdQuery>>) -> Result<String, ApiError> {
    if let Some(Query(UserIdQuery { user_id: Some(id) })) = query {
        Ok(id)
    } else {
        Err(ApiError::BadRequest("user_id query parameter required (temporary - will use JWT later)".to_string()))
    }
}

#[tokio::test]
async fn test_get_user_id_with_valid_query() {
    let query = Query(UserIdQuery {
        user_id: Some("test-user-123".to_string()),
    });
    let result = get_user_id(Some(query)).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "test-user-123");
}

#[tokio::test]
async fn test_get_user_id_without_query() {
    let result = get_user_id(None).await;
    assert!(matches!(result, Err(ApiError::BadRequest(_))));
}

#[tokio::test]
async fn test_get_user_id_with_none_user_id() {
    let query = Query(UserIdQuery { user_id: None });
    let result = get_user_id(Some(query)).await;
    assert!(matches!(result, Err(ApiError::BadRequest(_))));
}

#[tokio::test]
async fn test_get_user_id_error_message() {
    let result = get_user_id(None).await;
    if let Err(ApiError::BadRequest(msg)) = result {
        assert!(msg.contains("user_id"));
    } else {
        panic!("Expected BadRequest error");
    }
}
