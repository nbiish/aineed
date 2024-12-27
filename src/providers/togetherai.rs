use reqwest::Client;
use futures_util::{Stream, StreamExt};
use std::pin::Pin;
use crate::config;
use crate::error::AiNeedError;
use serde_json::json;
use tokio::fs;

const API_BASE: &str = "https://api.together.xyz/v1";

pub async fn generate_completion(
    model: &str,
    prompt: &str,
    max_tokens: u32,
    temperature: f32,
) -> Result<String, AiNeedError> {
    let client = Client::new();
    let api_key = config::get_togetherai_key()?;

    let response = client
        .post(format!("{}/chat/completions", API_BASE))
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&json!({
            "model": model,
            "messages": [{"role": "user", "content": prompt}],
            "max_tokens": max_tokens,
            "temperature": temperature,
        }))
        .send()
        .await
        .map_err(|e| AiNeedError::ApiError {
            provider: "togetherai".to_string(),
            endpoint: format!("{}/chat/completions", API_BASE),
            status: 500,
            message: e.to_string(),
        })?;

    let status = response.status();
    if !status.is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(AiNeedError::ApiError {
            provider: "togetherai".to_string(),
            endpoint: format!("{}/chat/completions", API_BASE),
            status: status.as_u16(),
            message: error_text,
        });
    }

    let json: serde_json::Value = response.json().await.map_err(|e| AiNeedError::ApiError {
        provider: "togetherai".to_string(),
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
    let api_key = config::get_togetherai_key()?;

    let response = client
        .post(format!("{}/chat/completions", API_BASE))
        .header("Authorization", format!("Bearer {}", api_key))
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
            provider: "togetherai".to_string(),
            endpoint: format!("{}/chat/completions", API_BASE),
            status: 500,
            message: e.to_string(),
        })?;

    let status = response.status();
    if !status.is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(AiNeedError::ApiError {
            provider: "togetherai".to_string(),
            endpoint: format!("{}/chat/completions", API_BASE),
            status: status.as_u16(),
            message: error_text,
        });
    }

    let stream = response.bytes_stream().map(move |chunk| {
        chunk
            .map_err(|e| AiNeedError::ApiError {
                provider: "togetherai".to_string(),
                endpoint: format!("{}/chat/completions", API_BASE),
                status: 500,
                message: e.to_string(),
            })
            .and_then(|bytes| {
                let text = String::from_utf8_lossy(&bytes);
                let mut result = String::new();
                
                for line in text.lines() {
                    if line.starts_with("data: ") {
                        let json_str = line.trim_start_matches("data: ");
                        if json_str == "[DONE]" {
                            continue;
                        }
                        if let Ok(json) = serde_json::from_str::<serde_json::Value>(json_str) {
                            if let Some(content) = json["choices"][0]["delta"]["content"].as_str() {
                                result.push_str(content);
                            }
                        }
                    }
                }
                
                Ok(result)
            })
    });

    Ok(Box::pin(stream))
}

pub async fn generate_image(
    model: &str,
    prompt: &str,
    output_path: Option<&str>,
) -> Result<String, AiNeedError> {
    let client = Client::new();
    let api_key = config::get_togetherai_key()?;

    let response = client
        .post(format!("{}/images/generations", API_BASE))
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&json!({
            "model": model,
            "prompt": prompt,
            "steps": 10,
            "n": 1
        }))
        .send()
        .await
        .map_err(|e| AiNeedError::ApiError {
            provider: "togetherai".to_string(),
            endpoint: format!("{}/images/generations", API_BASE),
            status: 500,
            message: e.to_string(),
        })?;

    let status = response.status();
    if !status.is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(AiNeedError::ApiError {
            provider: "togetherai".to_string(),
            endpoint: format!("{}/images/generations", API_BASE),
            status: status.as_u16(),
            message: error_text,
        });
    }

    let json: serde_json::Value = response.json().await.map_err(|e| AiNeedError::ApiError {
        provider: "togetherai".to_string(),
        endpoint: format!("{}/images/generations", API_BASE),
        status: 500,
        message: e.to_string(),
    })?;

    let image_url = json["data"][0]["url"]
        .as_str()
        .ok_or_else(|| AiNeedError::ApiError {
            provider: "togetherai".to_string(),
            endpoint: format!("{}/images/generations", API_BASE),
            status: 500,
            message: "No image URL in response".to_string(),
        })?;

    let filename = match output_path {
        Some(path) => path.to_string(),
        None => {
            // Extract model name without provider prefix
            let model_name = model.split('/').last().unwrap_or("togetherai");
            // Create a filename-safe version of the prompt
            let safe_prompt = prompt
                .chars()
                .map(|c| if c.is_alphanumeric() || c == ' ' { c } else { '_' })
                .collect::<String>()
                .replace(' ', "_")
                .to_lowercase();
            format!("{}_{}.png", model_name, safe_prompt)
        }
    };

    let image_bytes = client
        .get(image_url)
        .send()
        .await
        .map_err(|e| AiNeedError::ApiError {
            provider: "togetherai".to_string(),
            endpoint: image_url.to_string(),
            status: 500,
            message: e.to_string(),
        })?
        .bytes()
        .await
        .map_err(|e| AiNeedError::ApiError {
            provider: "togetherai".to_string(),
            endpoint: image_url.to_string(),
            status: 500,
            message: e.to_string(),
        })?;

    fs::write(&filename, image_bytes).await.map_err(|e| AiNeedError::IoError(e))?;
    Ok(filename)
}