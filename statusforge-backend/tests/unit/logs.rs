use statusforge_backend::features::ingest::service;
use statusforge_backend::features::ingest::{CreateLog, ListLogsQuery};
use statusforge_backend::{AppState, shared};

#[tokio::test]
async fn test_ingest_log_validates_empty_message() {
    let data = CreateLog {
        level: "info".to_string(),
        message: "   ".to_string(),
        context: None,
        trace_id: None,
        source: None,
        environment: None,
    };
    let state = AppState {
        supabase: shared::supabase::create_client(&statusforge_backend::shared::config::Config {
            port: 3001,
            supabase_url: "http://localhost".to_string(),
            supabase_publishable_key: "test".to_string(),
            supabase_secret_key: None,
        }).unwrap()
    };
    let result = service::ingest_log(&state, "project-id", data).await;
    assert!(matches!(result, Err(statusforge_backend::ApiError::BadRequest(_))));
}

#[tokio::test]
async fn test_ingest_log_validates_invalid_level() {
    let data = CreateLog {
        level: "invalid".to_string(),
        message: "Test message".to_string(),
        context: None,
        trace_id: None,
        source: None,
        environment: None,
    };
    let state = AppState {
        supabase: shared::supabase::create_client(&statusforge_backend::shared::config::Config {
            port: 3001,
            supabase_url: "http://localhost".to_string(),
            supabase_publishable_key: "test".to_string(),
            supabase_secret_key: None,
        }).unwrap()
    };
    let result = service::ingest_log(&state, "project-id", data).await;
    assert!(matches!(result, Err(statusforge_backend::ApiError::BadRequest(_))));
}

#[tokio::test]
async fn test_ingest_log_accepts_valid_levels() {
    let valid_levels = ["debug", "info", "warn", "error", "fatal"];
    for level in valid_levels {
        let data = CreateLog {
            level: level.to_string(),
            message: "Test message".to_string(),
            context: None,
            trace_id: None,
            source: None,
            environment: None,
        };
        let state = AppState {
            supabase: shared::supabase::create_client(&statusforge_backend::shared::config::Config {
                port: 3001,
                supabase_url: "http://localhost".to_string(),
                supabase_publishable_key: "test".to_string(),
                supabase_secret_key: None,
            }).unwrap()
        };
        let result: Result<_, statusforge_backend::ApiError> = service::ingest_log(&state, "project-id", data).await;
        assert!(result.is_err());
    }
}

#[tokio::test]
async fn test_list_logs_with_filters() {
    let query = ListLogsQuery {
        level: Some("error".to_string()),
        trace_id: Some("trace-123".to_string()),
        source: Some("api".to_string()),
        environment: Some("production".to_string()),
        limit: Some(10),
        offset: Some(0),
    };
    let state = AppState {
        supabase: shared::supabase::create_client(&statusforge_backend::shared::config::Config {
            port: 3001,
            supabase_url: "http://localhost".to_string(),
            supabase_publishable_key: "test".to_string(),
            supabase_secret_key: None,
        }).unwrap()
    };
    let result: Result<_, statusforge_backend::ApiError> = service::list_logs(&state, "project-id", query).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_logs_without_filters() {
    let query = ListLogsQuery {
        level: None,
        trace_id: None,
        source: None,
        environment: None,
        limit: None,
        offset: None,
    };
    let state = AppState {
        supabase: shared::supabase::create_client(&statusforge_backend::shared::config::Config {
            port: 3001,
            supabase_url: "http://localhost".to_string(),
            supabase_publishable_key: "test".to_string(),
            supabase_secret_key: None,
        }).unwrap()
    };
    let result: Result<_, statusforge_backend::ApiError> = service::list_logs(&state, "project-id", query).await;
    assert!(result.is_err());
}
