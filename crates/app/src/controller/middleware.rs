use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use sea_orm::ConnectionTrait;
use std::sync::Arc;

use crate::{
    service::online::{get_menu_path_by_token, is_admin_by_token},
    web_state::WebState,
};

pub const AUTH_HEADER: &str = "Authorization";

pub async fn auth_middleware<C>(
    State(state): State<Arc<WebState<C>>>,
    request: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)>
where
    C: ConnectionTrait,
{
    let token = request
        .headers()
        .get(AUTH_HEADER)
        .ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                String::from("Authorization not found in header"),
            )
        })
        .and_then(|v| {
            v.to_str().map(String::from).map_err(|e| {
                (
                    StatusCode::BAD_REQUEST,
                    format!("format Authorization error: {e}"),
                )
            })
        })?;

    let token = token.trim_start_matches("Bearer ");
    tracing::debug!("token: {}", token);

    if is_admin_by_token(&state.db, token)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    {
        tracing::debug!("is admin");
        Ok(next.run(request).await)
    } else {
        let menus = get_menu_path_by_token(&state.db, token)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        let uri = request.uri().path();

        tracing::debug!("uri: {}, menus: {:?}", uri, menus);

        if !menus.iter().any(|menu| uri.starts_with(menu)) {
            return Err((StatusCode::FORBIDDEN, String::from("No permission")));
        }

        let response = next.run(request).await;

        Ok(response)
    }
}
