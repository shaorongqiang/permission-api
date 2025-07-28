//#![deny(warnings, unused_crate_dependencies)]

pub mod entity;

pub mod service;

pub mod web_state;

pub mod controller;

use std::{net::SocketAddr, sync::Arc};

use anyhow::Result;
use axum::middleware::{self};
use sea_orm::ConnectionTrait;
use tokio::net::TcpListener;
use utoipa::{
    OpenApi,
    openapi::{
        Components,
        security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    },
};
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    controller::router,
    web_state::{ApiDoc, WebState, log_request},
};

pub async fn app_start<C>(addr: &SocketAddr, state: Arc<WebState<C>>) -> Result<()>
where
    C: ConnectionTrait + Send + Sync + 'static,
{
    let mut components = Components::new();
    components.security_schemes.insert(
        "Bearer".to_string(),
        SecurityScheme::Http(HttpBuilder::new().scheme(HttpAuthScheme::Bearer).build()),
    );

    let mut openapi = ApiDoc::openapi();
    openapi.components = Some(components);

    let (router, api) = OpenApiRouter::with_openapi(openapi)
        .merge(router(state))
        .layer(middleware::from_fn(log_request))
        .split_for_parts();

    let swagger_ui = SwaggerUi::new("/swagger").url("/apidoc/openapi.json", api);

    let app = router.merge(swagger_ui);

    let listener = TcpListener::bind(addr).await?;
    tracing::info!("Listening on http://{addr}");

    axum::serve(listener, app).await?;
    Ok(())
}

#[cfg(test)]
mod tests;
