use crate::AppState;
use serde_json::Value;

use super::Organization;

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

pub async fn list(state: &AppState, _user_id: &str) -> Result<Vec<Organization>, crate::ApiError> {
    let orgs = state
        .supabase
        .database()
        .from("organizations")
        .select("*")
        .execute::<Organization>()
        .await
        .map_err(|e| {
            eprintln!("Database error: {:?}", e);
            crate::ApiError::InternalServerError
        })?;

    Ok(orgs)
}

pub async fn get_by_id(state: &AppState, id: &str) -> Result<Organization, crate::ApiError> {
    let mut orgs = state
        .supabase
        .database()
        .from("organizations")
        .select("*")
        .eq("id", id)
        .execute::<Organization>()
        .await
        .map_err(|e| {
            eprintln!("Database error: {:?}", e);
            crate::ApiError::NotFound
        })?;

    orgs.pop().ok_or(crate::ApiError::NotFound)
}

pub async fn create(
    state: &AppState,
    name: &str,
    user_id: &str,
) -> Result<Organization, crate::ApiError> {
    let slug = format!("{}-{}", slugify(name), uuid::Uuid::new_v4().to_string()[..8].to_string());
    
    let data = serde_json::json!({
        "name": name,
        "slug": slug,
        "created_by": user_id,
    });

    let mut orgs = state
        .supabase
        .database()
        .insert("organizations")
        .values(data)
        .map_err(|e| {
            eprintln!("Serialization error: {:?}", e);
            crate::ApiError::InternalServerError
        })?
        .returning("*")
        .execute::<Organization>()
        .await
        .map_err(|e| {
            eprintln!("Database error: {:?}", e);
            crate::ApiError::BadRequest(format!("Failed to create organization: {:?}", e))
        })?;

    let org = orgs.pop().ok_or(crate::ApiError::InternalServerError)?;

    state
        .supabase
        .database()
        .insert("organization_members")
        .values(serde_json::json!({
            "organization_id": org.id,
            "user_id": user_id,
            "role": "owner"
        }))
        .map_err(|e| {
            eprintln!("Serialization error: {:?}", e);
            crate::ApiError::InternalServerError
        })?
        .execute::<serde_json::Value>()
        .await
        .map_err(|e| {
            eprintln!("Failed to add owner to organization: {:?}", e);
            crate::ApiError::InternalServerError
        })?;

    Ok(org)
}

pub async fn update(
    state: &AppState,
    id: &str,
    name: Option<&str>,
) -> Result<Organization, crate::ApiError> {
    let mut data = serde_json::Map::new();
    if let Some(name) = name {
        data.insert("name".to_string(), Value::String(name.to_string()));
    }

    let mut orgs = state
        .supabase
        .database()
        .update("organizations")
        .set(Value::Object(data))
        .map_err(|e| {
            eprintln!("Serialization error: {:?}", e);
            crate::ApiError::InternalServerError
        })?
        .eq("id", id)
        .returning("*")
        .execute::<Organization>()
        .await
        .map_err(|e| {
            eprintln!("Database error: {:?}", e);
            crate::ApiError::NotFound
        })?;

    orgs.pop().ok_or(crate::ApiError::NotFound)
}

pub async fn delete(state: &AppState, id: &str) -> Result<(), crate::ApiError> {
    state
        .supabase
        .database()
        .delete("organizations")
        .eq("id", id)
        .execute::<serde_json::Value>()
        .await
        .map_err(|e| {
            eprintln!("Database error: {:?}", e);
            crate::ApiError::NotFound
        })?;

    Ok(())
}
