use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct GlbModelsRequest {
    pub name: String,
    pub model_data: Vec<u8>,
}

impl GlbModelsRequest {
    pub fn new(name: String, model_data: Vec<u8>) -> Self {
        GlbModelsRequest { name, model_data }
    }
}
