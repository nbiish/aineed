# Chat
Given a list of messages comprising a conversation, the model will return a response. Related guide: Chat Completions

## Create chat completion

**POST** `https://api.openai.com/v1/chat/completions`

Creates a model response for the given chat conversation. Learn more in the text generation, vision, and audio guides.

### Request body

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| messages | array | Yes | A list of messages comprising the conversation so far. Supports different message types (text, images, audio) depending on the model. |
| model | string | Yes | ID of the model to use. See model endpoint compatibility table for Chat API details. |
| store | boolean | No | Whether to store output for model distillation or evals (default: false). |
| metadata | object | No | Developer-defined tags for filtering completions in dashboard. |
| frequency_penalty | number | No | Value between -2.0 and 2.0 to penalize token frequency (default: 0). |
| logit_bias | map | No | Modifies likelihood of specified tokens (-100 to 100). |
| logprobs | boolean | No | Returns log probabilities of output tokens if true (default: false). |
| top_logprobs | integer | No | Number of most likely tokens to return (0-20). Requires logprobs=true. |
| max_tokens | integer | No | **Deprecated** Maximum tokens for chat completion. Use max_completion_tokens instead. |
| max_completion_tokens | integer | No | Upper bound for generated tokens. |
| n | integer | No | Number of chat completion choices to generate (default: 1). |
| presence_penalty | number | No | Value between -2.0 and 2.0 to penalize token presence (default: 0). |
| response_format | object | No | Specifies output format. Supports JSON schema and JSON object types. |
| seed | integer | No | For deterministic sampling (Beta). |
| stop | string/array | No | Up to 4 sequences where token generation stops. |
| stream | boolean | No | Enables partial message deltas (default: false). |
| temperature | number | No | Sampling temperature between 0 and 2 (default: 1). |
| top_p | number | No | Nucleus sampling probability mass (default: 1). |
| tools | array | No | List of tools (functions) the model may call. Max 128 functions. |
| user | string | No | Unique end-user identifier for abuse monitoring. |

### Returns
Returns a chat completion object, or streamed sequence of chat completion chunks if streaming is enabled.

### Examples

#### Node.js Example

```javascript
import OpenAI from "openai";

const openai = new OpenAI();

async function main() {
    const completion = await openai.chat.completions.create({
        messages: [{ role: "system", content: "You are a helpful assistant." }],
        model: "gpt-4o-mini",
    });

    console.log(completion.choices[0]);
}

main();
```

##### Response

```json
{
    "id": "chatcmpl-123",
    "object": "chat.completion",
    "created": 1677652288,
    "model": "gpt-4o-mini",
    "system_fingerprint": "fp_44709d6fcb",
    "choices": [{
        "index": 0,
        "message": {
            "role": "assistant",
            "content": "\n\nHello there, how may I assist you today?"
        },
        "logprobs": null,
        "finish_reason": "stop"
    }],
    "usage": {
        "prompt_tokens": 9,
        "completion_tokens": 12,
        "total_tokens": 21,
        "completion_tokens_details": {
            "reasoning_tokens": 0,
            "accepted_prediction_tokens": 0,
            "rejected_prediction_tokens": 0
        }
    }
}
```

#### Python Example

```python
from openai import OpenAI
client = OpenAI()

completion = client.chat.completions.create(
    model="gpt-4o-mini",
    messages=[
        {"role": "system", "content": "You are a helpful assistant."},
        {"role": "user", "content": "Hello!"}
    ]
)

print(completion.choices[0].message)
```

##### Response

```json
{
    "id": "chatcmpl-123",
    "object": "chat.completion",
    "created": 1677652288,
    "model": "gpt-4o-mini",
    "system_fingerprint": "fp_44709d6fcb",
    "choices": [{
        "index": 0,
        "message": {
            "role": "assistant",
            "content": "\n\nHello there, how may I assist you today?"
        },
        "logprobs": null,
        "finish_reason": "stop"
    }],
    "usage": {
        "prompt_tokens": 9,
        "completion_tokens": 12,
        "total_tokens": 21,
        "completion_tokens_details": {
            "reasoning_tokens": 0,
            "accepted_prediction_tokens": 0,
            "rejected_prediction_tokens": 0
        }
    }
}
```

### Response Object Properties

- **id** (string): A unique identifier for the chat completion.
- **choices** (array): A list of chat completion choices. Can be more than one if n is greater than 1.
- **created** (integer): The Unix timestamp (in seconds) of when the chat completion was created.
- **model** (string): The model used for the chat completion.
- **service_tier** (string or null): The service tier used for processing the request. Only included if specified in the request.
- **system_fingerprint** (string): Backend configuration fingerprint for determinism tracking.
- **object** (string): Always "chat.completion".
- **usage** (object): Usage statistics for the completion request.

#### OBJECT: The chat completion object
```json
{
  "id": "chatcmpl-123456",
  "object": "chat.completion",
  "created": 1728933352,
  "model": "gpt-4o-2024-08-06",
  "choices": [
    {
      "index": 0,
      "message": {
        "role": "assistant",
        "content": "Hi there! How can I assist you today?",
        "refusal": null
      },
      "logprobs": null,
      "finish_reason": "stop"
    }
  ],
  "usage": {
    "prompt_tokens": 19,
    "completion_tokens": 10,
    "total_tokens": 29,
    "prompt_tokens_details": {
      "cached_tokens": 0
    },
    "completion_tokens_details": {
      "reasoning_tokens": 0,
      "accepted_prediction_tokens": 0,
      "rejected_prediction_tokens": 0
    }
  },
  "system_fingerprint": "fp_6b68a8204b"
}
```
# Streaming

The OpenAI API supports streaming responses for partial results on certain requests. This follows the Server-sent events standard. Our official Node and Python libraries provide helpers for parsing these events.

Streaming is available for both Chat Completions and Assistants APIs. This section covers streaming for Chat Completions. For Assistants API streaming, see the dedicated documentation.

## Python Example

```python
from openai import OpenAI

client = OpenAI()

stream = client.chat.completions.create(
    model="gpt-4o-mini",
    messages=[{"role": "user", "content": "Say this is a test"}],
    stream=True,
)
for chunk in stream:
    if chunk.choices[0].delta.content is not None:
        print(chunk.choices[0].delta.content, end="")
```

## Node.js / TypeScript Example

```javascript
import OpenAI from "openai";

const openai = new OpenAI();

async function main() {
    const stream = await openai.chat.completions.create({
        model: "gpt-4o-mini",
        messages: [{ role: "user", content: "Say this is a test" }],
        stream: true,
    });
    for await (const chunk of stream) {
        process.stdout.write(chunk.choices[0]?.delta?.content || "");
    }
}

main();
```

## Parsing Server-sent Events

Parsing Server-sent events requires careful handling. Simple methods like splitting by newlines can lead to errors. We strongly recommend using existing client libraries for reliable event parsing.