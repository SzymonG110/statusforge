use crate::AppState;

use super::Project;

fn slugify(name: &str) -> String {
    let mut result = String::new();
    let mut last_was_dash = false;
    
    for c in name.to_lowercase().chars() {
        if c.is_alphanumeric() {
            result.push(c);
            last_was_dash = false;
        } else if !last_was_dash {
            result.push('-');
            last_was_dash = true;
        }
    }
    
    result.trim_matches('-').to_string()
}

pub async fn list_by_org(state: &AppState, organization_id: &str) -> Result<Vec<Project>, crate::ApiError> {
    let projects = state
        .supabase
        .database()
        .from("projects")
        .select("*")
        .eq("organization_id", organization_id)
        .execute::<Project>()
        .await
        .map_err(|e| {
            eprintln!("Database error: {:?}", e);
            crate::ApiError::InternalServerError
        })?;

    Ok(projects)
}

pub async fn get_by_id(state: &AppState, id: &str) -> Result<Project, crate::ApiError> {
    let mut projects = state
        .supabase
        .database()
        .from("projects")
        .select("*")
        .eq("id", id)
        .execute::<Project>()
        .await
        .map_err(|e| {
            eprintln!("Database error: {:?}", e);
            crate::ApiError::NotFound
        })?;

    projects.pop().ok_or(crate::ApiError::NotFound)
}

pub async fn create(
    state: &AppState,
    organization_id: &str,
    name: &str,
    description: Option<&str>,
    user_id: &str,
) -> Result<Project, crate::ApiError> {
    let base_slug = slugify(name);
    let slug = format!("{}-{}", base_slug, uuid::Uuid::new_v4().to_string()[..8].to_string());
    
    let mut data = serde_json::json!({
        "organization_id": organization_id,
        "name": name,
        "slug": slug,
        "created_by": user_id,
    });

    if let Some(desc) = description {
        data["description"] = serde_json::Value::String(desc.to_string());
    }

    let mut projects = state
        .supabase
        .database()
        .insert("projects")
        .values(data)
        .map_err(|e| {
            eprintln!("Serialization error: {:?}", e);
            crate::ApiError::InternalServerError
        })?
        .returning("*")
        .execute::<Project>()
        .await
        .map_err(|e| {
            eprintln!("Database error: {:?}", e);
            crate::ApiError::BadRequest(format!("Failed to create project: {:?}", e))
        })?;

    projects.pop().ok_or(crate::ApiError::InternalServerError)
}

pub async fn update(
    state: &AppState,
    id: &str,
    name: Option<&str>,
    description: Option<&str>,
) -> Result<Project, crate::ApiError> {
    let mut data = serde_json::Map::new();
    if let Some(name) = name {
        data.insert("name".to_string(), serde_json::Value::String(name.to_string()));
    }
    if let Some(desc) = description {
        data.insert("description".to_string(), serde_json::Value::String(desc.to_string()));
    } else if description.is_some() {
        data.insert("description".to_string(), serde_json::Value::Null);
    }

    let mut projects = state
        .supabase
        .database()
        .update("projects")
        .set(serde_json::Value::Object(data))
        .map_err(|e| {
            eprintln!("Serialization error: {:?}", e);
            crate::ApiError::InternalServerError
        })?
        .eq("id", id)
        .returning("*")
        .execute::<Project>()
        .await
        .map_err(|e| {
            eprintln!("Database error: {:?}", e);
            crate::ApiError::NotFound
        })?;

    projects.pop().ok_or(crate::ApiError::NotFound)
}

pub async fn delete(state: &AppState, id: &str) -> Result<(), crate::ApiError> {
    state
        .supabase
        .database()
        .delete("projects")
        .eq("id", id)
        .execute::<serde_json::Value>()
        .await
        .map_err(|e| {
            eprintln!("Database error: {:?}", e);
            crate::ApiError::NotFound
        })?;

    Ok(())
}
