use crate::models::*;
use crate::AppState;
use askama::Template;
use axum::extract::State;
use axum::{http::StatusCode, response::{IntoResponse, Html}, Json};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct BirdUpload {
    bird: String,
    image: String,
    country: String,
    location: String,
}

#[derive(Template)]
#[template(path = "birds.html")] 
struct BirdTemplate {
    birds: Vec<BirdContext>,
}

pub async fn index(State(state): State<AppState>) -> Result<Html<String>, StatusCode> {
    match sqlx::query_as::<_, Bird>(
        "SELECT id, image, bird, country, location, created_at FROM birds ORDER BY created_at DESC",
    )
    .fetch_all(&state.pool)
    .await
    {
        Ok(birds) => {
            let bird_html = BirdTemplate { birds: birds.into_iter().map(|b| b.into()).collect() };

            match bird_html.render() {
                Ok(html) => Ok(Html(html)),
                Err(_) => Err(StatusCode::BAD_REQUEST)
            }
        },
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

pub async fn create(
    State(state): State<AppState>,
    Json(payload): Json<BirdUpload>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    if let Err(e) = sqlx::query(
        r#"
        INSERT INTO birds (bird, image, country, location)
        VALUES ($1, $2, $3, $4)
        RETURNING id
        "#,
    )
    .bind(payload.bird)
    .bind(payload.image)
    .bind(payload.country)
    .bind(payload.location)
    .execute(&state.pool)
    .await
    {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {e}"),
        ));
    }
    Ok(StatusCode::CREATED)
}
