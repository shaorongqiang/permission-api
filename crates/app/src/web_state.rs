use std::time::Instant;

use axum::{
    body::{Body, to_bytes},
    http::{Request, Response},
    middleware::Next,
    response::IntoResponse,
};
use sea_orm::ConnectionTrait;
use utoipa::OpenApi;
use uuid::Uuid;

use crate::controller::{AUTH_TAG, MENU_TAG, ROLE_TAG, USER_TAG};

#[derive(OpenApi)]
#[openapi(
    tags(
         (name = AUTH_TAG, description = "Auth API endpoints"),
         (name = USER_TAG, description = "User API endpoints"),
         (name = ROLE_TAG, description = "Role API endpoints"),
         (name = MENU_TAG, description = "Menu API endpoints"),
    ),
)]
pub struct ApiDoc;

pub struct WebState<C>
where
    C: ConnectionTrait,
{
    pub db: C,
}

impl<C> WebState<C>
where
    C: ConnectionTrait,
{
    pub fn new(db: C) -> Self {
        Self { db }
    }
}

pub async fn log_request(request: Request<Body>, next: Next) -> impl IntoResponse {
    let request_id = Uuid::new_v4();

    let request = {
        let (parts, body) = request.into_parts();
        tracing::debug!("[{}] Request: {:#?}", request_id, parts);
        let body = if let Ok(bytes) = to_bytes(body, usize::MAX).await {
            let body_str = String::from_utf8_lossy(&bytes);
            tracing::debug!("[{}] Request Body: {}", request_id, body_str);
            Body::from(bytes)
        } else {
            Body::from(vec![])
        };
        Request::from_parts(parts, body)
    };

    // 重新构建请求

    let start = Instant::now();

    let response = next.run(request).await;

    {
        let (parts, body) = response.into_parts();

        tracing::debug!(
            "[{}] run time: {}ms, Response: {:#?}",
            request_id,
            start.elapsed().as_millis(),
            parts,
        );

        let body = if let Ok(bytes) = to_bytes(body, usize::MAX).await {
            let body_str = String::from_utf8_lossy(&bytes);
            tracing::debug!("[{}] Response Body: {}", request_id, body_str);

            Body::from(bytes)
        } else {
            Body::from(vec![])
        };

        Response::from_parts(parts, body)
    }
}
