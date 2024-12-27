# Create a Message

Send a structured list of input messages with text and/or image content, and the model will generate the next message in the conversation.

The Messages API can be used for either single queries or stateless multi-turn conversations.

## Endpoint
```
POST /v1/messages
```

## Headers

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `anthropic-beta` | `string[]` | Optional | Header to specify beta version(s). Use comma separated list like `beta1,beta2` or multiple headers. |
| `anthropic-version` | `string` | Required | The version of the Anthropic API to use. [Read more about versioning](). |
| `x-api-key` | `string` | Required | Your API key for authentication. Get it from the Console. Scoped to a Workspace. |

## Request Body

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `model` | `string` | Required | The model to use. See [models]() for details. |
| `messages` | `object[]` | Required | Array of message objects with `role` and `content`. |
| `max_tokens` | `integer` | Required | Maximum tokens to generate. Models may stop before this limit. |
| `metadata` | `object` | Optional | Request metadata. |
| `stop_sequences` | `string[]` | Optional | Custom sequences that will stop generation. |
| `stream` | `boolean` | Optional | Whether to stream the response. |
| `system` | `string` | Optional | System prompt for context/instructions. |
| `temperature` | `number` | Optional | Randomness (0.0-1.0). Default 1.0. |
| `tool_choice` | `object` | Optional | How model should use provided tools. |
| `tools` | `object[]` | Optional | Tool definitions model may use. |
| `top_k` | `integer` | Optional | Sample from top K options per token. |
| `top_p` | `number` | Optional | Nucleus sampling threshold. |

### Message Format Examples

Basic user message:
```json
[{"role": "user", "content": "Hello, Claude"}]
```

Multiple turns:
```json
[
    {"role": "user", "content": "Hello there."},
    {"role": "assistant", "content": "Hi, I'm Claude. How can I help you?"},
    {"role": "user", "content": "Can you explain LLMs in plain English?"}
]
```

With images (Claude 3):
```json
{
    "role": "user", 
    "content": [
        {
            "type": "image",
            "source": {
                "type": "base64",
                "media_type": "image/jpeg", 
                "data": "/9j/4AAQSkZJRg..."
            }
        },
        {"type": "text", "text": "What is in this image?"}
    ]
}
```

## Response

```typescript
{
    id: string;
    type: "message";
    role: "assistant";
    content: ContentBlock[];
    model: string;
    stop_reason: "end_turn" | "max_tokens" | "stop_sequence" | "tool_use" | null;
    stop_sequence: string | null;
    usage: {
        input_tokens: number;
        output_tokens: number;
        cache_creation_input_tokens: number | null;
        cache_read_input_tokens: number | null;
    }
}
```
### javascript request EXAMPLE
```javascript
import { Anthropic } from '@anthropic-ai/sdk';

const anthropic = new Anthropic();

await anthropic.messages.create({
  model: "claude-3-5-sonnet-20241022",
  max_tokens: 1024,
  messages: [
    {"role": "user", "content": "Hello, world"}
  ]
});
```
#### response
```json
{
  "type": "error",
  "error": {
    "type": "invalid_request_error",
    "message": "<string>"
  }
}
```
### python request EXAMPLE
```python
import anthropic

anthropic.Anthropic().messages.create(
    model="claude-3-5-sonnet-20241022",
    max_tokens=1024,
    messages=[
        {"role": "user", "content": "Hello, world"}
    ]
)
```
#### response
```json
{
  "type": "error",
  "error": {
    "type": "invalid_request_error",
    "message": "<string>"
  }
}
```