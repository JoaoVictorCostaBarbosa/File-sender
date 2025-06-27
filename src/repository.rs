use axum::http::StatusCode;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{dto::GlbModelsRequest, model::GlbModels};

pub async fn save_file(
    pool: &PgPool,
    glb_models_request: GlbModelsRequest,
) -> Result<Uuid, sqlx::Error> {
    let glb_models = GlbModels::new(glb_models_request.name, glb_models_request.model_data);

    sqlx::query!(
        r#"
        INSERT INTO glb_models
        (
            id,
            name,
            model_data,
            created_at
        )
        VALUES ($1, $2, $3, $4)
        "#,
        glb_models.id,
        glb_models.name,
        glb_models.model_data,
        glb_models.created_at
    )
    .execute(pool)
    .await?;

    Ok(glb_models.id)
}

pub async fn get_file(pool: &PgPool, id: Uuid) -> Result<GlbModels, sqlx::Error> {
    let file = sqlx::query_as!(
        GlbModels,
        r#"
        SELECT id, name, model_data, created_at
        FROM glb_models
        WHERE id = $1
        "#,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(file)
}

pub async fn delete_file(pool: &PgPool, id: Uuid) -> Result<StatusCode, sqlx::Error> {
    sqlx::query!(
        r#"
        DELETE FROM glb_models
        WHERE id = $1
        "#,
        id
    )
    .execute(pool)
    .await?;

    Ok(StatusCode::OK)
}
