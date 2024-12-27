use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about = "AI assistant CLI tool")]
#[command(override_usage = "aineed [PROVIDER_MODEL] [OPTIONS]")]
#[command(after_help = "Examples:
    aineed -p \"Cyberpunk Nanaboozhoo\" -t 234 -o cyberpunk-nanaboozhoo.txt
    aineed openai:gpt-3.5-turbo -p \"who are the Anishinaabe?\"
    aineed openrouter:google/gemini-exp-1206:free -p \"who are the Anishinaabe?\" -o Anishinaabe.txt
    aineed openai:dall-e-3 -i -p \"Cyberpunk Nanaboozhoo\"
    aineed togetherai:black-forest-labs/FLUX.1-schnell -i -p \"Cyberpunk Nanaboozhoo\" -o cyberpunk-nanaboozhoo.png
    aineed anthropic:claude-3-5-sonnet-latest -p \"Give me a silly Nanaboozhoo story\" -o silly-nanaboozhoo-story.txt
    aineed openai:gpt-4o-turbo -s -p \"Tell me a story about Nanaboozhoo and the rabbits\"
    cat story.txt | aineed openai:gpt-3.5-turbo -s -t 100 -p \"summarize this\" > summary.txt
    aineed anthropic:claude-3-5-sonnet-latest -f story.txt -p \"summarize this\" > summary.txt
    aineed togetherai:meta-llama/Llama-3.3-70b-instruct-turbo -p \"Tell me a story about Nanaboozhoo and the rabbits\" -s -o nanaboozhoo-and-the-rabbits.txt
    aineed openrouter:google/gemini-2.0-flash-exp --temp 0.9 -t 999 -p \"Tell me a Nanaboozhoo fantasy adventure\" -o nanaboozhoo-fantasy-adventure.txt")]
pub struct Cli {
    /// Provider and model in format provider:model (default: togetherai:meta-llama/Llama-3.3-70b-instruct-turbo)
    /// Examples: openai:gpt-3.5-turbo, anthropic:claude-3.5-sonnet, togetherai:llama-2-70b, openrouter:openai/gpt-3.5-turbo
    #[arg(value_name = "PROVIDER_MODEL", default_value = "togetherai:meta-llama/Llama-3.3-70b-instruct-turbo")]
    pub provider_model: String,

    /// Prompt text for generation (default: "Who are the Anishinaabe?")
    #[arg(short = 'p', long = "prompt", value_name = "PROMPT")]
    pub prompt: Option<String>,

    /// Set provider:model as the new default model
    #[arg(short = 'd', long = "default")]
    pub default_model: Option<String>,

    /// Stream output (default: false)
    #[arg(short = 's', long = "stream")]
    pub stream: bool,

    /// Max tokens for the model (default: 2048)
    #[arg(short = 't', long = "tokens", default_value = "2048")]
    pub max_tokens: u32,

    /// Temperature for the model (default: 0.43, must be between 0 and 2)
    #[arg(long = "temp", default_value = "0.43")]
    pub temperature: f32,

    /// Path to the file to read input from (default: none)
    #[arg(short = 'f', long = "file", value_name = "FILE_PATH")]
    pub file: Option<String>,

    /// Generate image instead of text (default: false)
    #[arg(short = 'i', long = "image")]
    pub image: bool,

    /// Output path (default: none, for images: {model}_{timestamp}.png)
    #[arg(short = 'o', long = "output", value_name = "OUTPUT_PATH")]
    pub output: Option<String>,

    /// Set OpenAI API key
    #[arg(long = "set-openai", value_name = "API_KEY")]
    pub openai_key: Option<String>,

    /// Set Anthropic API key
    #[arg(long = "set-anthropic", value_name = "API_KEY")]
    pub anthropic_key: Option<String>,

    /// Set TogetherAI API key
    #[arg(long = "set-togetherai", value_name = "API_KEY")]
    pub togetherai_key: Option<String>,

    /// Set OpenRouter API key
    #[arg(long = "set-openrouter", value_name = "API_KEY")]
    pub openrouter_key: Option<String>,

    /// Show extra message ("Giin Inna Nanaboozhoo? Help me find Gichi-Waabooz")
    #[arg(short = 'x', long = "extra")]
    pub extra: bool,
}

impl Cli {
    pub fn is_image_generation(&self) -> bool {
        self.image
    }
}
