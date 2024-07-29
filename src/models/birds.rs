use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::Datelike;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Bird {
    pub id: i32,
    pub image: String,
    pub bird: String,
    pub country: String,
    pub location: String,
    pub created_at: DateTime<Utc>,
}

pub struct BirdContext {
    pub id: i32,
    pub image: String,
    pub name: String,
    pub location: String,
    pub date: String,
}

impl From<Bird> for BirdContext {
    fn from(bird: Bird) -> BirdContext {
        BirdContext {
            id: bird.id,
            image: bird.image.clone(),
            name: bird.bird.clone(),
            location: format!("{}, {}", bird.location, bird.country),
            date: format!("{} {}, {}", bird.created_at.format("%b"), bird.created_at.day(), bird.created_at.year())
        }
    }
}