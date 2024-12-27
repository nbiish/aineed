![aineed icon](aineed_icon.png)

# aineed

A minimal CLI tool for interacting with multiple AI providers (OpenAI, Anthropic, TogetherAI, OpenRouter) with a unified interface.

## Features

- 🤖 Multiple AI Provider Support
  - OpenAI (text and image generation with DALL-E 3)
  - Anthropic (Claude models)
  - TogetherAI (Llama and FLUX models)
  - OpenRouter (various models)
- 🔄 Streaming Support for Text Generation
- 📁 File Input/Output with Prompt Prefixing
- 🎨 Image Generation with Automatic Timestamped Filenames
- 🔒 Local API Key Management

## Installation

### From Source

```bash
git clone https://github.com/Nbiish/aineed
cd aineed
cargo build --release
```

The binary will be available at `target/release/aineed`

## Configuration

Set your API keys:

```bash
# OpenAI
aineed --set-openai "your-api-key"

# Anthropic
aineed --set-anthropic "your-api-key"

# TogetherAI
aineed --set-togetherai "your-api-key"

# OpenRouter
aineed --set-openrouter "your-api-key"
```

## Usage

### Basic Text Generation

```bash
# Default model (togetherai:meta-llama/Llama-3.3-70b-instruct-turbo)
aineed -p "Who are the Anishinaabe?"

# Using OpenAI
aineed openai:gpt-3.5-turbo -p "who are the Anishinaabe?"

# Using OpenRouter
aineed openrouter:google/gemini-exp-1206:free -p "who are the Anishinaabe?" -o Anishinaabe.txt
```

### Image Generation

```bash
# Using OpenAI DALL-E 3 (saves as dall-e-3_TIMESTAMP.png)
aineed openai:dall-e-3 -i -p "Cyberpunk Nanaboozhoo"

# Using TogetherAI FLUX (with custom output name)
aineed togetherai:black-forest-labs/FLUX.1-schnell -i -p "Cyberpunk Nanaboozhoo" -o cyberpunk-nanaboozhoo.png
```

### File Processing with Prompts

When using a file input with `-f`, you can provide a prompt with `-p` that will be prepended to the file content:

```bash
# Process a file with a specific instruction
aineed openai:gpt-4o -f story.txt -p "Take significant text and make it hashtags" -o story_optimized.txt
```

The API request will be formatted like so when using `-p` and `-f`:
```
USER PROMPT:
<your prompt>

FILE CONTENT:
<file content>
```

### Streaming and Token Control

```bash
# Stream the response
aineed openai:gpt-4o-turbo -s -p "Tell me a story about Nanaboozhoo and the rabbits"

# Control max tokens and temperature
aineed openrouter:google/gemini-2.0-flash-exp --temp 0.9 -t 999 -p "Tell me a Nanaboozhoo fantasy adventure"
```

### Setting Default Model

```bash
# Set a new default model
aineed -d "togetherai:meta-llama/Llama-3.3-70b-instruct-turbo"
```

## Error Handling

The tool provides detailed error messages from providers to help troubleshoot:
- API key issues
- Rate limiting
- Connection problems
- Model access restrictions
- File I/O errors

## License

MIT License - see [LICENSE](LICENSE) for details
