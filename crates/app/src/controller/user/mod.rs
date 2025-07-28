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
    USER_TAG,
    api_type::{ApiRequest, ApiResponse},
    middleware::auth_middleware,
};
use crate::{service::user, web_state::WebState};

#[utoipa::path(
  post,
  path = "/user/list",
  request_body(content = ApiRequest<ListRequest>, content_type = "application/json"),
  responses((status = OK, body = ApiResponse<ListResponse>,content_type = "application/json", description = "list users")),
  tag = USER_TAG,
  security(
    ("Bearer" = [])
  )
)]
pub async fn user_list<C>(
    State(state): State<Arc<WebState<C>>>,
    Valid(Json(request)): Valid<Json<ApiRequest<ListRequest>>>,
) -> Result<Json<ApiResponse<ListResponse>>, (StatusCode, String)>
where
    C: ConnectionTrait,
{
    let users = user::list(&state.db, request.params.page, request.params.page_size)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let response = ApiResponse::new_success(
        request.id,
        ListResponse {
            users: users.into_iter().map(|user| user.into()).collect(),
        },
    );
    Ok(Json(response))
}

#[utoipa::path(
    post,
    path = "/user/create",
    request_body(content = ApiRequest<CreateRequest>, content_type = "application/json"),
    responses((status = OK, body = ApiResponse<String>,content_type = "application/json", description = "create user")),
    tag = USER_TAG,
    security(
        ("Bearer" = [])
    )
)]
pub async fn user_create<C>(
    State(state): State<Arc<WebState<C>>>,
    Valid(Json(request)): Valid<Json<ApiRequest<CreateRequest>>>,
) -> Result<Json<ApiResponse<String>>, (StatusCode, String)>
where
    C: ConnectionTrait,
{
    if !check_user_exists(&state.db, None, Some(&request.params.username)).await? {
        return Err((
            StatusCode::BAD_REQUEST,
            "Username already exists".to_string(),
        ));
    }
    user::create(
        &state.db,
        &request.params.username,
        &request.params.password,
    )
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let response = ApiResponse::new_success_without_data(request.id);
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/user/delete/{id}",
    responses((status = OK, body = ApiResponse<String>,content_type = "application/json", description = "delete user")),
    tag = USER_TAG,
    security(
        ("Bearer" = [])
    )
)]
pub async fn user_delete<C>(
    State(state): State<Arc<WebState<C>>>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<String>>, (StatusCode, String)>
where
    C: ConnectionTrait,
{
    if !check_user_exists(&state.db, Some(id), None).await? {
        return Err((StatusCode::BAD_REQUEST, "User not found".to_string()));
    }
    user::delete(&state.db, id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let response = ApiResponse::new_success_without_data(Value::Number(id.into()));
    Ok(Json(response))
}

#[utoipa::path(
    post,
    path = "/user/update",
    request_body(content = ApiRequest<UpdateRequest>, content_type = "application/json"),
    responses((status = OK, body = ApiResponse<String>,content_type = "application/json", description = "update user")),
    tag = USER_TAG,
    security(
        ("Bearer" = [])
    )
)]
pub async fn user_update<C>(
    State(state): State<Arc<WebState<C>>>,
    Json(request): Json<ApiRequest<UpdateRequest>>,
) -> Result<Json<ApiResponse<String>>, (StatusCode, String)>
where
    C: ConnectionTrait,
{
    if !check_user_exists(&state.db, Some(request.params.id), None).await? {
        return Err((StatusCode::BAD_REQUEST, "User not found".to_string()));
    }
    user::update(
        &state.db,
        request.params.id,
        request.params.username,
        request.params.password,
    )
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let response = ApiResponse::new_success_without_data(request.id);
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/user/get/{id}",
    responses((status = OK, body = ApiResponse<GetResponse>,content_type = "application/json", description = "get user")),
    tag = USER_TAG,
    security(
        ("Bearer" = [])
    )
)]
pub async fn user_get<C>(
    State(state): State<Arc<WebState<C>>>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<GetResponse>>, (StatusCode, String)>
where
    C: ConnectionTrait,
{
    let user = user::get(&state.db, id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if let Some(user) = user {
        let response =
            ApiResponse::new_success(Value::Number(id.into()), GetResponse { user: user.into() });
        Ok(Json(response))
    } else {
        Err((StatusCode::NOT_FOUND, "User not found".to_string()))
    }
}

async fn check_user_exists<C: ConnectionTrait>(
    db: &C,
    id: Option<i64>,
    username: Option<&str>,
) -> Result<bool, (StatusCode, String)> {
    if let Some(id) = id {
        let user = user::get(db, id)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        Ok(user.is_some())
    } else if let Some(username) = username {
        let user = user::get_by_username(db, username)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        Ok(user.is_some())
    } else {
        Ok(false)
    }
}

pub fn router<C>(state: Arc<WebState<C>>) -> OpenApiRouter
where
    C: ConnectionTrait + Send + Sync + 'static,
{
    OpenApiRouter::new()
        .routes(routes!(user_list))
        .routes(routes!(user_create))
        .routes(routes!(user_delete))
        .routes(routes!(user_update))
        .routes(routes!(user_get))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(state)
}
