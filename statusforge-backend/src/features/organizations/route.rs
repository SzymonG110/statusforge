use axum::{extract::{Path, Query, State}, routing::{delete, get, post, put}, Json, Router};
use serde::Deserialize;
use serde_json::Value;

use crate::{AppState, ApiError};

use super::{service, CreateOrganization, UpdateOrganization};

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

async fn list_organizations(
    State(state): State<AppState>,
    Query(query): Query<UserIdQuery>,
) -> Result<Json<Value>, ApiError> {
    let user_id = get_user_id(Some(Query(query))).await?;
    let orgs = service::list_organizations(&state, &user_id).await?;
    Ok(Json(serde_json::to_value(orgs).unwrap()))
}

async fn get_organization(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Value>, ApiError> {
    let org = service::get_organization(&state, &id).await?;
    Ok(Json(serde_json::to_value(org).unwrap()))
}

async fn create_organization(
    State(state): State<AppState>,
    Query(query): Query<UserIdQuery>,
    Json(data): Json<CreateOrganization>,
) -> Result<Json<Value>, ApiError> {
    let user_id = get_user_id(Some(Query(query))).await?;
    let org = service::create_organization(&state, data, &user_id).await?;
    Ok(Json(serde_json::to_value(org).unwrap()))
}

async fn update_organization(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(data): Json<UpdateOrganization>,
) -> Result<Json<Value>, ApiError> {
    let org = service::update_organization(&state, &id, data).await?;
    Ok(Json(serde_json::to_value(org).unwrap()))
}

async fn delete_organization(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Value>, ApiError> {
    service::delete_organization(&state, &id).await?;
    Ok(Json(serde_json::json!({ "message": "Organization deleted" })))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(list_organizations).post(create_organization))
        .route("/{id}", get(get_organization).put(update_organization).delete(delete_organization))
}
