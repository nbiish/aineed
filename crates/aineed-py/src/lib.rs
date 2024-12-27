use pyo3::prelude::*;
use aineed_core::{config, providers};

/// Python module for aineed
#[pymodule]
fn aineed(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(set_openai_key, m)?)?;
    m.add_function(wrap_pyfunction!(set_anthropic_key, m)?)?;
    m.add_function(wrap_pyfunction!(set_togetherai_key, m)?)?;
    m.add_function(wrap_pyfunction!(set_openrouter_key, m)?)?;
    m.add_function(wrap_pyfunction!(generate_completion, m)?)?;
    m.add_function(wrap_pyfunction!(generate_image, m)?)?;
    Ok(())
}

/// Set OpenAI API key
#[pyfunction]
fn set_openai_key(key: String) -> PyResult<()> {
    config::set_openai_key(&key).map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}

/// Set Anthropic API key
#[pyfunction]
fn set_anthropic_key(key: String) -> PyResult<()> {
    config::set_anthropic_key(&key).map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}

/// Set TogetherAI API key
#[pyfunction]
fn set_togetherai_key(key: String) -> PyResult<()> {
    config::set_togetherai_key(&key).map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}

/// Set OpenRouter API key
#[pyfunction]
fn set_openrouter_key(key: String) -> PyResult<()> {
    config::set_openrouter_key(&key).map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}

/// Generate text completion
#[pyfunction]
fn generate_completion(
    provider: String,
    model: String,
    prompt: String,
    max_tokens: Option<u32>,
    temperature: Option<f32>,
) -> PyResult<String> {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        providers::generate_completion(
            &provider,
            &model,
            &prompt,
            max_tokens.unwrap_or(2048),
            temperature.unwrap_or(0.43),
        )
        .await
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
    })
}

/// Generate image
#[pyfunction]
fn generate_image(
    provider: String,
    model: String,
    prompt: String,
    output_path: Option<String>,
) -> PyResult<String> {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        providers::generate_image(
            &provider,
            &model,
            &prompt,
            output_path.as_deref(),
        )
        .await
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
    })
} 