pub mod config;
pub mod error;
pub mod providers;

pub use error::AiNeedError;
pub type Result<T> = std::result::Result<T, AiNeedError>; 