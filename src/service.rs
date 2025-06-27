use flate2::{write::GzEncoder, Compression};
use sqlx::PgPool;
use std::io::Write;
use uuid::Uuid;

use crate::dto::GlbModelsRequest;
use crate::repository::save_file;

pub async fn compress_file(pool: &PgPool, file: GlbModelsRequest) -> Result<Uuid, sqlx::Error> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&file.model_data)?;
    let compressed_file = GlbModelsRequest::new(file.name, encoder.finish()?);

    save_file(pool, compressed_file).await
}
