---
title: AI Configuration
sidebar_label: AI Config
---

# AI Configuration (.architect.ai.json)

Configure AI providers for auto-fix and suggestion features.

## File Location

Create `.architect.ai.json` in your project root (add to .gitignore).

## Supported Providers

### Anthropic (Claude)

```json
{
  "provider": "anthropic",
  "api_key": "sk-ant-...",
  "base_url": "https://api.anthropic.com",
  "model": "claude-3-sonnet-20240229"
}
```

Recommended models:
- claude-3-opus-20240229 (most capable)
- claude-3-sonnet-20240229 (balanced)
- claude-3-haiku-20240307 (fast, cheap)

### OpenAI

```json
{
  "provider": "openai",
  "api_key": "sk-...",
  "model": "gpt-4"
}
```

### Google Gemini

```json
{
  "provider": "gemini",
  "api_key": "your-key",
  "model": "gemini-pro"
}
```

### Groq

```json
{
  "provider": "groq",
  "api_key": "gsk_...",
  "model": "mixtral-8x7b-32768"
}
```

### Ollama (Local)

```json
{
  "provider": "ollama",
  "base_url": "http://localhost:11434",
  "model": "llama2"
}
```

### DeepSeek

```json
{
  "provider": "deepseek",
  "api_key": "sk-...",
  "model": "deepseek-chat"
}
```

### Kimi

```json
{
  "provider": "kimi",
  "api_key": "your-key",
  "model": "moonshot-v1-8k"
}
```

## Environment Variables

Alternative to .architect.ai.json:

```bash
export ANTHROPIC_AUTH_TOKEN="sk-ant-..."
export ANTHROPIC_BASE_URL="https://api.anthropic.com"
export ANTHROPIC_MODEL="claude-3-sonnet-20240229"
```

## Using AI Features

With AI configured, use:

```bash
architect --fix
```

Features:
- Auto-fix code violations
- Suggest architectural rules
- Explain violations
- Multi-model fallback support

## Privacy & Security

- API keys stored in .architect.ai.json (not committed)
- All code sent to provider for analysis
- Consider self-hosted options (Ollama) for privacy
