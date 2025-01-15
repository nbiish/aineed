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


### Prerequisites
- Python 3.7 or higher
- Rust toolchain (install from https://rustup.rs/)

### Using pip
```bash
# Install Rust first if you haven't already
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

```bash
pip install aineed
```

### Install without needing rust COMING SOON~


## Support

As a full-time student, your support would be greatly appreciated!

- **CashApp**: [\$Nbiish](https://cash.app/$Nbiish)
- **Venmo**: [@Nbiish](https://venmo.com/@Nbiish)
- **Linktree**: [https://linktr.ee/nbiish](https://linktr.ee/nbiish)


## Upcoming Features

We are excited to announce that future releases of **aineed** will include:

- **Google Models and Enhanced Capabilities**: Integrate with Google's extensive AI models to provide even more powerful and diverse functionalities.

- **Voice and Audio Input/Output**: Enable voice commands and audio responses for a more interactive and seamless user experience.

Stay tuned for these and more exciting updates!



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
aineed -d "openrouter:google/gemini-2.0-flash-exp"
```

### Additional Examples of Use

```bash
aineed openai:gpt-4o -t 444 -p "Short story about Nanaboozhoo" -o nanaboozhoo.txt
cat nanaboozhoo.txt | aineed openrouter:google/gemini-2.0-flash-exp -p "Rephrase the story to be extra silly" -o nanaboozhoo-silly.txt
aineed anthropic:claude-3.5-sonnet-latest -s -f nanaboozhoo-silly.txt -p "End every sentence with hashtags" -o nanaboozhoo-silly-hashtags.txt
```

#### Translating Text

Translate a sentence from English to Spanish using Anthropic's Claude model:

```bash
aineed anthropic:claude-3.5-sonnet-latest -p "Translate the following sentence to Spanish: 'Artificial intelligence is transforming the world.'"
```

#### Creating Detailed Images

Generate a high-resolution image of a futuristic cityscape with TogetherAI's FLUX model:

```bash
aineed togetherai:black-forest-labs/FLUX.1-schnell -i -p "Create a high-resolution image of Cyberpunk Nanaboozhoo" -o cyberpunk_nanaboozhoo.png
```

#### Automating File Processing

```bash
for file in reports/*.txt; do
    aineed openrouter:google/gemini-2.0-flash-exp -f "$file" -p "Summarize the key points of the following report." -o "summaries/$(basename "$file")"
done
```

#### Streaming Long-Form Content

Stream the generation of a comprehensive guide on machine learning with controlled token usage:

```bash
aineed openai:gpt-4o-mini -s -t 1500 -p "Write a comprehensive guide on machine learning, including definitions, algorithms, and applications."
```

#### Setting and Using Default Models

Set a default model and generate content without specifying the provider each time:

```bash
# Set the default model to OpenAI's GPT-4
aineed -d "openai:gpt-4o-mini"

# Now generate content using the default model
aineed -p "Describe the impact of climate change on global ecosystems and how indigenous people protect the majority of remaining healthy ecosystems." -o climate_change_impact.txt
```

#### Integrating with Other Tools

Use `aineed` in a shell script to automate the generation and processing of content:

```bash
#!/bin/bash

# Generate an article
aineed openai:gpt-4 -p "Write an article about the benefits of renewable energy." -o article.txt

# Summarize the article
aineed openai:gpt-4o -f article.txt -p "Summarize the main points of this article." -o summary.txt

# Log the actions
echo "Article and summary generated on $(date)" >> generation.log
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

# #LANDBACK
