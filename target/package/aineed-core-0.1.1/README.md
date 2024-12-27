<div align="center">
  <img src="aineed_icon.png" width="50%">
</div>

# aineed

`aineed` is a versatile AI assistant tool that provides a unified interface for interacting with multiple AI providers, including OpenAI, Anthropic, TogetherAI, and OpenRouter. It's available as both a Rust crate and a Python package, making it easy to integrate into your projects regardless of your preferred language.

## Features

-   **Multiple AI Providers**: Supports a wide range of AI models:
    -   OpenAI (GPT-3.5, GPT-4, DALL-E)
    -   Anthropic (Claude)
    -   TogetherAI (Llama, FLUX)
    -   OpenRouter (various models)
-   **Text Generation**: Create compelling content, generate code, or get answers to your questions with powerful language models.
    -   Streaming support for real-time text generation.
-   **Image Generation**: Turn your ideas into stunning visuals with state-of-the-art image generation models.
-   **File Processing**: Seamlessly process files as input and output for your AI tasks.
-   **Cross-Platform**: Use `aineed` as a command-line tool or integrate it into your Python scripts.
-   **Configurable**: Customize parameters like token limits and temperature to fine-tune the AI's output.

    ## Coming Soon

    We are excited to announce upcoming features for `aineed`:

    - **Google API Integration**: Seamlessly integrate with Google services to enhance your AI capabilities.
    - **Image and Video Input**: Support for processing and generating image and video content.
    - **Voice Input/Output (IO)**: Enable voice interactions for a more natural user experience.
    - **Real-Time Collaboration**: Collaborate with team members in real-time using `aineed`.
    - **Advanced Customization Options**: More flexible configurations to tailor `aineed` to your specific needs.

    Stay tuned for these and more updates in future releases!
    ## Support

    If you find `aineed` helpful,  
    consider supporting its development!  
    I'm a full-time student and this is a side project:

    - **CashApp**: [$Nbiish](https://cash.app/$Nbiish)
    - **Venmo**: [@Nbiish](https://venmo.com/Nbiish)
    - **Support Content**: [linktr.ee/nbiish](https://linktr.ee/nbiish)

    Thank you for your support!


## Installation

### Using Cargo (Rust)

To install `aineed` as a Rust crate, use Cargo:

```bash
cargo install aineed
```

This will download and compile `aineed` from crates.io and add it to your Cargo bin directory.

### Using pip (Python)

To install `aineed` as a Python package, use pip:

```bash
pip install aineed
```

This will download and install `aineed` from PyPI along with its dependencies.

## Usage

### CLI Usage

#### Setting API Keys

Before using `aineed`, you need to set your API keys for the providers you want to use:

```bash
aineed --set-openai "your-openai-key-here"
aineed --set-anthropic "your-anthropic-key-here"
aineed --set-togetherai "your-togetherai-key-here"
aineed --set-openrouter "your-openrouter-key-here"
```

#### Text Generation

Generate text using various models:

```bash
# OpenAI
aineed openai:gpt-3.5-turbo -p "Who are the Anishinaabe?"

# Anthropic
aineed anthropic:claude-3-sonnet -p "Tell me a story" -o story.txt

# TogetherAI
aineed togetherai:llama-2-70b -p "Explain quantum physics" -s

# OpenRouter
aineed openrouter:mistralai/mistral-7b-instruct -p "What is the capital of France?"
```

#### Image Generation

Create images from text prompts:

```bash
# OpenAI DALL-E 3
aineed openai:dall-e-3 -i -p "A cyberpunk scene with neon lights and flying cars"

# TogetherAI FLUX
aineed togetherai:black-forest-labs/FLUX.1-schnell -i -p "A fantasy landscape with a dragon and a castle"
```

#### File Input/Output

Process files with `aineed`:

```bash
# Summarize a file using OpenAI GPT-4
aineed openai:gpt-4 -f input.txt -p "Summarize this" > summary.txt

# Analyze text from stdin using Anthropic Claude 3
cat input.txt | aineed anthropic:claude-3 -p "Analyze this text"
```

### Python Usage

#### Setting API Keys

```python
from aineed import Client

# Set API key for OpenAI
Client.set_api_key("openai", "your-openai-key-here")
```

#### Text Generation

```python
from aineed import Client

# Create a client for OpenAI GPT-3.5 Turbo
client = Client("openai:gpt-4o-mini")

# Generate text
async def main():
    text = await client.generate("Tell me a story about a robot Nanaboozhoo doing magic.")
    print(text)

if __name__ == "__main__":
    import asyncio
    asyncio.run(main())
```

#### Image Generation

```python
from aineed import Client

# Create a client for OpenAI DALL-E 3
client = Client("openai:dall-e-3")

# Generate an image
async def main():
    image_path = await client.generate_image("A futuristic Anishinaabe cityscape at night", "output.png")
    print(f"Image saved to: {image_path}")

if __name__ == "__main__":
    import asyncio
    asyncio.run(main())
```
#### CLI Usage from Python

```python
import os
os.system("aineed openai:gpt-4o-mini -p 'Who are the Anishinaabe?'")
```

## Configuration

You can configure `aineed` to use default providers and models, so you don't have to specify them every time.

### Setting Default Provider

To set a default provider, use the `--set-default-provider` flag:

```bash
aineed --set-default-provider openai
```

### Setting Default Model

To set a default model for a provider, use the `--set-default-model` flag:

```bash
aineed --set-default-model openai gpt-4
```

Now, when you run `aineed` without specifying a provider or model, it will use the defaults you've set.

## Contributing

Contributions to `aineed` are welcome! If you'd like to contribute, please follow these steps:

1. Fork the repository on GitHub.
2. Create a new branch for your feature or bug fix.
3. Make your changes and commit them with clear, descriptive messages.
4. Push your branch to your forked repository.
5. Submit a pull request to the main `aineed` repository.

Please ensure your code follows the project's coding style and includes appropriate tests.

## License

`aineed` is released under the MIT License. See the `LICENSE` file for more details.

**Note:** Will not contain all available models from providers as they change often
    
```bash
aineed -p "Cyberpunk Nanaboozhoo" -t 234 -o cyberpunk-nanaboozhoo.txt

aineed openai:gpt-3.5-turbo -p "who are the Anishinaabe?"

aineed openrouter:google/gemini-exp-1206:free -p "who are the Anishinaabe?" -o Anishinaabe.txt

aineed openai:dall-e-3 -i -p "Cyberpunk Nanaboozhoo"

aineed togetherai:black-forest-labs/FLUX.1-schnell -i -p "Cyberpunk Nanaboozhoo" -o cyberpunk-nanaboozhoo.png

aineed anthropic:claude-3-5-sonnet-latest -p "Give me a silly Nanaboozhoo story" -o silly-nanaboozhoo-story.txt

aineed openai:gpt-4o-turbo -s -p "Tell me a story about Nanaboozhoo and the rabbits"

cat story.txt | aineed openai:gpt-3.5-turbo -s -t 100 -p "summarize this" > summary.txt

aineed anthropic:claude-3-5-sonnet-latest -f story.txt -p "summarize this" > summary.txt

aineed togetherai:meta-llama/Llama-3.3-70b-instruct-turbo -p "Tell me a story about Nanaboozhoo and the rabbits" -s -o nanaboozhoo-and-the-rabbits.txt

aineed openrouter:google/gemini-2.0-flash-exp --temp 0.9 -t 999 -p "Tell me a Nanaboozhoo fantasy adventure" -o nanaboozhoo-fantasy-adventure.txt
```