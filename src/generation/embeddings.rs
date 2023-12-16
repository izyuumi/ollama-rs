use serde::{Deserialize, Serialize};

use crate::Ollama;

use super::options::GenerationOptions;

impl Ollama {
    /// Generate embeddings from a model
    /// * `model_name` - Name of model to generate embeddings from
    /// * `prompt` - Prompt to generate embeddings for
    pub async fn generate_embeddings(
        &self,
        model_name: String,
        prompt: String,
        options: Option<GenerationOptions>,
    ) -> crate::error::Result<GenerateEmbeddingsResponse> {
        let request = GenerateEmbeddingsRequest {
            model_name,
            prompt,
            options,
        };

        let uri = format!("{}/api/embeddings", self.uri());
        let serialized = serde_json::to_string(&request).map_err(|e| e.to_string())?;
        let res = self
            .reqwest_client
            .post(uri)
            .body(serialized)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if !res.status().is_success() {
            return Err(res.text().await.unwrap_or_else(|e| e.to_string()).into());
        }

        let res = res.bytes().await.map_err(|e| e.to_string())?;
        let res = String::from_utf8(res.to_vec()).map_err(|e| e.to_string())?;
        let res = json5::from_str::<GenerateEmbeddingsResponse>(&res)
            .map_err(|e| e.to_string())?;

        Ok(res)
    }
}

/// An embeddings generation request to Ollama.
#[derive(Debug, Serialize)]
struct GenerateEmbeddingsRequest {
    #[serde(rename = "model")]
    model_name: String,
    prompt: String,
    options: Option<GenerationOptions>,
}

/// An embeddings generation response from Ollama.
#[derive(Debug, Deserialize, Clone)]
pub struct GenerateEmbeddingsResponse {
    #[serde(rename = "embedding")]
    #[allow(dead_code)]
    pub embeddings: Vec<f64>,
}
