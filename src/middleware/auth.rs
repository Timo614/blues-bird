use crate::AppState;
use axum::body::Body;
use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};

pub async fn auth_middleware(
    State(state): State<AppState>,
    request: Request<Body>,
    next: Next,
) -> Response {
    // Retrieve the API key from the state
    let expected_api_key = &state.api_key;

    // Check the Authorization header
    if let Some(auth_header) = request.headers().get("Authorization") {
        if auth_header == expected_api_key {
            return next.run(request).await;
        }
    }

    (StatusCode::UNAUTHORIZED, "Unauthorized").into_response()
}
