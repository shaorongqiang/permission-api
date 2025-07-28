use std::sync::Arc;

use sea_orm::ConnectionTrait;
use utoipa_axum::router::OpenApiRouter;

use crate::{
    controller::{auth, menu, role, user},
    web_state::WebState,
};

pub fn router<C>(state: Arc<WebState<C>>) -> OpenApiRouter
where
    C: ConnectionTrait + Send + Sync + 'static,
{
    OpenApiRouter::new()
        .merge(auth::router(state.clone()))
        .merge(user::router(state.clone()))
        .merge(role::router(state.clone()))
        .merge(menu::router(state.clone()))
}
