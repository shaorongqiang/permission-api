mod types;
use types::{LoginReqest, LoginResponse, RegisterRequest, RegisterResponse};

use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};
use sea_orm::ConnectionTrait;
use utoipa_axum::{router::OpenApiRouter, routes};

use super::{
    AUTH_TAG,
    api_type::{ApiRequest, ApiResponse},
};
use crate::{
    service::{online, user},
    web_state::WebState,
};

#[utoipa::path(
  post,
  path = "/auth/login",
  request_body(content = ApiRequest<LoginReqest>, content_type = "application/json"),
  responses((status = OK, body = ApiResponse<LoginResponse>,content_type = "application/json", description = "Login success")),
  tag = AUTH_TAG
)]
pub async fn auth_login<C>(
    State(state): State<Arc<WebState<C>>>,
    Json(request): Json<ApiRequest<LoginReqest>>,
) -> Result<Json<ApiResponse<LoginResponse>>, (StatusCode, String)>
where
    C: ConnectionTrait,
{
    let user = user::get_by_username(&state.db, &request.params.username)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let response = if let Some(user) = user {
        if user.password != request.params.password {
            ApiResponse::wrong_password(request.id)
        } else {
            let online = online::create(&state.db, user.id)
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            ApiResponse::new_success(
                request.id,
                LoginResponse {
                    token: online.token,
                },
            )
        }
    } else {
        ApiResponse::username_not_found(request.id)
    };

    Ok(Json(response))
}

#[utoipa::path(
    post,
    path = "/auth/register",
    request_body(content = ApiRequest<RegisterRequest>, content_type = "application/json"),
    responses((status = OK, body = ApiResponse<RegisterResponse>,content_type = "application/json", description = "Register success")),
    tag = AUTH_TAG
  )]
pub async fn auth_register<C>(
    State(state): State<Arc<WebState<C>>>,
    Json(request): Json<ApiRequest<RegisterRequest>>,
) -> Result<Json<ApiResponse<RegisterResponse>>, (StatusCode, String)>
where
    C: ConnectionTrait,
{
    if user::get_by_username(&state.db, &request.params.username)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .is_some()
    {
        return Err((
            StatusCode::BAD_REQUEST,
            "Username already exists".to_string(),
        ));
    }

    let user = user::create(
        &state.db,
        &request.params.username,
        &request.params.password,
    )
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let online = online::create(&state.db, user.id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let response = ApiResponse::new_success(
        request.id,
        RegisterResponse {
            token: online.token,
        },
    );
    Ok(Json(response))
}

pub fn router<C>(state: Arc<WebState<C>>) -> OpenApiRouter
where
    C: ConnectionTrait + Send + Sync + 'static,
{
    OpenApiRouter::new()
        .routes(routes!(auth_login))
        .routes(routes!(auth_register))
        .with_state(state)
}
