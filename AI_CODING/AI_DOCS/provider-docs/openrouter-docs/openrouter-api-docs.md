# Quick Start

OpenRouter provides an OpenAI-compatible completion API to 281 models & providers that you can call directly, or using the OpenAI SDK. Additionally, some third-party SDKs are available.

In the examples below, the OpenRouter-specific headers are optional. Setting them allows your app to appear on the OpenRouter leaderboards.

## Using the OpenAI SDK

```typescript
import OpenAI from "openai"

const openai = new OpenAI({
  baseURL: "https://openrouter.ai/api/v1",
  apiKey: $OPENROUTER_API_KEY,
  defaultHeaders: {
    "HTTP-Referer": $YOUR_SITE_URL, // Optional, for including your app on openrouter.ai rankings.
    "X-Title": $YOUR_APP_NAME, // Optional. Shows in rankings on openrouter.ai.
  }
})

async function main() {
  const completion = await openai.chat.completions.create({
    model: "openai/gpt-3.5-turbo",
    messages: [
      {
        "role": "user",
        "content": "What is the meaning of life?"
      }
    ]
  })

  console.log(completion.choices[0].message)
}
main()
```

## Using the OpenRouter API directly

```typescript
fetch("https://openrouter.ai/api/v1/chat/completions", {
  method: "POST",
  headers: {
    "Authorization": `Bearer ${OPENROUTER_API_KEY}`,
    "HTTP-Referer": `${YOUR_SITE_URL}`, // Optional, for including your app on openrouter.ai rankings.
    "X-Title": `${YOUR_SITE_NAME}`, // Optional. Shows in rankings on openrouter.ai.
    "Content-Type": "application/json"
  },
  body: JSON.stringify({
    "model": "openai/gpt-3.5-turbo",
    "messages": [
      {
        "role": "user",
        "content": "What is the meaning of life?"
      }
    ]
  })
});
```

```python
from openai import OpenAI

client = OpenAI(
  base_url="https://openrouter.ai/api/v1",
  api_key="$OPENROUTER_API_KEY",
)

completion = client.chat.completions.create(
  extra_headers={
    "HTTP-Referer": $YOUR_SITE_URL, // Optional, for including your app on openrouter.ai rankings.
    "X-Title": $YOUR_APP_NAME, // Optional. Shows in rankings on openrouter.ai.
  },
  model="openai/gpt-3.5-turbo",
  messages=[
    {
      "role": "user",
      "content": "What is the meaning of life?"
    }
  ]
)
print(completion.choices[0].message.content)
```

## Using the OpenRouter API directly

```python
import requests
import json

response = requests.post(
  url="https://openrouter.ai/api/v1/chat/completions",
  headers={
    "Authorization": f"Bearer {OPENROUTER_API_KEY}",
    "HTTP-Referer": f"{YOUR_SITE_URL}", # Optional, for including your app on openrouter.ai rankings.
    "X-Title": f"{YOUR_APP_NAME}", # Optional. Shows in rankings on openrouter.ai.
  },
  data=json.dumps({
    "model": "openai/gpt-3.5-turbo", # Optional
    "messages": [
      {
        "role": "user",
        "content": "What is the meaning of life?"
      }
    ]
  })
)
```

# Requests

OpenRouter's request and response schemas are very similar to the OpenAI Chat API, with a few small differences. At a high level, OpenRouter normalizes the schema across models and providers so you only need to learn one.

## Request Body

Here is the request schema as a TypeScript type. This will be the body of your POST request to the /api/v1/chat/completions endpoint (see the quick start above for an example).

