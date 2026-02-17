use statusforge_backend::features::organizations::service;
use statusforge_backend::features::organizations::{CreateOrganization, UpdateOrganization};
use statusforge_backend::{AppState, shared};

#[tokio::test]
async fn test_create_organization_validates_empty_name() {
    let data = CreateOrganization {
        name: "   ".to_string(),
    };
    let state = AppState {
        supabase: shared::supabase::create_client(&statusforge_backend::shared::config::Config {
            port: 3001,
            supabase_url: "http://localhost".to_string(),
            supabase_publishable_key: "test".to_string(),
            supabase_secret_key: None,
        }).unwrap()
    };
    let result = service::create_organization(&state, data, "test-user-id").await;
    assert!(matches!(result, Err(statusforge_backend::ApiError::BadRequest(_))));
}

#[tokio::test]
async fn test_create_organization_validates_empty_string() {
    let data = CreateOrganization {
        name: "".to_string(),
    };
    let state = AppState {
        supabase: shared::supabase::create_client(&statusforge_backend::shared::config::Config {
            port: 3001,
            supabase_url: "http://localhost".to_string(),
            supabase_publishable_key: "test".to_string(),
            supabase_secret_key: None,
        }).unwrap()
    };
    let result = service::create_organization(&state, data, "test-user-id").await;
    assert!(matches!(result, Err(statusforge_backend::ApiError::BadRequest(_))));
}

#[tokio::test]
async fn test_create_organization_validates_whitespace_only() {
    let data = CreateOrganization {
        name: "\t\n  \r".to_string(),
    };
    let state = AppState {
        supabase: shared::supabase::create_client(&statusforge_backend::shared::config::Config {
            port: 3001,
            supabase_url: "http://localhost".to_string(),
            supabase_publishable_key: "test".to_string(),
            supabase_secret_key: None,
        }).unwrap()
    };
    let result = service::create_organization(&state, data, "test-user-id").await;
    assert!(matches!(result, Err(statusforge_backend::ApiError::BadRequest(_))));
}

#[tokio::test]
async fn test_update_organization_validates_empty_name() {
    let data = UpdateOrganization {
        name: Some("   ".to_string()),
    };
    let state = AppState {
        supabase: shared::supabase::create_client(&statusforge_backend::shared::config::Config {
            port: 3001,
            supabase_url: "http://localhost".to_string(),
            supabase_publishable_key: "test".to_string(),
            supabase_secret_key: None,
        }).unwrap()
    };
    let result = service::update_organization(&state, "test-id", data).await;
    assert!(matches!(result, Err(statusforge_backend::ApiError::BadRequest(_))));
}

#[tokio::test]
async fn test_update_organization_allows_none() {
    let data = UpdateOrganization { name: None };
    let state = AppState {
        supabase: shared::supabase::create_client(&statusforge_backend::shared::config::Config {
            port: 3001,
            supabase_url: "http://localhost".to_string(),
            supabase_publishable_key: "test".to_string(),
            supabase_secret_key: None,
        }).unwrap()
    };
    let result: Result<_, statusforge_backend::ApiError> = service::update_organization(&state, "test-id", data).await;
    assert!(result.is_err());
}
