"""
llm_client.py — Cliente LLM multi-proveedor para Architect.

Soporta: Gemini (Google), Claude (Anthropic), OpenAI, Ollama (local).
Selección por variable de entorno: LLM_PROVIDER=gemini|claude|openai|ollama

El rol de este módulo es recibir un contexto + resultado crudo de Architect Core
y retornar un análisis/síntesis en texto que va de vuelta a Cerebro.
"""

import os
import json
from typing import Optional, Dict, Any
from .settings import settings


# ──────────────────────────────────────────────
# Prompt base para análisis de arquitectura
# ──────────────────────────────────────────────

SYSTEM_PROMPT = """Eres el Agente Architect de Skrymir Suite — un Consultor de Arquitectura Senior.
Tu trabajo es analizar los resultados de análisis de arquitectura y producir
un reporte conciso, claro y accionable en español.

Reglas:
- Si hay violaciones de arquitectura, clasifícalas por severidad y tipo (acoplamiento, cohesión, complejidad).
- Para cada violación importante, propón una solución concreta de refactoring.
- Si hay dependencias circulares, explica por qué son peligrosas y cómo romperlas.
- Si el contexto histórico indica que un archivo tiene problemas recurrentes, menciónalo.
- Si el health score es bajo (<60), marca esto como crítico.
- Sé directo y útil. No repitas datos que ya están en el JSON, interprétalos.
- Máximo 400 palabras en la respuesta."""


def _build_analysis_prompt(action: str, raw_result: dict, memory_context: Optional[dict]) -> str:
    """
    Construye el prompt que recibe el LLM.
    Combina el resultado crudo del Core Rust + contexto de memoria histórica.
    """
    lines = [
        f"## Acción ejecutada: `{action}`",
        "",
        "### Resultado del Architect Core:",
        "```json",
        json.dumps(raw_result, indent=2, ensure_ascii=False)[:3000],  # Truncar si es muy largo
        "```",
    ]

    if memory_context:
        hot_files = memory_context.get("hot_files", [])
        recent_critical = memory_context.get("recent_critical_findings", [])
        learned_patterns = memory_context.get("learned_patterns", [])
        avg_health = memory_context.get("average_health_score_7d", 100.0)

        if hot_files:
            lines += [
                "",
                "### Archivos con más problemas históricos (hot files):",
                "```json",
                json.dumps(hot_files[:5], indent=2, ensure_ascii=False),
                "```",
            ]
        if recent_critical:
            lines += [
                "",
                "### Hallazgos críticos recientes:",
                "```json",
                json.dumps(recent_critical[:3], indent=2, ensure_ascii=False),
                "```",
            ]
        if learned_patterns:
            lines += [
                "",
                "### Patrones de arquitectura detectados:",
                "```json",
                json.dumps(learned_patterns[:3], indent=2, ensure_ascii=False),
                "```",
            ]
        lines += [
            "",
            f"### Health Score promedio (últimos 7 días): {avg_health}",
        ]

    lines += [
        "",
        "Analiza los resultados anteriores y produce un reporte accionable con recomendaciones de refactoring.",
    ]

    return "\n".join(lines)


def _build_fix_prompt(violation: dict, file_content: str, memory_context: Optional[dict]) -> str:
    """
    Construye el prompt para proponer fixes automáticos.
    """
    lines = [
        "Eres un experto en refactoring de arquitectura de software.",
        "",
        "## Violación detectada:",
        f"- Archivo: {violation.get('file', 'N/A')}",
        f"- Regla violada: {violation.get('rule', 'N/A')}",
        f"- Severidad: {violation.get('severity', 'warning')}",
        f"- Mensaje: {violation.get('message', '')}",
        "",
        "## Contenido actual del archivo:",
        "```",
        file_content[:2000],  # Limitar tamaño
        "```",
    ]

    if memory_context:
        hot_files = memory_context.get("hot_files", [])
        relevant = [f for f in hot_files if violation.get('file') in f.get('file_path', '')]
        if relevant:
            lines += [
                "",
                "### Historial de este archivo:",
                json.dumps(relevant[0], indent=2),
            ]

    lines += [
        "",
        "## Tu tarea:",
        "1. Identifica qué cambio arquitectónico resolvería la violación.",
        "2. Propón el código modificado o el plan de refactoring.",
        "3. Si es necesario mover/crear archivos, indica la estructura propuesta.",
        "",
        "Responde en formato JSON:",
        '{"fix_type": "refactor|move|extract|create", "description": "...", "code_changes": "..."}'
    ]

    return "\n".join(lines)


