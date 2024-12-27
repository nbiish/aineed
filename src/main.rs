use clap::Parser;
use futures_util::StreamExt;
use std::error::Error;
use std::io::{self, Write};
use tokio;

mod cli;
mod providers;
mod config;
mod error;

use cli::Cli;

// Default prompt as specified in the schema
const DEFAULT_PROMPT: &str = "Who are the Anishinaabe?";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // Parse CLI arguments with detailed error handling
    let mut cli = match Cli::try_parse() {
        Ok(cli) => cli,
        Err(e) => {
            eprintln!("Error: {}", e);
            return Ok(());
        }
    };

    // Show extra message if requested
    if cli.extra {
        println!("Giin Inna Nanaboozhoo? Help me find Gichi-Waabooz");
        return Ok(());
    }

    // Set default prompt if none provided
    let prompt = cli.prompt.get_or_insert_with(|| DEFAULT_PROMPT.to_string()).clone();

    // Handle API key setting
    if let Some(key) = cli.openai_key.as_ref() {
        config::set_openai_key(key)?;
        println!("OpenAI API key set successfully");
        return Ok(());
    }

    if let Some(key) = cli.anthropic_key.as_ref() {
        config::set_anthropic_key(key)?;
        println!("Anthropic API key set successfully");
        return Ok(());
    }

    if let Some(key) = cli.togetherai_key.as_ref() {
        config::set_togetherai_key(key)?;
        println!("TogetherAI API key set successfully");
        return Ok(());
    }

    if let Some(key) = cli.openrouter_key.as_ref() {
        config::set_openrouter_key(key)?;
        println!("OpenRouter API key set successfully");
        return Ok(());
    }

    // Set default model if provided
    if let Some(model) = cli.default_model.as_ref() {
        config::set_default_model(model)?;
        println!("Default model set to: {}", model);
        return Ok(());
    }

    // Get the model to use (either from args or default)
    let model = cli.provider_model.clone();

    // Validate the model format
    if !model.contains(':') {
        eprintln!("Validation Error: Invalid model format. Examples:");
        eprintln!("- openai:gpt-3.5-turbo");
        eprintln!("- anthropic:claude-3.5-sonnet");
        eprintln!("- togetherai:llama-2-70b");
        eprintln!("- openrouter:openai/gpt-3.5-turbo");
        return Ok(());
    }

    // Split model into provider and model name
    let (provider, model_name) = model.split_once(':').unwrap();

    // Get input from file if provided, format with clear sections
    let input = if let Some(file_path) = cli.file.as_ref() {
        let file_content = std::fs::read_to_string(file_path)?;
        if prompt != DEFAULT_PROMPT {
            // Format with clear sections for prompt and file content
            format!(
                "USER PROMPT:\n{}\n\nFILE CONTENT:\n{}", 
                prompt, 
                file_content
            )
        } else {
            // If using default prompt, just use file content
            file_content
        }
    } else {
        prompt
    };

    // Get output path
    let output_path = cli.output.as_deref();
    let is_stream = cli.stream;
    let max_tokens = cli.max_tokens;
    let temperature = cli.temperature;
    let is_image = cli.is_image_generation();

    // Handle image generation
    if is_image {
        let output_path = if output_path.is_none() {
            use chrono::Local;
            let timestamp = Local::now().format("%Y%m%d_%H%M%S");
            // Create a safe version of the model name
            let safe_model = model_name
                .chars()
                .map(|c| if c.is_alphanumeric() || c == '-' { c } else { '_' })
                .collect::<String>();
            format!("{}_{}.png", safe_model, timestamp)
        } else {
            output_path.unwrap().to_string()
        };
        let result = providers::generate_image(provider, model_name, &input, Some(&output_path)).await?;
        println!("Image saved to: {}", result);
        return Ok(());
    }

    // Handle text generation
    if is_stream {
        let mut stream = providers::stream_completion(
            provider,
            model_name,
            &input,
            max_tokens,
            temperature,
        ).await?;

        let mut stdout = io::stdout();
        while let Some(chunk) = stream.next().await {
            match chunk {
                Ok(text) => {
                    if !text.is_empty() {
                        stdout.write_all(text.as_bytes())?;
                        stdout.flush()?;
                    }
                }
                Err(e) => {
                    eprintln!("\nError during streaming: {}", e);
                    return Ok(());
                }
            }
        }
        println!(); // Add newline after streaming
    } else {
        let result = providers::generate_completion(
            provider,
            model_name,
            &input,
            max_tokens,
            temperature,
        ).await?;

        // Handle output
        if let Some(path) = output_path {
            std::fs::write(path, result)?;
            println!("Output saved to: {}", path);
        } else {
            println!("{}", result);
        }
    }

    Ok(())
}
