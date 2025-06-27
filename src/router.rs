use axum::{
    routing::{delete, get, post},
    Router,
};
use sqlx::PgPool;

use crate::handler::{delete_file_handler, get_file_handler, save_file_handler, tests};

pub fn glb_router(pool: PgPool) -> Router {
    Router::new()
        .route("/test", get(tests))
        .route("/file", post(save_file_handler))
        .route("/file/:id", get(get_file_handler))
        .route("/file/:id", delete(delete_file_handler))
        .with_state(pool)
}
