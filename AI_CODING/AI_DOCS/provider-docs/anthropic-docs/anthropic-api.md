### cURL input
```json
curl https://api.anthropic.com/v1/messages \
     --header "x-api-key: $ANTHROPIC_API_KEY" \
     --header "anthropic-version: 2023-06-01" \
     --header "content-type: application/json" \
     --data \
'{
    "model": "claude-3-5-sonnet-lastest",
    "max_tokens": 1024,
    "messages": [
        {"role": "user", "content": "Hello, world"}
    ]
}'
```

### cURL input example2
```bash
#!/bin/sh

# Define image parameters for Claude
# Supported media types
IMAGE_FORMATS=(
    "image/jpeg"
    "image/png"
    "image/gif"
    "image/webp"
)

# Set image path and detect media type
IMAGE_PATH="/path/to/local/image"
IMAGE_MEDIA_TYPE=$(file -b --mime-type "$IMAGE_PATH")

# Validate media type
if [[ ! " ${IMAGE_FORMATS[@]} " =~ " ${IMAGE_MEDIA_TYPE} " ]]; then
    echo "Error: Unsupported image format. Supported formats are: ${IMAGE_FORMATS[*]}"
    exit 1
fi
IMAGE_BASE64=$(base64 "$IMAGE_PATH")
```

```json
curl https://api.anthropic.com/v1/messages \
     --header "x-api-key: $ANTHROPIC_API_KEY" \
     --header "anthropic-version: 2023-06-01" \
     --header "content-type: application/json" \
     --data \
'{
    "model": "claude-3-5-sonnet-latest",
    "max_tokens": 1024,
    "messages": [
        {"role": "user", "content": [
            {"type": "image", "source": {
                "type": "base64",
                "media_type": "'$IMAGE_MEDIA_TYPE'",
                "data": "'$IMAGE_BASE64'"
            }},
            {"type": "text", "text": "What is in the above image?"}
        ]}
    ]
}'
```

### model naming convention for anthropic
### MUST KEEP THESE MODEL NAMES
 <CRITICAL-NAMING-CONVENTION>
            claude-3-5-sonnet-latest
            claude-3-5-haiku-latest
            claude-3-opus-latest
 </CRITICAL-NAMING-CONVENTION>
 
 # Streaming Messages

When creating a Message, you can set `"stream": true` to incrementally stream the response using server-sent events (SSE).

## Streaming with SDKs

Our Python and TypeScript SDKs offer multiple ways of streaming. The Python SDK allows both sync and async streams. See the documentation in each SDK for details.

<CodeTabs>
<CodeTab title="Python">

```python
import anthropic

client = anthropic.Anthropic()

with client.messages.stream(
    max_tokens=1024,
    messages=[{"role": "user", "content": "Hello"}],
    model="claude-3-5-sonnet-20241022",
) as stream:
  for text in stream.text_stream:
      print(text, end="", flush=True)
```

</CodeTab>
<CodeTab title="TypeScript">

```typescript
import Anthropic from '@anthropic-ai/sdk';

const client = new Anthropic();

await client.messages.stream({
    messages: [{role: 'user', content: "Hello"}],
    model: 'claude-3-5-sonnet-20241022',
    max_tokens: 1024,
}).on('text', (text) => {
    console.log(text);
});
```

</CodeTab>
</CodeTabs>

## Event types

Each server-sent event includes a named event type and associated JSON data. Each event will use an SSE event name (e.g. `event: message_stop`), and include the matching event type in its data.

Each stream uses the following event flow:

1. `message_start`: contains a Message object with empty content.
2. A series of content blocks, each of which have a `content_block_start`, one or more `content_block_delta` events, and a `content_block_stop` event. Each content block will have an index that corresponds to its index in the final Message content array.
3. One or more `message_delta` events, indicating top-level changes to the final Message object.
4. A final `message_stop` event.

### Ping events

Event streams may also include any number of ping events.

### Error events

