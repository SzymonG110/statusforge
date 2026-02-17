use serde_json;
use statusforge_backend::features::organizations::{CreateOrganization, UpdateOrganization, Organization};

#[test]
fn test_create_organization_deserializes_valid_json() {
    let json = r#"{"name": "My Organization"}"#;
    let result: Result<CreateOrganization, _> = serde_json::from_str(json);
    assert!(result.is_ok());
    let org = result.unwrap();
    assert_eq!(org.name, "My Organization");
}

#[test]
fn test_create_organization_requires_name() {
    let json = r#"{}"#;
    let result: Result<CreateOrganization, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

#[test]
fn test_update_organization_deserializes_with_name() {
    let json = r#"{"name": "Updated Name"}"#;
    let result: Result<UpdateOrganization, _> = serde_json::from_str(json);
    assert!(result.is_ok());
    let update = result.unwrap();
    assert_eq!(update.name, Some("Updated Name".to_string()));
}

#[test]
fn test_update_organization_deserializes_without_name() {
    let json = r#"{}"#;
    let result: Result<UpdateOrganization, _> = serde_json::from_str(json);
    assert!(result.is_ok());
    let update = result.unwrap();
    assert_eq!(update.name, None);
}

#[test]
fn test_update_organization_deserializes_with_null_name() {
    let json = r#"{"name": null}"#;
    let result: Result<UpdateOrganization, _> = serde_json::from_str(json);
    assert!(result.is_ok());
    let update = result.unwrap();
    assert_eq!(update.name, None);
}

#[test]
fn test_organization_serializes_all_fields() {
    let org = Organization {
        id: "123e4567-e89b-12d3-a456-426614174000".to_string(),
        name: "Test Org".to_string(),
        slug: "test-org".to_string(),
        created_by: "user-123".to_string(),
        created_at: "2024-01-01T00:00:00Z".to_string(),
        updated_at: "2024-01-01T00:00:00Z".to_string(),
    };
    let json = serde_json::to_string(&org).unwrap();
    assert!(json.contains("\"id\""));
    assert!(json.contains("\"name\""));
    assert!(json.contains("\"slug\""));
    assert!(json.contains("\"created_by\""));
    assert!(json.contains("\"created_at\""));
    assert!(json.contains("\"updated_at\""));
}

#[test]
fn test_organization_deserializes_from_json() {
    let json = r#"{
        "id": "123e4567-e89b-12d3-a456-426614174000",
        "name": "Test Org",
        "slug": "test-org",
        "created_by": "user-123",
        "created_at": "2024-01-01T00:00:00Z",
        "updated_at": "2024-01-01T00:00:00Z"
    }"#;
    let result: Result<Organization, _> = serde_json::from_str(json);
    assert!(result.is_ok());
    let org = result.unwrap();
    assert_eq!(org.id, "123e4567-e89b-12d3-a456-426614174000");
    assert_eq!(org.name, "Test Org");
    assert_eq!(org.slug, "test-org");
}
