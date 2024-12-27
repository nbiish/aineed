pub mod openai;
pub mod anthropic;
pub mod togetherai;
pub mod openrouter;

use futures_util::{Stream, StreamExt};
use std::pin::Pin;
use crate::error::AiNeedError;

pub type StreamResult = Pin<Box<dyn Stream<Item = Result<String, AiNeedError>> + Send>>;

pub async fn generate_completion(
    provider: &str,
    model: &str,
    prompt: &str,
    max_tokens: u32,
    temperature: f32,
) -> Result<String, AiNeedError> {
    match provider {
        "openai" => openai::generate_completion(model, prompt, max_tokens, temperature).await,
        "anthropic" => anthropic::generate_completion(model, prompt, max_tokens, temperature).await,
        "togetherai" => togetherai::generate_completion(model, prompt, max_tokens, temperature).await,
        "openrouter" => openrouter::generate_completion(model, prompt, max_tokens, temperature).await,
        _ => Err(AiNeedError::UnsupportedModel {
            provider: provider.to_string(),
            model: model.to_string(),
            supported_models: vec![
                "openai:gpt-4".to_string(),
                "anthropic:claude-3".to_string(),
                "togetherai:llama".to_string(),
                "openrouter:gemini".to_string(),
            ],
        }),
    }
}

pub async fn stream_completion(
    provider: &str,
    model: &str,
    prompt: &str,
    max_tokens: u32,
    temperature: f32,
) -> Result<StreamResult, AiNeedError> {
    // Validate provider and model first
    let provider_stream = match provider {
        "openai" => openai::stream_completion(model, prompt, max_tokens, temperature).await,
        "anthropic" => anthropic::stream_completion(model, prompt, max_tokens, temperature).await,
        "togetherai" => togetherai::stream_completion(model, prompt, max_tokens, temperature).await,
        "openrouter" => openrouter::stream_completion(model, prompt, max_tokens, temperature).await,
        _ => Err(AiNeedError::UnsupportedModel {
            provider: provider.to_string(),
            model: model.to_string(),
            supported_models: vec![
                "openai:gpt-4".to_string(),
                "anthropic:claude-3".to_string(),
                "togetherai:llama".to_string(),
                "openrouter:gemini".to_string(),
            ],
        }),
    }?;

    // Transform the stream to handle formatting
    let formatted_stream = provider_stream.map(|chunk_result| {
        chunk_result.map(|chunk| {
            if chunk.trim().is_empty() {
                String::new()
            } else {
                // Clean up the text chunk
                chunk.chars()
                    .filter(|&c| c.is_ascii_graphic() || c.is_whitespace())
                    .collect::<String>()
                    .trim()
                    .to_string() + " "
            }
        })
    });

    Ok(Box::pin(formatted_stream))
}

pub async fn generate_image(
    provider: &str,
    model: &str,
    prompt: &str,
    output_path: Option<&str>,
) -> Result<String, AiNeedError> {
    match provider {
        "openai" => openai::generate_image(model, prompt, output_path).await,
        "togetherai" => togetherai::generate_image(model, prompt, output_path).await,
        _ => Err(AiNeedError::UnsupportedModel {
            provider: provider.to_string(),
            model: model.to_string(),
            supported_models: vec!["Provider not supported for image generation".to_string()],
        }),
    }
} 