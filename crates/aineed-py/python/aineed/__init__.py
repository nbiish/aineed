"""
aineed - AI assistant CLI tool for multiple providers
"""

from .aineed import (
    set_openai_key,
    set_anthropic_key,
    set_togetherai_key,
    set_openrouter_key,
    generate_completion,
    generate_image,
)

__version__ = "0.1.1" 