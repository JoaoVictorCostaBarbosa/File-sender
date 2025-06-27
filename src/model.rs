use chrono::Utc;
use serde::Deserialize;
use sqlx::{types::chrono::DateTime, FromRow};
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize)]
pub struct GlbModels {
    pub id: Uuid,
    pub name: String,
    pub model_data: Vec<u8>,
    pub created_at: Option<DateTime<Utc>>,
}

impl GlbModels {
    pub fn new(name: String, model_data: Vec<u8>) -> Self {
        GlbModels {
            id: Uuid::new_v4(),
            name,
            model_data,
            created_at: Some(chrono::Utc::now()),
        }
    }
}
