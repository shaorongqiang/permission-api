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
    MENU_TAG,
    api_type::{ApiRequest, ApiResponse},
    middleware::auth_middleware,
};
use crate::{service::menu, web_state::WebState};

#[utoipa::path(
  post,
  path = "/menu/list",
  request_body(content = ApiRequest<ListRequest>, content_type = "application/json"),
  responses((status = OK, body = ApiResponse<ListResponse>,content_type = "application/json", description = "list users")),
  tag = MENU_TAG,
  security(
    ("Bearer" = [])
  )
)]
pub async fn menu_list<C>(
    State(state): State<Arc<WebState<C>>>,
    Valid(Json(request)): Valid<Json<ApiRequest<ListRequest>>>,
) -> Result<Json<ApiResponse<ListResponse>>, (StatusCode, String)>
where
    C: ConnectionTrait,
{
    let menus = menu::list(&state.db, request.params.page, request.params.page_size)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let response = ApiResponse::new_success(
        request.id,
        ListResponse {
            menus: menus.into_iter().map(|menu| menu.into()).collect(),
        },
    );
    Ok(Json(response))
}

#[utoipa::path(
    post,
    path = "/menu/create",
    request_body(content = ApiRequest<CreateRequest>, content_type = "application/json"),
    responses((status = OK, body = ApiResponse<String>,content_type = "application/json", description = "create user")),
    tag = MENU_TAG,
    security(
        ("Bearer" = [])
    )
)]
pub async fn menu_create<C>(
    State(state): State<Arc<WebState<C>>>,
    Valid(Json(request)): Valid<Json<ApiRequest<CreateRequest>>>,
) -> Result<Json<ApiResponse<String>>, (StatusCode, String)>
where
    C: ConnectionTrait,
{
    menu::create(
        &state.db,
        &request.params.name,
        &request.params.path,
        request.params.is_frame,
    )
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let response = ApiResponse::new_success(request.id, "success".to_string());
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/menu/delete/{id}",
    responses((status = OK, body = ApiResponse<String>,content_type = "application/json", description = "delete user")),
    tag = MENU_TAG,
    security(
        ("Bearer" = [])
    )
)]
pub async fn menu_delete<C>(
    State(state): State<Arc<WebState<C>>>,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<String>>, (StatusCode, String)>
where
    C: ConnectionTrait,
{
    menu::delete(&state.db, id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let response = ApiResponse::new_success(Value::Number(id.into()), "success".to_string());
    Ok(Json(response))
}

#[utoipa::path(
    post,
    path = "/menu/update",
    request_body(content = ApiRequest<UpdateRequest>, content_type = "application/json"),
    responses((status = OK, body = ApiResponse<String>,content_type = "application/json", description = "update user")),
    tag = MENU_TAG,
    security(
        ("Bearer" = [])
    )
)]
pub async fn menu_update<C>(
    State(state): State<Arc<WebState<C>>>,
    Json(request): Json<ApiRequest<UpdateRequest>>,
) -> Result<Json<ApiResponse<String>>, (StatusCode, String)>
where
    C: ConnectionTrait,
{
    menu::update(
        &state.db,
        request.params.id,
        request.params.name,
        request.params.path,
        request.params.is_frame,
    )
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let response = ApiResponse::new_success(request.id, "success".to_string());
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/menu/get/{id}",
    responses((status = OK, body = ApiResponse<GetResponse>,content_type = "application/json", description = "get user")),
    tag = MENU_TAG,
    security(
        ("Bearer" = [])
    )
)]
pub async fn menu_get<C>(
    State(state): State<Arc<WebState<C>>>,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<GetResponse>>, (StatusCode, String)>
where
    C: ConnectionTrait,
{
    let menu = menu::get(&state.db, id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if let Some(menu) = menu {
        let response =
            ApiResponse::new_success(Value::Number(id.into()), GetResponse { menu: menu.into() });
        Ok(Json(response))
    } else {
        Err((StatusCode::NOT_FOUND, "Menu not found".to_string()))
    }
}

pub fn router<C>(state: Arc<WebState<C>>) -> OpenApiRouter
where
    C: ConnectionTrait + Send + Sync + 'static,
{
    OpenApiRouter::new()
        .routes(routes!(menu_list))
        .routes(routes!(menu_create))
        .routes(routes!(menu_delete))
        .routes(routes!(menu_update))
        .routes(routes!(menu_get))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(state)
}