```typescript
// Definitions of subtypes are below
type Request = {
  // Either "messages" or "prompt" is required
  messages?: Message[];
  prompt?: string;

  // If "model" is unspecified, uses the user's default
  model?: string; // See "Supported Models" section

  // Allows to force the model to produce specific output format.
  // See models page and note on this docs page for which models support it.
  response_format?: { type: 'json_object' };

  stop?: string | string[];
  stream?: boolean; // Enable streaming

  // See LLM Parameters (openrouter.ai/docs/parameters)
  max_tokens?: number; // Range: [1, context_length)
  temperature?: number; // Range: [0, 2]

  // Tool calling
  // Will be passed down as-is for providers implementing OpenAI's interface.
  // For providers with custom interfaces, we transform and map the properties.
  // Otherwise, we transform the tools into a YAML template. The model responds with an assistant message.
  // See models supporting tool calling: openrouter.ai/models?supported_parameters=tools
  tools?: Tool[];
  tool_choice?: ToolChoice;

  // Advanced optional parameters
  seed?: number; // Integer only
  top_p?: number; // Range: (0, 1]
  top_k?: number; // Range: [1, Infinity) Not available for OpenAI models
  frequency_penalty?: number; // Range: [-2, 2]
  presence_penalty?: number; // Range: [-2, 2]
  repetition_penalty?: number; // Range: (0, 2]
  logit_bias?: { [key: number]: number };
  top_logprobs: number; // Integer only
  min_p?: number; // Range: [0, 1]
  top_a?: number; // Range: [0, 1]

  // Reduce latency by providing the model with a predicted output
  // https://platform.openai.com/docs/guides/latency-optimization#use-predicted-outputs
  prediction?: { type: 'content'; content: string; };

  // OpenRouter-only parameters
  // See "Prompt Transforms" section: openrouter.ai/docs/transforms
  transforms?: string[];
  // See "Model Routing" section: openrouter.ai/docs/model-routing
  models?: string[];
  route?: 'fallback';
  // See "Provider Routing" section: openrouter.ai/docs/provider-routing
  provider?: ProviderPreferences;
};

// Subtypes:

type TextContent = {
  type: 'text';
  text: string;
};

type ImageContentPart = {
  type: 'image_url';
  image_url: {
    url: string; // URL or base64 encoded image data
    detail?: string; // Optional, defaults to 'auto'
  };
};

type ContentPart = TextContent | ImageContentPart;

type Message =
  | {
      role: 'user' | 'assistant' | 'system';
      // ContentParts are only for the 'user' role:
      content: string | ContentPart[];
      // If "name" is included, it will be prepended like this
      // for non-OpenAI models: `{name}: {content}`
      name?: string;
    }
  | {
      role: 'tool';
      content: string;
      tool_call_id: string;
      name?: string;
    };

type FunctionDescription = {
  description?: string;
  name: string;
  parameters: object; // JSON Schema object
};

type Tool = {
  type: 'function';
  function: FunctionDescription;
};

type ToolChoice =
  | 'none'
  | 'auto'
  | {
      type: 'function';
      function: {
        name: string;
      };
    };
```

The `response_format` parameter ensures you receive a structured response from the LLM. The parameter is only supported by OpenAI models, Nitro models, and some others - check the providers on the model page on openrouter.ai/models to see if it's supported, and set `require_parameters` to true in your Provider Preferences. See openrouter.ai/docs/provider-routing

## Request Headers

OpenRouter allows you to specify an optional `HTTP-Referer` header to identify your app and make it discoverable to users on openrouter.ai. You can also include an optional `X-Title` header to set or modify the title of your app. Example:

```javascript
fetch("https://openrouter.ai/api/v1/chat/completions", {
  method: "POST",
  headers: {
    "Authorization": `Bearer ${OPENROUTER_API_KEY}`,
    "HTTP-Referer": `${YOUR_SITE_URL}`, // Optional, for including your app on openrouter.ai rankings.
    "X-Title": `${YOUR_SITE_NAME}`, // Optional. Shows in rankings on openrouter.ai.
    "Content-Type": "application/json"
  },
  body: JSON.stringify({
    "model": "mistralai/mixtral-8x7b-instruct",
    "messages": [
      
      {"role": "user", "content": "Who are you?"},
      
    ],
  })
});
```

Model routing: If the model parameter is omitted, the user or payer's default is used. Otherwise, remember to select a value for model from the supported models or API, and include the organization prefix. OpenRouter will select the least expensive and best GPUs available to serve the request, and fall back to other providers or GPUs if it receives a 5xx response code or if you are rate-limited.

Streaming: Server-Sent Events (SSE) are supported as well, to enable streaming for all models. Simply send stream: true in your request body. The SSE stream will occasionally contain a "comment" payload, which you should ignore (noted below).

Non-standard parameters: If the chosen model doesn't support a request parameter (such as logit_bias in non-OpenAI models, or top_k for OpenAI), then the parameter is ignored. The rest are forwarded to the underlying model API.

Assistant Prefill: OpenRouter supports asking models to complete a partial response. This can be useful for guiding models to respond in a certain way.

To use this features, simply include a message with role: "assistant" at the end of your messages array. Example:

```javascript
fetch("https://openrouter.ai/api/v1/chat/completions", {
  method: "POST",
  headers: {
    "Authorization": `Bearer ${OPENROUTER_API_KEY}`,
    "HTTP-Referer": `${YOUR_SITE_URL}`, // Optional, for including your app on openrouter.ai rankings.
    "X-Title": `${YOUR_SITE_NAME}`, // Optional. Shows in rankings on openrouter.ai.
    "Content-Type": "application/json"
  },
  body: JSON.stringify({
    "model": "mistralai/mixtral-8x7b-instruct",
    "messages": [
      
      {"role": "user", "content": "Who are you?"},
      {"role": "assistant", "content": "I'm not sure, but my best guess is"},
    ],
  })
});
```

Images & Multimodal Requests
Multimodal requests are only available via the /api/v1/chat/completions API with a multi-part messages parameter. The image_url can either be a URL or a data-base64 encoded image. Example:

