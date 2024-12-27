<outline critical=true>
    Create a rust cli utilizing asychronous tooling which has the following schema:
</outline critical=true>

<rules critical=true>
    <rule>Keep all provider errors and DO NOT IGNORE THEM</rule>
    <rule>DO NOT IGNORE ANY ERRORS</rule>
    <rule>DO NOT create any tests</rule>
    <rule>Verify project works with bash scripts that simulate user input</rule>
    <rule>Keep cli tool bare bones and minimal</rule>
    <rule>Users MUST be able to view all real world provder errors and responses to troubleshoot provider-user issues like bad keys and low credits</rule>
    <rule>The naming schema for models MUST stay consistent across the codebase:
        - MUST maintain this naming throughout codebase (CRITICAL NOTE: DO NOT BREAK THIS RULE, ALLOW USERS TO USE ANY <provider>:<model> IN CODEBASE)
            - openai:gpt-4o-mini
            - anthropic:claude-3-5-sonnet-latest  
            - togetherai:meta-llama/Llama-3.3-70b-tnstruct-turbo
            - openrouter:google/gemini-2.0-flash-exp
            - anthropic:claude-3.5-haiku-latest
    </rule>
    <rule>Implement everything possible "in-house" for rust, python, and typescript/javascript implementations within rust files to avoid unnecessary dependencies, bloat, and complexity</rule>
    <rule>Give yourself detailed comments throughout the codebase to strategically plan edits and fix critical errors</rule>
    <rule>use testing api keys in testing.txt</rule>
    <CRITICAL_RULE>The system and provider errors MUST be shown to the power users who will be scripting with this cli tool</CRITICAL_RULE>
    <rule>This cli tool is for power users to script with and its very important that all the system and provider errors come back for them to troubleshoot.</rule>
</rules critical=true>
<schema>
usage: aineed -d <set_new_default_model> <provider:model> -s <stream_TRUE> -t <max_tokens> --temp <temperature> -f <file_path_to_read_from> -i <image_generation_TRUE> -p <prompt> -o <output_path>

positional arguments:
    <set_new_default_model>  set <provider:model> as the new default model (default if not provided: togetherai:meta-llama/Llama-3.3-70b-instruct-turbo, -d not required)
    <provider:model>  wrap in model string in " " until encounter " " again or -f or -t use providers naming convention GIVE THESE EXAMPLES: (openai:gpt-3.5-turbo, anthropic:claude-3.5-sonnet, togetherai:llama-2-70b, openrouter:openai/gpt-3.5-turbo)
    <stream_TRUE>  stream output (default if not provided: false, -s not required)
    <max_tokens>  max tokens for the model (default if not provided: none, -t not required)
    <temperature>  temperature for the model (default if not provided: absent from request, --temp not required, MUST BE BETWEEN 0 and 2)
    <file_path_to_read_from>  path to the file (default if not provided: none, -f not required)
    <image_generation_TRUE>  image generation (default if not provided: false, -i not required, DEFAULT filename is provider_model_timestamp.png UNLESS -o user_filename.png is provided)
    <prompt>  wrap prompt in " " (default if " " empty: "Who are the Anishinaabe?")
    <output_path>  path to the output file (default if not provided: none, -o not required)

key arguments:
    --set-openai <openai_api_key>   set the openai api key
    --set-togetherai <togetherai_api_key>   set the togetherai api key
    --set-openrouter <openrouter_api_key>   set the openrouter api key
    --set-anthropic <anthropic_api_key>   set the anthropic api key

optional arguments:
    -h, --help   show this help message and exit
    -v, --version   show program's version number and exit
    -x  --extra   show this extra message and exit KEEP VAGUE AND THEN OUTPUT THE FOLLOWING:("Giin Inna Nanaboozhoo? Help me find Gichi-Waabooz")

</schema>

<examples>
    <note>Will not contain all available models from providers as they change often</note>
    <input example>
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
    </input example>
</examples>
