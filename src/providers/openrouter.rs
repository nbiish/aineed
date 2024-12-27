use futures_util::{Stream, StreamExt};
use reqwest::Client;
use serde_json::json;
use std::pin::Pin;
use crate::error::AiNeedError;
use crate::config;

const API_BASE: &str = "https://openrouter.ai/api/v1";

pub async fn generate_completion(
    model: &str,
    prompt: &str,
    max_tokens: u32,
    temperature: f32,
) -> Result<String, AiNeedError> {
    let client = Client::new();
    let api_key = config::get_openrouter_key()?;

    let response = client
        .post(&format!("{}/chat/completions", API_BASE))
        .header("Authorization", format!("Bearer {}", api_key))
        .header("HTTP-Referer", "https://github.com/nbiish/aineed")
        .header("X-Title", "aineed")
        .json(&json!({
            "model": model,
            "messages": [{"role": "user", "content": prompt}],
            "max_tokens": max_tokens,
            "temperature": temperature,
        }))
        .send()
        .await
        .map_err(|e| AiNeedError::ApiError {
            provider: "openrouter".to_string(),
            endpoint: format!("{}/chat/completions", API_BASE),
            status: 500,
            message: e.to_string(),
        })?;

    let status = response.status();
    if !status.is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(AiNeedError::ApiError {
            provider: "openrouter".to_string(),
            endpoint: format!("{}/chat/completions", API_BASE),
            status: status.as_u16(),
            message: error_text,
        });
    }

    let json: serde_json::Value = response.json().await.map_err(|e| AiNeedError::ApiError {
        provider: "openrouter".to_string(),
        endpoint: format!("{}/chat/completions", API_BASE),
        status: 500,
        message: e.to_string(),
    })?;

    Ok(json["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("")
        .to_string())
}

pub async fn stream_completion(
    model: &str,
    prompt: &str,
    max_tokens: u32,
    temperature: f32,
) -> Result<Pin<Box<dyn Stream<Item = Result<String, AiNeedError>> + Send>>, AiNeedError> {
    let client = Client::new();
    let api_key = config::get_openrouter_key()?;

    let response = client
        .post(&format!("{}/chat/completions", API_BASE))
        .header("Authorization", format!("Bearer {}", api_key))
        .header("HTTP-Referer", "https://github.com/nbiish/aineed")
        .header("X-Title", "aineed")
        .json(&json!({
            "model": model,
            "messages": [{"role": "user", "content": prompt}],
            "max_tokens": max_tokens,
            "temperature": temperature,
            "stream": true,
        }))
        .send()
        .await
        .map_err(|e| AiNeedError::ApiError {
            provider: "openrouter".to_string(),
            endpoint: format!("{}/chat/completions", API_BASE),
            status: 500,
            message: e.to_string(),
        })?;

    let status = response.status();
    if !status.is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(AiNeedError::ApiError {
            provider: "openrouter".to_string(),
            endpoint: format!("{}/chat/completions", API_BASE),
            status: status.as_u16(),
            message: error_text,
        });
    }

    let stream = response.bytes_stream().map(move |chunk| {
        chunk
            .map_err(|e| AiNeedError::ApiError {
                provider: "openrouter".to_string(),
                endpoint: format!("{}/chat/completions", API_BASE),
                status: 500,
                message: e.to_string(),
            })
            .and_then(|bytes| {
                let text = String::from_utf8_lossy(&bytes).to_string();
                for line in text.lines() {
                    if line.starts_with("data: ") {
                        let json_str = line.trim_start_matches("data: ");
                        if json_str == "[DONE]" {
                            return Ok("".to_string());
                        }
                        if let Ok(json) = serde_json::from_str::<serde_json::Value>(json_str) {
                            if let Some(content) = json["choices"][0]["delta"]["content"].as_str() {
                                return Ok(content.to_string());
                            }
                        }
                    }
                }
                Ok("".to_string())
            })
    });

    Ok(Box::pin(stream))
} 