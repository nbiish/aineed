### cURL input
```json
curl -X POST "https://api.together.xyz/v1/chat/completions" \
  -H "Authorization: Bearer $TOGETHER_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "model": "Qwen/QwQ-32B-Preview",
    "messages": [{"role": "user", "content": "What are some fun things to do in New York?"}]
  }'
  ```
### cURL input example2
```json
curl --request POST \
     --url https://api.together.xyz/inference \
     --header 'Authorization: Bearer f07663ad69f60681d07a762895c7b269a9f9cd0d790fdad00b3e376841ce7302' \
     --header 'accept: application/json' \
     --header 'content-type: application/json' \
     --data '
{
  "model": "mistralai/Mixtral-8x7B-Instruct-v0.1",
  "prompt": "<s>[INST] What is the capital of France? [/INST]",
  "max_tokens": 512,
  "stop": [
    "</s>",
    "[/INST]"
  ],
  "temperature": 0.7,
  "top_p": 0.7,
  "top_k": 50,
  "repetition_penalty": 1,
  "n": 1
}
```

### model naming convention for togetherai
<CRITICAL-NAMING-CONVENTION>
        Qwen/QwQ-32B-Preview
        meta-llama/Llama-3.3-70B-Instruct-Turbo
</CRITICAL-NAMING-CONVENTION>
