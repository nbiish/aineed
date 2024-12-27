use std::fmt;
use std::io;
use reqwest::StatusCode;

#[derive(Debug)]
pub enum AiNeedError {
    IoError(io::Error),
    ConfigError(String),
    MissingApiKey {
        provider: String,
        endpoint: String,
        quota_reset: Option<String>,
    },
    InvalidApiKey {
        provider: String,
        endpoint: String,
        quota_reset: Option<String>,
    },
    QuotaExceeded {
        provider: String,
        endpoint: String,
        quota_reset: Option<String>,
    },
    UnsupportedModel {
        provider: String,
        model: String,
        supported_models: Vec<String>,
    },
    ApiError {
        provider: String,
        endpoint: String,
        status: u16,
        message: String,
    },
    RequestError {
        status: StatusCode,
        message: String,
        details: Option<String>,
    },
}

impl fmt::Display for AiNeedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AiNeedError::IoError(err) => write!(f, "IO error: {}", err),
            AiNeedError::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
            AiNeedError::MissingApiKey { provider, endpoint, quota_reset } => {
                write!(f, "Missing API key for {} ({})", provider, endpoint)?;
                if let Some(reset) = quota_reset {
                    write!(f, " - Quota resets at {}", reset)?;
                }
                Ok(())
            },
            AiNeedError::InvalidApiKey { provider, endpoint, quota_reset } => {
                write!(f, "Invalid API key for {} ({})", provider, endpoint)?;
                if let Some(reset) = quota_reset {
                    write!(f, " - Quota resets at {}", reset)?;
                }
                Ok(())
            },
            AiNeedError::QuotaExceeded { provider, endpoint, quota_reset } => {
                write!(f, "Quota exceeded for {} ({})", provider, endpoint)?;
                if let Some(reset) = quota_reset {
                    write!(f, " - Quota resets at {}", reset)?;
                }
                Ok(())
            },
            AiNeedError::UnsupportedModel { provider, model, supported_models } => {
                write!(f, "Unsupported model '{}' for provider {}. Supported models: {}", 
                    model, provider, supported_models.join(", "))
            },
            AiNeedError::ApiError { provider, endpoint, status, message } => {
                write!(f, "API error from {} ({}): {} - {}", provider, endpoint, status, message)
            },
            AiNeedError::RequestError { status, message, details } => {
                if let Some(detail) = details {
                    write!(f, "Request error ({}): {} - {}", status, message, detail)
                } else {
                    write!(f, "Request error ({}): {}", status, message)
                }
            },
        }
    }
}

impl std::error::Error for AiNeedError {}

impl From<io::Error> for AiNeedError {
    fn from(err: io::Error) -> Self {
        AiNeedError::IoError(err)
    }
}