```javascript
...
"messages": [
  {
    "role": "user",
    "content": [
      {
        "type": "text",
        "text": "What's in this image?"
      },
      {
        "type": "image_url",
        "image_url": {
          "url": "https://upload.wikimedia.org/wikipedia/commons/thumb/d/dd/Gfp-wisconsin-madison-the-nature-boardwalk.jpg/2560px-Gfp-wisconsin-madison-the-nature-boardwalk.jpg"
        }
      }
    ]
  }
]
```

Sample LLM's response:

```json
{
  "choices": [
    {
      "role": "assistant",
      "content": "This image depicts a scenic natural landscape featuring a long wooden boardwalk that stretches out through an expansive field of green grass. The boardwalk provides a clear path and invites exploration through the lush environment. The scene is surrounded by a variety of shrubbery and trees in the background, indicating a diverse plant life in the area."
    }
  ]
}
```

Uploading base64 encoded images
For locally stored images, you can send them to the model using base64 encoding. Here's an example:

```typescript
import { readFile } from 'fs/promises';

const getFlowerImage = async (): Promise<string> => {
  const imagePath = new URL('flower.jpg', import.meta.url);
  const imageBuffer = await readFile(imagePath);
  const base64Image = imageBuffer.toString('base64');
  return `data:image/jpeg;base64,${base64Image}`;
};

...

'messages': [
  {
    role: 'user',
    content: [
      {
        type: 'text',
        text: "What's in this image?",
      },
      {
        type: 'image_url',
        image_url: {
          url: `${await getFlowerImage()}`,
        },
      },
    ],
  },
];
```

When sending data-base64 string, ensure it contains the content-type of the image. Example:

```javascript
data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAgAAAAIAQMAAAD+wSzIAAAABlBMVEX///+/v7+jQ3Y5AAAADklEQVQI12P4AIX8EAgALgAD/aNpbtEAAAAASUVORK5CYII
```

Supported content types are:

-   image/png
-   image/jpeg
-   image/webp

Tool Calls
Tool calls (also known as function calling) allow you to give an LLM access to external tools. The LLM does not call the tools directly. Instead, it suggests the tool to call. The user then calls the tool separately and provides the results back to the LLM. Finally, the LLM formats the response into an answer to the user's original question.

An example of the five-turn sequence:

The user asks a question, while supplying a list of available tools in a JSON schema format:

```json
...
"messages": [{
  "role": "user",
  "content": "What is the weather like in Boston?"
}],
"tools": [{
"type": "function",
"function": {
    "name": "get_current_weather",
    "description": "Get the current weather in a given location",
    "parameters": {
      "type": "object",
      "properties": {
        "location": {
          "type": "string",
          "description": "The city and state, e.g. San Francisco, CA"
        },
        "unit": {
          "type": "string",
          "enum": [
            "celsius",
            "fahrenheit"
          ]
        }
      },
      "required": [
        "location"
      ]
    }
  }
}],
```

The LLM responds with tool suggestion, together with appropriate arguments:

```json
// Some models might include their reasoning in content
"message": {
  "role": "assistant",
  "content": null,
  "tool_calls": [
    {
      "id": "call_9pw1qnYScqvGrCH58HWCvFH6",
      "type": "function",
      "function": {
        "name": "get_current_weather",
        "arguments": "{ \"location\": \"Boston, MA\"}"
      }
    }
  ]
},
```

The user calls the tool separately:

```typescript
const weather = await getWeather({ location: 'Boston, MA' });
console.log(weather); // { "temperature": "22", "unit": "celsius", "description": "Sunny"}
```

The user provides the tool results back to the LLM:

```json
...
"messages": [
  {
    "role": "user",
    "content": "What is the weather like in Boston?"
  },
  {
    "role": "assistant",
    "content": null,
    "tool_calls": [
      {
        "id": "call_9pw1qnYScqvGrCH58HWCvFH6",
        "type": "function",
        "function": {
          "name": "get_current_weather",
          "arguments": "{ \"location\": \"Boston, MA\"}"
        }
      }
    ]
  },
  {
    "role": "tool",
    "name": "get_current_weather",
    "tool_call_id": "call_9pw1qnYScqvGrCH58HWCvFH6",
    "content": "{\"temperature\": \"22\", \"unit\": \"celsius\", \"description\": \"Sunny\"}"
  }
],
```

The LLM formats the tool result into a natural language response:

```json
...
"message": {
  "role": "assistant",
  "content": "The current weather in Boston, MA is sunny with a temperature of 22°C."
}
```

OpenRouter standardizes the tool calling interface. However, different providers and models may support less tool calling features and arguments. (ex: tool_choice, tool_use, tool_result)

