mod types;
use types::{CreateRequest, GetResponse, ListRequest, ListResponse, UpdateRequest};

use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    middleware,
};
use axum_valid::Valid;
use sea_orm::ConnectionTrait;
use serde_json::Value;
use utoipa_axum::{router::OpenApiRouter, routes};

use super::{
    ROLE_TAG,
    api_type::{ApiRequest, ApiResponse},
    middleware::auth_middleware,
};
use crate::{service::role, web_state::WebState};

#[utoipa::path(
  post,
  path = "/role/list",
  request_body(content = ApiRequest<ListRequest>, content_type = "application/json"),
  responses((status = OK, body = ApiResponse<ListResponse>,content_type = "application/json", description = "list users")),
  tag = ROLE_TAG,
  security(
    ("Bearer" = [])
  )
)]
pub async fn role_list<C>(
    State(state): State<Arc<WebState<C>>>,
    Valid(Json(request)): Valid<Json<ApiRequest<ListRequest>>>,
) -> Result<Json<ApiResponse<ListResponse>>, (StatusCode, String)>
where
    C: ConnectionTrait,
{
    let roles = role::list(&state.db, request.params.page, request.params.page_size)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let response = ApiResponse::new_success(
        request.id,
        ListResponse {
            roles: roles.into_iter().map(|role| role.into()).collect(),
        },
    );
    Ok(Json(response))
}

#[utoipa::path(
    post,
    path = "/role/create",
    request_body(content = ApiRequest<CreateRequest>, content_type = "application/json"),
    responses((status = OK, body = ApiResponse<String>,content_type = "application/json", description = "create user")),
    tag = ROLE_TAG,
    security(
        ("Bearer" = [])
    )
)]
pub async fn role_create<C>(
    State(state): State<Arc<WebState<C>>>,
    Valid(Json(request)): Valid<Json<ApiRequest<CreateRequest>>>,
) -> Result<Json<ApiResponse<String>>, (StatusCode, String)>
where
    C: ConnectionTrait,
{
    role::create(
        &state.db,
        &request.params.name,
        request.params.data_scope,
        request.params.status,
    )
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let response = ApiResponse::new_success(request.id, "success".to_string());
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/role/delete/{id}",
    responses((status = OK, body = ApiResponse<String>,content_type = "application/json", description = "delete user")),
    tag = ROLE_TAG,
    security(
        ("Bearer" = [])
    )
)]
pub async fn role_delete<C>(
    State(state): State<Arc<WebState<C>>>,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<String>>, (StatusCode, String)>
where
    C: ConnectionTrait,
{
    role::delete(&state.db, id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let response = ApiResponse::new_success(Value::Number(id.into()), "success".to_string());
    Ok(Json(response))
}

#[utoipa::path(
    post,
    path = "/role/update",
    request_body(content = ApiRequest<UpdateRequest>, content_type = "application/json"),
    responses((status = OK, body = ApiResponse<String>,content_type = "application/json", description = "update user")),
    tag = ROLE_TAG,
    security(
        ("Bearer" = [])
    )
)]
pub async fn role_update<C>(
    State(state): State<Arc<WebState<C>>>,
    Json(request): Json<ApiRequest<UpdateRequest>>,
) -> Result<Json<ApiResponse<String>>, (StatusCode, String)>
where
    C: ConnectionTrait,
{
    role::update(
        &state.db,
        request.params.id,
        request.params.name,
        request.params.data_scope,
        request.params.status,
    )
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let response = ApiResponse::new_success(request.id, "success".to_string());
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/role/get/{id}",
    responses((status = OK, body = ApiResponse<GetResponse>,content_type = "application/json", description = "get user")),
    tag = ROLE_TAG,
    security(
        ("Bearer" = [])
    )
)]
pub async fn role_get<C>(
    State(state): State<Arc<WebState<C>>>,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<GetResponse>>, (StatusCode, String)>
where
    C: ConnectionTrait,
{
    let role = role::get(&state.db, id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if let Some(role) = role {
        let response =
            ApiResponse::new_success(Value::Number(id.into()), GetResponse { role: role.into() });
        Ok(Json(response))
    } else {
        Err((StatusCode::NOT_FOUND, "Role not found".to_string()))
    }
}

pub fn router<C>(state: Arc<WebState<C>>) -> OpenApiRouter
where
    C: ConnectionTrait + Send + Sync + 'static,
{
    OpenApiRouter::new()
        .routes(routes!(role_list))
        .routes(routes!(role_create))
        .routes(routes!(role_delete))
        .routes(routes!(role_update))
        .routes(routes!(role_get))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(state)
}
