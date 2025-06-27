use axum::{
    body::Body,
    extract::{Path, State},
    http::{header, StatusCode},
    response::{Html, IntoResponse, Response},
    Json,
};
use axum_extra::extract::Multipart;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    dto::GlbModelsRequest,
    repository::{delete_file, get_file},
    service::compress_file,
};

pub async fn save_file_handler(
    State(pool): State<PgPool>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    let mut name = None;
    let mut data = None;

    while let Some(field) = multipart.next_field().await.unwrap() {
        match field.name() {
            Some("name") => {
                name = Some(field.text().await.unwrap());
            }
            Some("model_file") => {
                data = Some(field.bytes().await.unwrap().to_vec());
            }
            _ => {}
        }
    }

    let (name, data) = match (name, data) {
        (Some(n), Some(d)) => (n, d),
        _ => return StatusCode::BAD_REQUEST.into_response(),
    };

    match compress_file(&pool, GlbModelsRequest::new(name, data)).await {
        Ok(id) => (StatusCode::CREATED, Json(id)).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn get_file_handler(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match get_file(&pool, id).await {
        Ok(file) => {
            let conten_disposition = format!("attachment; filename=\"{}.glb.gz", file.name);

            Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "application/gzip")
                .header(header::CONTENT_DISPOSITION, conten_disposition)
                .body(Body::from(file.model_data))
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
                .into_response()
        }
        Err(sqlx::Error::RowNotFound) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn delete_file_handler(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match delete_file(&pool, id).await {
        Ok(response) => response.into_response(),
        Err(sqlx::Error::RowNotFound) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn tests() -> impl IntoResponse {
    let html = r#"
        <html>
            <head><title>Modelo Enviado</title></head>
            <body>
                <h1>API funcionando</h1>
                <p>...</p>
            </body>
        </html>
    "#;

    Html(html)
}