Stream Cancellation
For some providers, streaming requests can be canceled by aborting the connection or simply disconnecting.

When aborting the connection to a provider that supports stream cancellation, the model will stop processing the request, and billing will stop as soon as the upstream provider detects the disconnection.

If you're using the Fetch API, you can use the AbortController to cancel the stream. Here's an example:

```typescript
const controller = new AbortController();

fetch('https://openrouter.ai/api/v1/chat/completions', {
  method: 'POST',
  headers: ...,
  body: ...,
  signal: controller.signal
})

...

// Later, to cancel the stream:
controller.abort();
```

NOTE: Aborting/disconnecting from a non-stream request or a stream request to a provider that does not support stream cancellation will not halt the model's processing in the background. You will still be billed for the rest of the completion.

Chat
Learn how to query our open-source chat models.

Suggest Edits
You can use Together's APIs to send individual queries or have long-running conversations with chat models. You can also configure a system prompt to customize how a model should respond.

Queries run against a model of your choice. For most use cases, we recommend using Meta Llama 3.

Running a single query
Use chat.completions.create to send a single query to a chat model:

Python
TypeScript
HTTP

import os
from together import Together

client = Together()

response = client.chat.completions.create(
    model="meta-llama/Meta-Llama-3-8B-Instruct-Turbo",
    messages=[{"role": "user", "content": "What are some fun things to do in New York?"}],
)

print(response.choices[0].message.content)
The create method takes in a model name and a messages array. Each message is an object that has the content of the query, as well as a role for the message's author.

In the example above, you can see that we're using "user" for the role. The "user" role tells the model that this message comes from the end user of our system – for example, a customer using your chatbot app.

The other two roles are "assistant" and "system", which we'll talk about next.

Having a long-running conversation
Every query to a chat model is self-contained. This means that new queries won't automatically have access to any queries that may have come before them. This is exactly why the "assistant" role exists.

The "assistant" role is used to provide historical context for how a model has responded to prior queries. This makes it perfect for building apps that have long-running conversations, like chatbots.

To provide a chat history for a new query, pass the previous messages to the messages array, denoting the user-provided queries with the "user" role, and the model's responses with the "assistant" role:

Python
TypeScript
HTTP

import os
from together import Together

client = Together()

response = client.chat.completions.create(
    model="meta-llama/Meta-Llama-3-8B-Instruct-Turbo",
    messages=[
      {"role": "user", "content": "What are some fun things to do in New York?"},
      {"role": "assistant", "content": "You could go to the Empire State Building!"},
      {"role": "user", "content": "That sounds fun! Where is it?"},
    ],
)

print(response.choices[0].message.content)
How your app stores historical messages is up to you.

Customizing how the model responds
While you can query a model just by providing a user message, typically you'll want to give your model some context for how you'd like it to respond. For example, if you're building a chatbot to help your customers with travel plans, you might want to tell your model that it should act like a helpful travel guide.

To do this, provide an initial message that uses the "system" role:

Python
TypeScript
HTTP

import os
from together import Together

client = Together()

response = client.chat.completions.create(
    model="meta-llama/Llama-3-8b-chat-hf",
    messages=[
      {"role": "system", "content": "You are a helpful travel guide."},
      {"role": "user", "content": "What are some fun things to do in New York?"},
    ],
)

print(response.choices[0].message.content)
Streaming responses
Since models can take some time to respond to a query, Together's APIs support streaming back responses in chunks. This lets you display results from each chunk while the model is still running, instead of having to wait for the entire response to finish.

To return a stream, set the stream option to true. (If using HTTP, the option name is stream_tokens.)

Python
TypeScript
HTTP

import os
from together import Together

client = Together()

stream = client.chat.completions.create(
  model="meta-llama/Llama-3-8b-chat-hf",
  messages=[{"role": "user", "content": "What are some fun things to do in New York?"}],
  stream=True,
)

for chunk in stream:
  print(chunk.choices[0].delta.content or "", end="", flush=True)
A note on async support in Python
Since I/O in Python is synchronous, multiple queries will execute one after another in sequence, even if they are independent.

If you have multiple independent calls that you want to run in parallel, you can use our Python library's AsyncTogether module:

Python

import os, asyncio
from together import AsyncTogether

async_client = AsyncTogether()
messages = [
    "What are the top things to do in San Francisco?",
    "What country is Paris in?",
]

async def async_chat_completion(messages):
    async_client = AsyncTogether(api_key=os.environ.get("TOGETHER_API_KEY"))
    tasks = [
        async_client.chat.completions.create(
            model="mistralai/Mixtral-8x7B-Instruct-v0.1",
            messages=[{"role": "user", "content": message}],
        )
        for message in messages
    ]
    responses = await asyncio.gather(*tasks)

    for response in responses:
        print(response.choices[0].message.content)

asyncio.run(async_chat_completion(messages))