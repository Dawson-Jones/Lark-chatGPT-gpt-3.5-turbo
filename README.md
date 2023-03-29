# chatGpt 飞书 gpt-3.5-turbo Rust

使用方式

- 打开 `config.toml`

修改

```toml
[feishu]
LARK_HOST = "https://open.feishu.cn"
Verification_Token = "iiTAgh6hqGfsOgYours_Verification_Token"
APP_ID = "cli_a48a6xxxxxxx"
APP_SECRET = "Yj8SWiGMz5p09Ul5nGk0yxxxxx"
BOT_NAME = "your_bot_name"

[gpt]
URL = "https://api.openai.com/v1/chat/completions"
API_KEY = "sk-3tyG7e8uVPG3wyEhtE0VT3Blgpt_api_key"
CONTEXT_LIMIT = 5   // 上下文的数量

    [gpt.body]
    model = "gpt-3.5-turbo"
    temperature = 0.9
    max_tokens = 2048

[Host]
IP = "0.0.0.0"
Port = 3000
```
