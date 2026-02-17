use serde_json;
use statusforge_backend::features::projects::{CreateProject, UpdateProject, Project};

#[test]
fn test_create_project_deserializes_valid_json() {
    let json = r#"{"name": "My Project"}"#;
    let result: Result<CreateProject, _> = serde_json::from_str(json);
    assert!(result.is_ok());
    let project = result.unwrap();
    assert_eq!(project.name, "My Project");
    assert_eq!(project.description, None);
}

#[test]
fn test_create_project_deserializes_with_description() {
    let json = r#"{"name": "My Project", "description": "Project description"}"#;
    let result: Result<CreateProject, _> = serde_json::from_str(json);
    assert!(result.is_ok());
    let project = result.unwrap();
    assert_eq!(project.name, "My Project");
    assert_eq!(project.description, Some("Project description".to_string()));
}

#[test]
fn test_create_project_requires_name() {
    let json = r#"{}"#;
    let result: Result<CreateProject, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

#[test]
fn test_update_project_deserializes_with_name() {
    let json = r#"{"name": "Updated Name"}"#;
    let result: Result<UpdateProject, _> = serde_json::from_str(json);
    assert!(result.is_ok());
    let update = result.unwrap();
    assert_eq!(update.name, Some("Updated Name".to_string()));
    assert_eq!(update.description, None);
}

#[test]
fn test_update_project_deserializes_with_description() {
    let json = r#"{"description": "Updated description"}"#;
    let result: Result<UpdateProject, _> = serde_json::from_str(json);
    assert!(result.is_ok());
    let update = result.unwrap();
    assert_eq!(update.name, None);
    assert_eq!(update.description, Some("Updated description".to_string()));
}

#[test]
fn test_update_project_deserializes_empty() {
    let json = r#"{}"#;
    let result: Result<UpdateProject, _> = serde_json::from_str(json);
    assert!(result.is_ok());
    let update = result.unwrap();
    assert_eq!(update.name, None);
    assert_eq!(update.description, None);
}

#[test]
fn test_project_serializes_all_fields() {
    let project = Project {
        id: "123e4567-e89b-12d3-a456-426614174000".to_string(),
        organization_id: "org-123".to_string(),
        name: "Test Project".to_string(),
        slug: "test-project".to_string(),
        description: Some("Description".to_string()),
        created_by: "user-123".to_string(),
        created_at: "2024-01-01T00:00:00Z".to_string(),
        updated_at: "2024-01-01T00:00:00Z".to_string(),
    };
    let json = serde_json::to_string(&project).unwrap();
    assert!(json.contains("\"id\""));
    assert!(json.contains("\"organization_id\""));
    assert!(json.contains("\"name\""));
    assert!(json.contains("\"slug\""));
    assert!(json.contains("\"description\""));
}

#[test]
fn test_project_deserializes_from_json() {
    let json = r#"{
        "id": "123e4567-e89b-12d3-a456-426614174000",
        "organization_id": "org-123",
        "name": "Test Project",
        "slug": "test-project",
        "description": "Description",
        "created_by": "user-123",
        "created_at": "2024-01-01T00:00:00Z",
        "updated_at": "2024-01-01T00:00:00Z"
    }"#;
    let result: Result<Project, _> = serde_json::from_str(json);
    assert!(result.is_ok());
    let project = result.unwrap();
    assert_eq!(project.id, "123e4567-e89b-12d3-a456-426614174000");
    assert_eq!(project.organization_id, "org-123");
    assert_eq!(project.name, "Test Project");
    assert_eq!(project.slug, "test-project");
}
