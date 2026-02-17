use serde_json;
use statusforge_backend::features::ingest::{CreateLog, Log};

#[test]
fn test_create_log_deserializes_minimal() {
    let json = r#"{"level": "info", "message": "Test message"}"#;
    let result: Result<CreateLog, _> = serde_json::from_str(json);
    assert!(result.is_ok());
    let log = result.unwrap();
    assert_eq!(log.level, "info");
    assert_eq!(log.message, "Test message");
    assert_eq!(log.context, None);
    assert_eq!(log.trace_id, None);
}

#[test]
fn test_create_log_deserializes_full() {
    let json = r#"{
        "level": "error",
        "message": "Error occurred",
        "context": {"key": "value"},
        "trace_id": "trace-123",
        "source": "api",
        "environment": "production"
    }"#;
    let result: Result<CreateLog, _> = serde_json::from_str(json);
    assert!(result.is_ok());
    let log = result.unwrap();
    assert_eq!(log.level, "error");
    assert_eq!(log.message, "Error occurred");
    assert!(log.context.is_some());
    assert_eq!(log.trace_id, Some("trace-123".to_string()));
    assert_eq!(log.source, Some("api".to_string()));
    assert_eq!(log.environment, Some("production".to_string()));
}

#[test]
fn test_create_log_requires_level_and_message() {
    let json = r#"{}"#;
    let result: Result<CreateLog, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

#[test]
fn test_log_serializes_all_fields() {
    let log = Log {
        id: "123e4567-e89b-12d3-a456-426614174000".to_string(),
        project_id: "project-123".to_string(),
        level: "error".to_string(),
        message: "Error message".to_string(),
        context: Some(serde_json::json!({"key": "value"})),
        trace_id: Some("trace-123".to_string()),
        source: Some("api".to_string()),
        environment: Some("production".to_string()),
        created_at: "2024-01-01T00:00:00Z".to_string(),
    };
    let json = serde_json::to_string(&log).unwrap();
    assert!(json.contains("\"id\""));
    assert!(json.contains("\"project_id\""));
    assert!(json.contains("\"level\""));
    assert!(json.contains("\"message\""));
    assert!(json.contains("\"context\""));
    assert!(json.contains("\"trace_id\""));
    assert!(json.contains("\"source\""));
    assert!(json.contains("\"environment\""));
    assert!(json.contains("\"created_at\""));
}

#[test]
fn test_log_deserializes_from_json() {
    let json = r#"{
        "id": "123e4567-e89b-12d3-a456-426614174000",
        "project_id": "project-123",
        "level": "error",
        "message": "Error message",
        "context": {"key": "value"},
        "trace_id": "trace-123",
        "source": "api",
        "environment": "production",
        "created_at": "2024-01-01T00:00:00Z"
    }"#;
    let result: Result<Log, _> = serde_json::from_str(json);
    assert!(result.is_ok());
    let log = result.unwrap();
    assert_eq!(log.id, "123e4567-e89b-12d3-a456-426614174000");
    assert_eq!(log.project_id, "project-123");
    assert_eq!(log.level, "error");
    assert_eq!(log.message, "Error message");
}
