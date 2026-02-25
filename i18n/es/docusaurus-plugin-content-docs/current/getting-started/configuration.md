---
title: Configuration
sidebar_label: Configuration
---

# Configuring Architect Linter Pro

Architect Linter Pro uses two configuration files:
- **architect.json** - Shared architectural rules (commit to git)
- **.architect.ai.json** - Private API keys (add to .gitignore)

## architect.json

This file defines your architectural rules and constraints.

### Basic Structure

```json
{
  "max_lines_per_function": 50,
  "architecture_pattern": "MVC",
  "forbidden_imports": [
    {
      "from": "src/components/**",
      "to": "src/services/**"
    }
  ]
}
```

### Configuration Fields

#### max_lines_per_function (required)

Maximum number of lines a function can have:

```json
{
  "max_lines_per_function": 50
}
```

**Recommended values by framework:**
- **React**: 20-30 (small components)
- **NestJS**: 30-50 (class methods)
- **Angular**: 40-60 (complex components)
- **Express**: 50-80 (handlers and middleware)
- **Django**: 40-50 (views)

#### architecture_pattern (required)

Your project's architecture pattern:

```json
{
  "architecture_pattern": "MVC"
}
```

**Valid values:**
- `"Hexagonal"` - Hexagonal/Ports and Adapters pattern
- `"Clean"` - Clean Architecture
- `"MVC"` - Model-View-Controller
- `"Ninguno"` - No specific pattern (custom rules only)

#### forbidden_imports (required)

Array of import rules between modules:

```json
{
  "forbidden_imports": [
    {
      "from": "src/components/**",
      "to": "src/services/**",
      "reason": "Components should not directly import services"
    }
  ]
}
```

Each rule requires:
- `from` - Source module path (glob pattern)
- `to` - Target module path (glob pattern)
- `reason` (optional) - Why this import is forbidden

## .architect.ai.json

This file stores API keys for AI features. **Never commit this to git!**

### Basic Structure

```json
{
  "provider": "anthropic",
  "api_key": "your-api-key-here",
  "base_url": "https://api.anthropic.com",
  "model": "claude-3-sonnet-20240229"
}
```

### Supported Providers

- **Claude (Anthropic)** - Recommended
- **OpenAI** - GPT-4, GPT-3.5
- **Google Gemini** - Gemini Pro
- **Groq** - Fast inference
- **Ollama** - Local models
- **DeepSeek** - Cost-effective
- **Kimi** - Alternative option

### Environment Variables

Instead of .architect.ai.json, you can use environment variables:

```bash
export ANTHROPIC_AUTH_TOKEN="your-api-key"
export ANTHROPIC_BASE_URL="https://api.anthropic.com"
export ANTHROPIC_MODEL="claude-3-sonnet-20240229"
```

## Framework Templates

For framework-specific configuration examples, see [Templates](/docs/templates).

## Next Steps

- [Write Your First Rules](/docs/getting-started/first-rules)
- [Configuration Error Troubleshooting](/docs/troubleshooting/config-errors)
- [Available Guides](/docs/guides)
