# Create completion

**POST** `https://api.together.xyz/v1/completions`

Query a language, code, or image model.

## Body Params

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| prompt | string | Yes | A string providing context for the model to complete. |
| model | string | Yes | The name of the model to query. See all of Together AI's chat models. |
| max_tokens | integer | No | The maximum number of tokens to generate. |
| stop | array of strings | No | A list of string sequences that will truncate (stop) inference text output. For example, "" will stop generation as soon as the model generates the given token. |
| temperature | float | No | A decimal number from 0-1 that determines the degree of randomness in the response. A temperature less than 1 favors more correctness and is appropriate for question answering or summarization. A value closer to 1 introduces more randomness in the output. |
| top_p | float | No | A percentage (also called the nucleus parameter) that's used to dynamically adjust the number of choices for each predicted token based on the cumulative probabilities. It specifies a probability threshold below which all less likely tokens are filtered out. This technique helps maintain diversity and generate more fluent and natural-sounding text. |
| top_k | int32 | No | An integer that's used to limit the number of choices for the next predicted word or token. It specifies the maximum number of tokens to consider at each step, based on their probability of occurrence. This technique helps to speed up the generation process and can improve the quality of the generated text by focusing on the most likely options. |
| repetition_penalty | float | No | A number that controls the diversity of generated text by reducing the likelihood of repeated sequences. Higher values decrease repetition. |
| stream | boolean | No | If true, stream tokens as Server-Sent Events as the model generates them instead of waiting for the full model response. The stream terminates with data: [DONE]. If false, return a single JSON object containing the results. |
| logprobs | integer | No | Determines the number of most likely tokens to return at each token position log probabilities to return. Range: 0 to 1. |
| echo | boolean | No | If true, the response will contain the prompt. Can be used with logprobs to return prompt logprobs. |
| n | integer | No | The number of completions to generate for each prompt. Range: 1 to 128. |
| safety_model | string | No | The name of the moderation model used to validate tokens. Choose from the available moderation models found here. |
| min_p | float | No | A number between 0 and 1 that can be used as an alternative to top-p and top-k. |
| presence_penalty | float | No | A number between -2.0 and 2.0 where a positive value increases the likelihood of a model talking about new topics. |
| frequency_penalty | float | No | A number between -2.0 and 2.0 where a positive value decreases the likelihood of repeating tokens that have already been mentioned. |
| logit_bias | object | No | Adjusts the likelihood of specific tokens appearing in the generated output. |
| seed | integer | No | Seed value for reproducibility. |

## Node.js request example

```javascript
const url = 'https://api.together.xyz/v1/completions';
const options = {
  method: 'POST',
  headers: {
    accept: 'application/json',
    'content-type': 'application/json',
    authorization: 'Bearer default'
  },
  body: JSON.stringify({model: 'meta-llama/Llama-3.3-70B-Instruct-Turbo', prompt: 'Who are the Anishinaabe?'})
};

fetch(url, options)
  .then(res => res.json())
  .then(json => console.log(json))
  .catch(err => console.error(err));
```

## Python request example

```python
import requests

url = "https://api.together.xyz/v1/completions"

payload = {
    "model": "meta-llama/Llama-3.3-70B-Instruct-Turbo",
    "prompt": "Who are the Anishinaabe?"
}
headers = {
    "accept": "application/json",
    "content-type": "application/json",
    "authorization": "Bearer default"
}

response = requests.post(url, json=payload, headers=headers)

print(response.text)
```

# Create image

**POST** `https://api.together.xyz/v1/images/generations`

Use an image model to generate an image for a given prompt.

## Body Params

| Parameter | Type | Required | Description | Default |
|-----------|------|----------|-------------|---------|
| prompt | string | Yes | A description of the desired images. Maximum length varies by model. | |
| model | string | Yes | The model to use for image generation. See all of Together AI's image models. | |
| steps | integer | No | Number of generation steps. | 20 |
| image_url | string | No | URL of an image to use for image models that support it. | |
| seed | integer | No | Seed used for generation. Can be used to reproduce image generations. | |
| n | integer | No | Number of image results to generate. | 1 |
| height | integer | No | Height of the image to generate in number of pixels. | 1024 |
| width | integer | No | Width of the image to generate in number of pixels. | 1024 |
| negative_prompt | string | No | The prompt or prompts not to guide the image generation. | |
| response_format | string | No | Format of the image response. Can be either a base64 string or a URL. | |

## Node.js request example

```javascript
const url = 'https://api.together.xyz/v1/images/generations';
const options = {
  method: 'POST',
  headers: {
    accept: 'application/json',
    'content-type': 'application/json',
    authorization: 'Bearer default'
  },
  body: JSON.stringify({
    model: 'black-forest-labs/FLUX.1-schnell',
    prompt: 'Who are the Anishinaabe?',
    steps: 20,
    n: 1,
    height: 1024,
    width: 1024
  })
};

fetch(url, options)
  .then(res => res.json())
  .then(json => console.log(json))
  .catch(err => console.error(err));
```

## Python request example

```python
import requests

url = "https://api.together.xyz/v1/images/generations"

payload = {
    "model": "black-forest-labs/FLUX.1-schnell",
    "prompt": "Who are the Anishinaabe?",
    "steps": 20,
    "n": 1,
    "height": 1024,
    "width": 1024
}
headers = {
    "accept": "application/json",
    "content-type": "application/json",
    "authorization": "Bearer default"
}

response = requests.post(url, json=payload, headers=headers)

print(response.text)
```