# ──────────────────────────────────────────────
# Implementaciones por proveedor
# ──────────────────────────────────────────────

async def _analyze_with_gemini(prompt: str) -> str:
    try:
        import google.generativeai as genai
        genai.configure(api_key=settings.google_api_key)
        model = genai.GenerativeModel(
            model_name=settings.gemini_model,
            system_instruction=SYSTEM_PROMPT,
        )
        response = await model.generate_content_async(prompt)
        return response.text
    except ImportError:
        return "[Error] google-generativeai no instalado. Ejecuta: pip install google-generativeai"
    except Exception as exc:
        return f"[Error Gemini] {exc}"


async def _analyze_with_claude(prompt: str) -> str:
    try:
        import anthropic
        client = anthropic.AsyncAnthropic(api_key=settings.anthropic_api_key)
        message = await client.messages.create(
            model=settings.claude_model,
            max_tokens=1024,
            system=SYSTEM_PROMPT,
            messages=[{"role": "user", "content": prompt}],
        )
        return message.content[0].text
    except ImportError:
        return "[Error] anthropic no instalado. Ejecuta: pip install anthropic"
    except Exception as exc:
        return f"[Error Claude] {exc}"


async def _analyze_with_openai(prompt: str) -> str:
    try:
        from openai import AsyncOpenAI
        client = AsyncOpenAI(api_key=settings.openai_api_key)
        response = await client.chat.completions.create(
            model=settings.openai_model,
            messages=[
                {"role": "system", "content": SYSTEM_PROMPT},
                {"role": "user", "content": prompt},
            ],
            max_tokens=1024,
        )
        return response.choices[0].message.content or ""
    except ImportError:
        return "[Error] openai no instalado. Ejecuta: pip install openai"
    except Exception as exc:
        return f"[Error OpenAI] {exc}"


async def _analyze_with_ollama(prompt: str) -> str:
    """
    Llama a Ollama usando su endpoint OpenAI-compatible /v1/chat/completions.
    No requiere librerías adicionales: usa httpx (ya es dependencia).
    """
    import httpx

    url = f"{settings.ollama_base_url.rstrip('/')}/v1/chat/completions" if hasattr(settings, 'ollama_base_url') else "http://localhost:11434/v1/chat/completions"
    model = getattr(settings, 'ollama_model', 'qwen3:8b')

    print(f"  🦙 [Ollama] URL: {url}")
    print(f"  🦙 [Ollama] Modelo: {model}")
    print(f"  🦙 [Ollama] Prompt size: {len(prompt)} chars")

    payload = {
        "model": model,
        "messages": [
            {"role": "system", "content": SYSTEM_PROMPT},
            {"role": "user",   "content": prompt},
        ],
        "stream": False,
    }
    try:
        print(f"  🦙 [Ollama] Enviando request...")
        async with httpx.AsyncClient(timeout=300.0) as client:
            resp = await client.post(url, json=payload)
            resp.raise_for_status()
            data = resp.json()
            print(f"  ✅ [Ollama] Respuesta recibida!")
            return data["choices"][0]["message"]["content"]
    except httpx.ConnectError:
        return (
            f"[Error Ollama] No hay conexión en {url}. "
            "Verifica que Ollama esté corriendo con: ollama serve"
        )
    except httpx.HTTPStatusError as exc:
        return f"[Error Ollama] HTTP {exc.response.status_code}: {exc.response.text[:200]}"
    except Exception as exc:
        return f"[Error Ollama] {exc}"


# ──────────────────────────────────────────────
# Interfaz pública
# ──────────────────────────────────────────────

