use crate::controllers;
use crate::AppState;
use axum::middleware;
use axum::routing::{get, post};
use axum::Router;
use shuttle_axum::AxumService;

pub(crate) async fn routing(state: AppState) -> AxumService {
    Router::new()
        .route(
            "/birds",
            get(controllers::birds::index))
        .route(
                "/birds",post(controllers::birds::create).route_layer(middleware::from_fn_with_state(
                    state.clone(),
                    crate::middleware::auth_middleware,
                )),
        )
        .with_state(state)
        .into()
}
