use serde_json::json;

#[test]
fn test_monitor_deserialization() {
    let json = json!({
        "id": "123e4567-e89b-12d3-a456-426614174000",
        "project_id": "123e4567-e89b-12d3-a456-426614174001",
        "name": "Test Monitor",
        "kind": "http",
        "url": "https://example.com",
        "keyword": null,
        "interval_seconds": 300,
        "enabled": true,
        "created_at": "2026-02-17T10:00:00Z",
        "updated_at": "2026-02-17T10:00:00Z"
    });

    let monitor: statusforge_backend::features::monitors::Monitor = serde_json::from_value(json).unwrap();
    assert_eq!(monitor.name, "Test Monitor");
    assert_eq!(monitor.kind, "http");
    assert_eq!(monitor.url, "https://example.com");
    assert_eq!(monitor.interval_seconds, 300);
    assert!(monitor.enabled);
    assert!(monitor.keyword.is_none());
}

#[test]
fn test_monitor_with_keyword() {
    let json = json!({
        "id": "123e4567-e89b-12d3-a456-426614174000",
        "project_id": "123e4567-e89b-12d3-a456-426614174001",
        "name": "Keyword Monitor",
        "kind": "keyword",
        "url": "https://example.com",
        "keyword": "test keyword",
        "interval_seconds": 600,
        "enabled": true,
        "created_at": "2026-02-17T10:00:00Z",
        "updated_at": "2026-02-17T10:00:00Z"
    });

    let monitor: statusforge_backend::features::monitors::Monitor = serde_json::from_value(json).unwrap();
    assert_eq!(monitor.kind, "keyword");
    assert_eq!(monitor.keyword, Some("test keyword".to_string()));
}

#[test]
fn test_monitor_result_deserialization() {
    let json = json!({
        "id": "123e4567-e89b-12d3-a456-426614174000",
        "monitor_id": "123e4567-e89b-12d3-a456-426614174001",
        "region": "EU",
        "status": "up",
        "response_time_ms": 150,
        "http_status": 200,
        "ssl_valid": true,
        "ssl_expires_at": "2026-12-31T23:59:59Z",
        "error_message": null,
        "created_at": "2026-02-17T10:00:00Z"
    });

    let result: statusforge_backend::features::monitors::MonitorResult = serde_json::from_value(json).unwrap();
    assert_eq!(result.region, "EU");
    assert_eq!(result.status, "up");
    assert_eq!(result.response_time_ms, Some(150));
    assert_eq!(result.http_status, Some(200));
    assert_eq!(result.ssl_valid, Some(true));
    assert!(result.error_message.is_none());
}

#[test]
fn test_monitor_result_with_error() {
    let json = json!({
        "id": "123e4567-e89b-12d3-a456-426614174000",
        "monitor_id": "123e4567-e89b-12d3-a456-426614174001",
        "region": "US",
        "status": "down",
        "response_time_ms": null,
        "http_status": null,
        "ssl_valid": null,
        "ssl_expires_at": null,
        "error_message": "Connection timeout",
        "created_at": "2026-02-17T10:00:00Z"
    });

    let result: statusforge_backend::features::monitors::MonitorResult = serde_json::from_value(json).unwrap();
    assert_eq!(result.region, "US");
    assert_eq!(result.status, "down");
    assert_eq!(result.error_message, Some("Connection timeout".to_string()));
    assert!(result.response_time_ms.is_none());
}
