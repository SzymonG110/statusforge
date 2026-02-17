use axum::{extract::{Path, Query, State}, routing::{delete, get, post, put}, Json, Router};
use serde::Deserialize;
use serde_json::Value;

use crate::{AppState, ApiError};

use super::{service, CreateProject, UpdateProject};

#[derive(Deserialize)]
struct UserIdQuery {
    user_id: Option<String>,
}

async fn get_user_id(query: Option<Query<UserIdQuery>>) -> Result<String, ApiError> {
    if let Some(Query(UserIdQuery { user_id: Some(id) })) = query {
        Ok(id)
    } else {
        Err(ApiError::BadRequest("user_id query parameter required (temporary - will use JWT later)".to_string()))
    }
}

async fn list_projects(
    State(state): State<AppState>,
    Path(organization_id): Path<String>,
    Query(query): Query<UserIdQuery>,
) -> Result<Json<Value>, ApiError> {
    let _user_id = get_user_id(Some(Query(query))).await?;
    let projects = service::list_projects(&state, &organization_id).await?;
    Ok(Json(serde_json::to_value(projects).unwrap()))
}

async fn get_project(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Value>, ApiError> {
    let project = service::get_project(&state, &id).await?;
    Ok(Json(serde_json::to_value(project).unwrap()))
}

async fn create_project(
    State(state): State<AppState>,
    Path(organization_id): Path<String>,
    Query(query): Query<UserIdQuery>,
    Json(data): Json<CreateProject>,
) -> Result<Json<Value>, ApiError> {
    let user_id = get_user_id(Some(Query(query))).await?;
    let project = service::create_project(&state, &organization_id, data, &user_id).await?;
    Ok(Json(serde_json::to_value(project).unwrap()))
}

async fn update_project(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(data): Json<UpdateProject>,
) -> Result<Json<Value>, ApiError> {
    let project = service::update_project(&state, &id, data).await?;
    Ok(Json(serde_json::to_value(project).unwrap()))
}

async fn delete_project(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Value>, ApiError> {
    service::delete_project(&state, &id).await?;
    Ok(Json(serde_json::json!({ "message": "Project deleted" })))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/organizations/:organization_id/projects", get(list_projects).post(create_project))
        .route("/projects/:id", get(get_project).put(update_project).delete(delete_project))
}
