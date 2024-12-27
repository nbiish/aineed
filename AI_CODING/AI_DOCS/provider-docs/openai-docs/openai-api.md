### cURL input
```json
curl https://api.openai.com/v1/chat/completions \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $OPENAI_API_KEY" \
  -d '{
     "model": "gpt-4o-mini",
     "messages": [{"role": "user", "content": "Say this is a test!"}],
     "temperature": 0.7
   }'
   ```

### cURL input example2
```json
curl https://api.openai.com/v1/chat/completions \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $OPENAI_API_KEY" \
  -d '{
    "model": "gpt-4o-mini",
    "messages": [
      {
        "role": "user",
        "content": [
          {
            "type": "text",
            "text": "What’s in this image?"
          },
          {
            "type": "image_url",
            "image_url": {
              "url": "https://upload.wikimedia.org/wikipedia/commons/thumb/d/dd/Gfp-wisconsin-madison-the-nature-boardwalk.jpg/2560px-Gfp-wisconsin-madison-the-nature-boardwalk.jpg"
            }
          }
        ]
      }
    ],
    "max_tokens": 300
  }'
  ```

### cURL input example3
```json
#!/bin/sh

IMAGE_PATH="/path/to/local/image.jpg"
IMAGE_BASE64=$(base64 "$IMAGE_PATH")

curl https://api.openai.com/v1/chat/completions \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $OPENAI_API_KEY" \
  -d '{
    "model": "gpt-4o-mini",
    "messages": [
      {
        "role": "user",
        "content": [
          {
            "type": "text",
            "text": "What are in these images? Is there any difference between them?"
          },
          {
            "type": "image_url",
            "image_url": {
              "url": "data:image/jpeg;base64,'$IMAGE_BASE64_1'"
            }
          }
        ]
      }
    ],
    "max_tokens": 300
  }'
```

### model naming convention for openai
### MUST KEEP THESE MODEL NAMES
<CRITICAL-NAMING-CONVENTION>
        gpt-4o
        gpt-4o-mini
        o1-preview
        o1-mini
        gpt-4-turbo
        gpt-4
        gpt-3.5-turbo
</CRITICAL-NAMING-CONVENTION>
