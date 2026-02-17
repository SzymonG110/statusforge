use statusforge_backend::ApiError;
use axum::{http::StatusCode, response::IntoResponse};

#[test]
fn test_not_found_error_status() {
    let error = ApiError::NotFound;
    let response = error.into_response();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[test]
fn test_bad_request_error_status() {
    let error = ApiError::BadRequest("Invalid input".to_string());
    let response = error.into_response();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[test]
fn test_internal_server_error_status() {
    let error = ApiError::InternalServerError;
    let response = error.into_response();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[test]
fn test_error_variants() {
    assert!(matches!(ApiError::NotFound, ApiError::NotFound));
    assert!(matches!(
        ApiError::BadRequest("test".to_string()),
        ApiError::BadRequest(_)
    ));
    assert!(matches!(ApiError::InternalServerError, ApiError::InternalServerError));
}
