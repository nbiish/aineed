use std::fs;
use std::path::PathBuf;
use std::io;
use crate::error::AiNeedError;

const CONFIG_DIR: &str = ".aineed";
const OPENAI_KEY_FILE: &str = "openai.key";
const ANTHROPIC_KEY_FILE: &str = "anthropic.key";
const TOGETHERAI_KEY_FILE: &str = "togetherai.key";
const OPENROUTER_KEY_FILE: &str = "openrouter.key";
const DEFAULT_MODEL_FILE: &str = "default_model";

fn get_config_dir() -> PathBuf {
    let home = dirs::home_dir().expect("Could not find home directory");
    home.join(CONFIG_DIR)
}

fn ensure_config_dir() -> io::Result<()> {
    let config_dir = get_config_dir();
    if !config_dir.exists() {
        fs::create_dir_all(&config_dir)?;
    }
    Ok(())
}

pub fn set_openai_key(key: &str) -> Result<(), AiNeedError> {
    ensure_config_dir()?;
    let key_path = get_config_dir().join(OPENAI_KEY_FILE);
    fs::write(key_path, key)?;
    Ok(())
}

pub fn set_anthropic_key(key: &str) -> Result<(), AiNeedError> {
    ensure_config_dir()?;
    let key_path = get_config_dir().join(ANTHROPIC_KEY_FILE);
    fs::write(key_path, key)?;
    Ok(())
}

pub fn set_togetherai_key(key: &str) -> Result<(), AiNeedError> {
    ensure_config_dir()?;
    let key_path = get_config_dir().join(TOGETHERAI_KEY_FILE);
    fs::write(key_path, key)?;
    Ok(())
}

pub fn set_openrouter_key(key: &str) -> Result<(), AiNeedError> {
    ensure_config_dir()?;
    let key_path = get_config_dir().join(OPENROUTER_KEY_FILE);
    fs::write(key_path, key)?;
    Ok(())
}

pub fn get_openai_key() -> Result<String, AiNeedError> {
    let key_path = get_config_dir().join(OPENAI_KEY_FILE);
    match fs::read_to_string(key_path) {
        Ok(key) => Ok(key.trim().to_string()),
        Err(_) => Err(AiNeedError::MissingApiKey {
            provider: "openai".to_string(),
            endpoint: "https://api.openai.com/v1".to_string(),
            quota_reset: None,
        })
    }
}

pub fn get_anthropic_key() -> Result<String, AiNeedError> {
    let key_path = get_config_dir().join(ANTHROPIC_KEY_FILE);
    match fs::read_to_string(key_path) {
        Ok(key) => Ok(key.trim().to_string()),
        Err(_) => Err(AiNeedError::MissingApiKey {
            provider: "anthropic".to_string(),
            endpoint: "https://api.anthropic.com/v1".to_string(),
            quota_reset: None,
        })
    }
}

pub fn get_togetherai_key() -> Result<String, AiNeedError> {
    let key_path = get_config_dir().join(TOGETHERAI_KEY_FILE);
    match fs::read_to_string(key_path) {
        Ok(key) => Ok(key.trim().to_string()),
        Err(_) => Err(AiNeedError::MissingApiKey {
            provider: "togetherai".to_string(),
            endpoint: "https://api.together.xyz/v1".to_string(),
            quota_reset: None,
        })
    }
}

pub fn get_openrouter_key() -> Result<String, AiNeedError> {
    let key_path = get_config_dir().join(OPENROUTER_KEY_FILE);
    match fs::read_to_string(key_path) {
        Ok(key) => Ok(key.trim().to_string()),
        Err(_) => Err(AiNeedError::MissingApiKey {
            provider: "openrouter".to_string(),
            endpoint: "https://openrouter.ai/api/v1".to_string(),
            quota_reset: None,
        })
    }
}

pub fn set_default_model(model: &str) -> Result<(), AiNeedError> {
    ensure_config_dir()?;
    let model_path = get_config_dir().join(DEFAULT_MODEL_FILE);
    fs::write(model_path, model)?;
    Ok(())
}

pub fn get_default_model() -> Result<String, AiNeedError> {
    let model_path = get_config_dir().join(DEFAULT_MODEL_FILE);
    match fs::read_to_string(model_path) {
        Ok(model) => Ok(model.trim().to_string()),
        Err(_) => Err(AiNeedError::ConfigError("No default model set".to_string()))
    }
}
