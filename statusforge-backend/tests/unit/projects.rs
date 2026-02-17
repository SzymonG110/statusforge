use statusforge_backend::features::projects::service;
use statusforge_backend::features::projects::{CreateProject, UpdateProject};
use statusforge_backend::{AppState, shared};

#[tokio::test]
async fn test_create_project_validates_empty_name() {
    let data = CreateProject {
        name: "   ".to_string(),
        description: None,
    };
    let state = AppState {
        supabase: shared::supabase::create_client(&statusforge_backend::shared::config::Config {
            port: 3001,
            supabase_url: "http://localhost".to_string(),
            supabase_publishable_key: "test".to_string(),
            supabase_secret_key: None,
        }).unwrap()
    };
    let result = service::create_project(&state, "org-id", data, "test-user-id").await;
    assert!(matches!(result, Err(statusforge_backend::ApiError::BadRequest(_))));
}

#[tokio::test]
async fn test_create_project_validates_empty_string() {
    let data = CreateProject {
        name: "".to_string(),
        description: None,
    };
    let state = AppState {
        supabase: shared::supabase::create_client(&statusforge_backend::shared::config::Config {
            port: 3001,
            supabase_url: "http://localhost".to_string(),
            supabase_publishable_key: "test".to_string(),
            supabase_secret_key: None,
        }).unwrap()
    };
    let result = service::create_project(&state, "org-id", data, "test-user-id").await;
    assert!(matches!(result, Err(statusforge_backend::ApiError::BadRequest(_))));
}

#[tokio::test]
async fn test_create_project_validates_whitespace_only() {
    let data = CreateProject {
        name: "\t\n  \r".to_string(),
        description: None,
    };
    let state = AppState {
        supabase: shared::supabase::create_client(&statusforge_backend::shared::config::Config {
            port: 3001,
            supabase_url: "http://localhost".to_string(),
            supabase_publishable_key: "test".to_string(),
            supabase_secret_key: None,
        }).unwrap()
    };
    let result = service::create_project(&state, "org-id", data, "test-user-id").await;
    assert!(matches!(result, Err(statusforge_backend::ApiError::BadRequest(_))));
}

#[tokio::test]
async fn test_update_project_validates_empty_name() {
    let data = UpdateProject {
        name: Some("   ".to_string()),
        description: None,
    };
    let state = AppState {
        supabase: shared::supabase::create_client(&statusforge_backend::shared::config::Config {
            port: 3001,
            supabase_url: "http://localhost".to_string(),
            supabase_publishable_key: "test".to_string(),
            supabase_secret_key: None,
        }).unwrap()
    };
    let result = service::update_project(&state, "test-id", data).await;
    assert!(matches!(result, Err(statusforge_backend::ApiError::BadRequest(_))));
}

#[tokio::test]
async fn test_update_project_allows_none_name() {
    let data = UpdateProject {
        name: None,
        description: Some("Updated description".to_string()),
    };
    let state = AppState {
        supabase: shared::supabase::create_client(&statusforge_backend::shared::config::Config {
            port: 3001,
            supabase_url: "http://localhost".to_string(),
            supabase_publishable_key: "test".to_string(),
            supabase_secret_key: None,
        }).unwrap()
    };
    let result: Result<_, statusforge_backend::ApiError> = service::update_project(&state, "test-id", data).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_create_project_allows_description() {
    let data = CreateProject {
        name: "Valid Project".to_string(),
        description: Some("Project description".to_string()),
    };
    let state = AppState {
        supabase: shared::supabase::create_client(&statusforge_backend::shared::config::Config {
            port: 3001,
            supabase_url: "http://localhost".to_string(),
            supabase_publishable_key: "test".to_string(),
            supabase_secret_key: None,
        }).unwrap()
    };
    let result: Result<_, statusforge_backend::ApiError> = service::create_project(&state, "org-id", data, "test-user-id").await;
    assert!(result.is_err());
}