async def analyze_result(
    action: str,
    raw_result: dict,
    memory_context: Optional[dict] = None,
) -> str:
    """
    Punto de entrada principal.
    Toma el resultado crudo del Architect Core y lo analiza con el LLM configurado.

    Args:
        action:          Acción que generó el resultado (lint, analyze, check-circular, etc.)
        raw_result:      JSON retornado por el Architect Core (Rust)
        memory_context:  Contexto histórico de la memoria SQLite (opcional)

    Returns:
        Análisis textual conciso y accionable en español.
    """
    provider = settings.llm_provider
    print(f"🤖 [LLM] Provider configurado: '{provider}'")
    print(f"🤖 [LLM] Action: '{action}'")

    # Verificar que el raw_result tiene datos
    res_data = raw_result.get("result", {})
    findings = res_data.get("findings_count", 0)
    health = res_data.get("health_score", "N/A")
    print(f"🤖 [LLM] Datos recibidos: {findings} violaciones, health_score={health}")

    prompt = _build_analysis_prompt(action, raw_result, memory_context)
    print(f"🤖 [LLM:{provider}] Llamando a proveedor...")

    if provider == "gemini":
        return await _analyze_with_gemini(prompt)
    elif provider == "claude":
        return await _analyze_with_claude(prompt)
    elif provider == "openai":
        return await _analyze_with_openai(prompt)
    elif provider == "ollama":
        return await _analyze_with_ollama(prompt)
    else:
        return (
            f"[Error] Proveedor LLM desconocido: '{provider}'. "
            "Usa: gemini | claude | openai | ollama"
        )


async def suggest_fix(
    violation: dict,
    file_content: str,
    memory_context: Optional[dict] = None,
) -> dict:
    """
    Sugiere un fix automático para una violación específica.

    Args:
        violation:      Dict con la información de la violación
        file_content:   Contenido actual del archivo
        memory_context: Contexto histórico

    Returns:
        Dict con fix_type, description, code_changes
    """
    prompt = _build_fix_prompt(violation, file_content, memory_context)
    provider = settings.llm_provider

    print(f"🤖 [LLM:{provider}] Sugiriendo fix para violación en {violation.get('file', 'N/A')}...")

    response = ""
    if provider == "gemini":
        response = await _analyze_with_gemini(prompt)
    elif provider == "claude":
        response = await _analyze_with_claude(prompt)
    elif provider == "openai":
        response = await _analyze_with_openai(prompt)
    elif provider == "ollama":
        response = await _analyze_with_ollama(prompt)
    else:
        return {"fix_type": "none", "description": f"Proveedor no soportado: {provider}", "code_changes": None}

    # Intentar parsear JSON de la respuesta
    try:
        import re
        json_match = re.search(r'\{.*\}', response, re.DOTALL)
        if json_match:
            return json.loads(json_match.group(0))
    except Exception:
        pass

    # Fallback: retornar como descripción textual
    return {
        "fix_type": "refactor",
        "description": response[:500],
        "code_changes": None
    }


async def identify_patterns_in_code(file_path: str, content: str) -> Dict[str, str]:
    """
    Usa el LLM configurado para identificar patrones de diseño en un archivo dado.
    Servirá para la 'Memoria de Patrones' de Architect.
    """
    prompt = f"""
    Identifica los PATRONES DE DISEÑO o el ROL ARQUITECTÓNICO del siguiente archivo: '{file_path}'.

    Código:
    ```
    {content[:3000]}
    ```

    Responde en formato JSON:
    {{
       "pattern_type": "Repository | Service | Controller | Model | Middleware | Component | Utility | Custom",
       "summary": "Breve resumen (1 frase) de la responsabilidad del archivo"
    }}
    Responde ÚNICAMENTE el JSON.
    """

    provider = settings.llm_provider
    response = ""

    try:
        if provider == "gemini" and settings.google_api_key:
            response = await _analyze_with_gemini(prompt)
        elif provider == "claude" and settings.anthropic_api_key:
            response = await _analyze_with_claude(prompt)
        elif provider == "openai" and settings.openai_api_key:
            response = await _analyze_with_openai(prompt)
        elif provider == "ollama":
            response = await _analyze_with_ollama(prompt)
        else:
            return {}

        # Extraer JSON de markdown si viene así
        import re
        clean_json = re.search(r'\{.*\}', response, re.DOTALL)
        if clean_json:
            return json.loads(clean_json.group(0))
        return {}
    except Exception as e:
        print(f"❌ [LLM Pattern] Error: {e}")
        return {}


# Legacy: mantener para compatibilidad
async def analyze_design(action: str, raw_result: Dict[str, Any], context: Optional[Dict[str, Any]] = None) -> str:
    """Legacy wrapper para analyze_result."""
    return await analyze_result(action, raw_result, context)