We may occasionally send errors in the event stream. For example, during periods of high usage, you may receive an `overloaded_error`, which would normally correspond to an HTTP 529 in a non-streaming context:

<CodeBlock title="Example error">

```
event: error
data: {"type": "error", "error": {"type": "overloaded_error", "message": "Overloaded"}}
```

</CodeBlock>

### Other events

In accordance with our versioning policy, we may add new event types, and your code should handle unknown event types gracefully.

## Delta types

Each `content_block_delta` event contains a delta of a type that updates the content block at a given index.

### Text delta

A text content block delta looks like:

<CodeBlock title="Text delta">

```
event: content_block_delta
data: {"type": "content_block_delta","index": 0,"delta": {"type": "text_delta", "text": "ello frien"}}
```

</CodeBlock>

### Input JSON delta

The deltas for `tool_use` content blocks correspond to updates for the input field of the block. To support maximum granularity, the deltas are partial JSON strings, whereas the final `tool_use.input` is always an object.

You can accumulate the string deltas and parse the JSON once you receive a `content_block_stop` event, by using a library like Pydantic to do partial JSON parsing, or by using our SDKs, which provide helpers to access parsed incremental values.

A `tool_use` content block delta looks like:

<CodeBlock title="Input JSON delta">

```
event: content_block_delta
data: {"type": "content_block_delta","index": 1,"delta": {"type": "input_json_delta","partial_json": "{\"location\": \"San Fra"}}
```

</CodeBlock>

Note: Our current models only support emitting one complete key and value property from input at a time. As such, when using tools, there may be delays between streaming events while the model is working. Once an input key and value are accumulated, we emit them as multiple `content_block_delta` events with chunked partial json so that the format can automatically support finer granularity in future models.

## Raw HTTP Stream response

We strongly recommend that use our client SDKs when using streaming mode. However, if you are building a direct API integration, you will need to handle these events yourself.

A stream response is comprised of:

1. A `message_start` event
2. Potentially multiple content blocks, each of which contains:
   a. A `content_block_start` event
   b. Potentially multiple `content_block_delta` events
   c. A `content_block_stop` event
3. A `message_delta` event
4. A `message_stop` event

There may be ping events dispersed throughout the response as well. See Event types for more details on the format.

### Basic streaming request

<CodeBlock title="Request">

```bash
curl https://api.anthropic.com/v1/messages \
     --header "anthropic-version: 2023-06-01" \
     --header "content-type: application/json" \
     --header "x-api-key: $ANTHROPIC_API_KEY" \
     --data \
'{
  "model": "claude-3-5-sonnet-20241022",
  "messages": [{"role": "user", "content": "Hello"}],
  "max_tokens": 256,
  "stream": true
}'
```

</CodeBlock>

<CodeBlock title="Response">

```
event: message_start
data: {"type": "message_start", "message": {"id": "msg_1nZdL29xx5MUA1yADyHTEsnR8uuvGzszyY", "type": "message", "role": "assistant", "content": [], "model": "claude-3-5-sonnet-20241022", "stop_reason": null, "stop_sequence": null, "usage": {"input_tokens": 25, "output_tokens": 1}}}

event: content_block_start
data: {"type": "content_block_start", "index": 0, "content_block": {"type": "text", "text": ""}}

event: ping
data: {"type": "ping"}

event: content_block_delta
data: {"type": "content_block_delta", "index": 0, "delta": {"type": "text_delta", "text": "Hello"}}

event: content_block_delta
data: {"type": "content_block_delta", "index": 0, "delta": {"type": "text_delta", "text": "!"}}

event: content_block_stop
data: {"type": "content_block_stop", "index": 0}

event: message_delta
data: {"type": "message_delta", "delta": {"stop_reason": "end_turn", "stop_sequence":null}, "usage": {"output_tokens": 15}}

event: message_stop
data: {"type": "message_stop"}
```

</CodeBlock>

### Streaming request with tool use

In this request, we ask Claude to use a tool to tell us the weather.

<CodeBlock title="Request">