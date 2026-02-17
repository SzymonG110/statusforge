use statusforge_backend::features::monitors::service;

#[tokio::test]
async fn test_create_monitor_validation() {
    let state = statusforge_backend::AppState {
        supabase: statusforge_backend::shared::supabase::create_client().unwrap(),
    };

    let invalid_kind = statusforge_backend::features::monitors::CreateMonitor {
        name: "Test Monitor".to_string(),
        kind: "invalid".to_string(),
        url: "https://example.com".to_string(),
        keyword: None,
        interval_seconds: Some(300),
        enabled: Some(true),
    };

    let result: Result<_, statusforge_backend::ApiError> = service::create_monitor(&state, "test-project-id", invalid_kind).await;
    assert!(result.is_err());
    if let Err(statusforge_backend::ApiError::BadRequest(msg)) = result {
        assert!(msg.contains("Invalid kind"));
    } else {
        panic!("Expected BadRequest error");
    }
}

#[tokio::test]
async fn test_create_monitor_empty_name() {
    let state = statusforge_backend::AppState {
        supabase: statusforge_backend::shared::supabase::create_client().unwrap(),
    };

    let empty_name = statusforge_backend::features::monitors::CreateMonitor {
        name: "   ".to_string(),
        kind: "http".to_string(),
        url: "https://example.com".to_string(),
        keyword: None,
        interval_seconds: Some(300),
        enabled: Some(true),
    };

    let result: Result<_, statusforge_backend::ApiError> = service::create_monitor(&state, "test-project-id", empty_name).await;
    assert!(result.is_err());
    if let Err(statusforge_backend::ApiError::BadRequest(msg)) = result {
        assert!(msg.contains("name cannot be empty"));
    } else {
        panic!("Expected BadRequest error");
    }
}

#[tokio::test]
async fn test_create_monitor_empty_url() {
    let state = statusforge_backend::AppState {
        supabase: statusforge_backend::shared::supabase::create_client().unwrap(),
    };

    let empty_url = statusforge_backend::features::monitors::CreateMonitor {
        name: "Test Monitor".to_string(),
        kind: "http".to_string(),
        url: "   ".to_string(),
        keyword: None,
        interval_seconds: Some(300),
        enabled: Some(true),
    };

    let result: Result<_, statusforge_backend::ApiError> = service::create_monitor(&state, "test-project-id", empty_url).await;
    assert!(result.is_err());
    if let Err(statusforge_backend::ApiError::BadRequest(msg)) = result {
        assert!(msg.contains("URL cannot be empty"));
    } else {
        panic!("Expected BadRequest error");
    }
}

#[tokio::test]
async fn test_create_monitor_keyword_required() {
    let state = statusforge_backend::AppState {
        supabase: statusforge_backend::shared::supabase::create_client().unwrap(),
    };

    let keyword_monitor = statusforge_backend::features::monitors::CreateMonitor {
        name: "Test Monitor".to_string(),
        kind: "keyword".to_string(),
        url: "https://example.com".to_string(),
        keyword: None,
        interval_seconds: Some(300),
        enabled: Some(true),
    };

    let result: Result<_, statusforge_backend::ApiError> = service::create_monitor(&state, "test-project-id", keyword_monitor).await;
    assert!(result.is_err());
    if let Err(statusforge_backend::ApiError::BadRequest(msg)) = result {
        assert!(msg.contains("Keyword is required"));
    } else {
        panic!("Expected BadRequest error");
    }
}

#[tokio::test]
async fn test_create_monitor_interval_too_low() {
    let state = statusforge_backend::AppState {
        supabase: statusforge_backend::shared::supabase::create_client().unwrap(),
    };

    let low_interval = statusforge_backend::features::monitors::CreateMonitor {
        name: "Test Monitor".to_string(),
        kind: "http".to_string(),
        url: "https://example.com".to_string(),
        keyword: None,
        interval_seconds: Some(30),
        enabled: Some(true),
    };

    let result: Result<_, statusforge_backend::ApiError> = service::create_monitor(&state, "test-project-id", low_interval).await;
    assert!(result.is_err());
    if let Err(statusforge_backend::ApiError::BadRequest(msg)) = result {
        assert!(msg.contains("at least 60 seconds"));
    } else {
        panic!("Expected BadRequest error");
    }
}

#[tokio::test]
async fn test_update_monitor_validation() {
    let state = statusforge_backend::AppState {
        supabase: statusforge_backend::shared::supabase::create_client().unwrap(),
    };

    let invalid_kind = statusforge_backend::features::monitors::UpdateMonitor {
        name: None,
        kind: Some("invalid".to_string()),
        url: None,
        keyword: None,
        interval_seconds: None,
        enabled: None,
    };

    let result: Result<_, statusforge_backend::ApiError> = service::update_monitor(&state, "test-id", invalid_kind).await;
    assert!(result.is_err());
    if let Err(statusforge_backend::ApiError::BadRequest(msg)) = result {
        assert!(msg.contains("Invalid kind"));
    } else {
        panic!("Expected BadRequest error");
    }
}

#[tokio::test]
async fn test_create_monitor_result_validation() {
    let state = statusforge_backend::AppState {
        supabase: statusforge_backend::shared::supabase::create_client().unwrap(),
    };

    let invalid_region = statusforge_backend::features::monitors::CreateMonitorResult {
        region: "INVALID".to_string(),
        status: "up".to_string(),
        response_time_ms: None,
        http_status: None,
        ssl_valid: None,
        ssl_expires_at: None,
        error_message: None,
    };

    let result: Result<_, statusforge_backend::ApiError> = service::create_monitor_result(&state, "test-monitor-id", invalid_region).await;
    assert!(result.is_err());
    if let Err(statusforge_backend::ApiError::BadRequest(msg)) = result {
        assert!(msg.contains("Invalid region"));
    } else {
        panic!("Expected BadRequest error");
    }

    let invalid_status = statusforge_backend::features::monitors::CreateMonitorResult {
        region: "EU".to_string(),
        status: "invalid".to_string(),
        response_time_ms: None,
        http_status: None,
        ssl_valid: None,
        ssl_expires_at: None,
        error_message: None,
    };

    let result: Result<_, statusforge_backend::ApiError> = service::create_monitor_result(&state, "test-monitor-id", invalid_status).await;
    assert!(result.is_err());
    if let Err(statusforge_backend::ApiError::BadRequest(msg)) = result {
        assert!(msg.contains("Invalid status"));
    } else {
        panic!("Expected BadRequest error");
    }
}
