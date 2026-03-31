"""
tools.py — Wrappers de comunicación con el Core Rust de Architect (4002).

Cada función ejecuta una acción en el Core y persiste el resultado en memoria SQLite.
El análisis/razonamiento lo hace el LLM en bridge.py.

Flujo:
  Cerebro solicita action  →  tool llama al Core Rust
  Core retorna raw JSON    →  tool persiste en SQLite
  raw JSON sube al bridge  →  LLM analiza y produce síntesis
  síntesis va a Cerebro    →  Cerebro decide qué hacer
"""

import time
import uuid
import httpx
from typing import Optional, Tuple, Dict, Any
from .settings import settings
from . import memory


CORE = settings.architect_core_url


async def call_core(action: str, target: Optional[str] = None, options: Optional[dict] = None, request_id: Optional[str] = None) -> dict:
    """
    Realiza una petición POST al endpoint /command del Core Rust.
    """
    url = f"{CORE}/command"
    payload = {
        "action": action,
        "target": target or ".",
        "options": options or {},
        "request_id": request_id or str(uuid.uuid4())
    }

    print(f"📡 [Architect-Core] Enviando comando: {action} en {target or '.'} (req_id: {payload['request_id'][:8]}...)")

    try:
        async with httpx.AsyncClient(timeout=60.0) as client:
            resp = await client.post(url, json=payload)
            if resp.status_code != 200:
                print(f"❌ [Architect-Core] Error status {resp.status_code}: {resp.text}")
                return {"status": "error", "error": f"Status code {resp.status_code}", "raw_response": resp.text}

            return resp.json()
    except Exception as e:
        print(f"❌ [Architect-Core] Error de conexión: {str(e)}")
        return {"status": "error", "error": f"Connection error: {str(e)}"}


# ──────────────────────────────────────────────
# Acciones disponibles con persistencia
# Cada función = 1 acción del Core + persistencia en SQLite
# ──────────────────────────────────────────────

async def execute_lint(target: str = ".", request_id: Optional[str] = None) -> Tuple[dict, int]:
    """Linting de arquitectura. Retorna (raw_result, memory_id)."""
    start = time.monotonic()
    result = await call_core("lint", target=target, request_id=request_id)
    duration_ms = int((time.monotonic() - start) * 1000)

    # Extraer datos para persistencia
    res_data = result.get("result", {})
    findings_count = res_data.get("findings_count", 0)
    health_score = res_data.get("health_score", 100)
    has_cycles = res_data.get("has_cycles", False)

    severity = "error" if result.get("status") == "error" else (
        "critical" if has_cycles else ("warning" if findings_count > 0 else "info")
    )

    # Guardar hallazgo
    mid = await memory.save_finding("lint_completed", severity, result, target)

    # Guardar análisis run
    await memory.save_analysis_run(
        "lint", target, {"status": result.get("status")},
        duration_ms, health_score, findings_count
    )

    # Actualizar perfiles de archivos si hay hallazgos
    if "findings" in res_data and isinstance(res_data["findings"], list):
        for finding in res_data["findings"]:
            if isinstance(finding, dict) and "file" in finding:
                await memory.update_file_profile(
                    finding["file"],
                    health_score,
                    1,
                    finding.get("severity") == "error",
                    None
                )

    return result, mid


async def execute_analyze(target: str = ".", request_id: Optional[str] = None) -> Tuple[dict, int]:
    """Análisis completo de arquitectura. Retorna (raw_result, memory_id)."""
    start = time.monotonic()
    result = await call_core("analyze", target=target, request_id=request_id)
    duration_ms = int((time.monotonic() - start) * 1000)

    res_data = result.get("result", {})
    findings_count = res_data.get("findings_count", 0)
    health_score = res_data.get("health_score", 100)

    severity = "error" if result.get("status") == "error" else (
        "warning" if findings_count > 0 else "info"
    )

    mid = await memory.save_finding("analyze_completed", severity, result, target)
    await memory.save_analysis_run(
        "analyze", target, {"status": result.get("status")},
        duration_ms, health_score, findings_count
    )

    return result, mid


async def execute_check_circular(target: str = ".", request_id: Optional[str] = None) -> Tuple[dict, int]:
    """Detección de dependencias circulares. Retorna (raw_result, memory_id)."""
    start = time.monotonic()
    result = await call_core("check-circular", target=target, request_id=request_id)
    duration_ms = int((time.monotonic() - start) * 1000)

    res_data = result.get("result", {})
    has_cycles = res_data.get("has_cycles", False)
    cycles_count = res_data.get("cycles_count", 0)

    severity = "critical" if has_cycles else "info"

    mid = await memory.save_finding("circular_check_completed", severity, result, target)
    await memory.save_analysis_run(
        "check-circular", target, {"status": result.get("status"), "has_cycles": has_cycles},
        duration_ms, None, cycles_count
    )

    return result, mid


async def execute_deep_analysis(target: str = ".", request_id: Optional[str] = None) -> Tuple[dict, int]:
    """Análisis profundo de arquitectura. Retorna (raw_result, memory_id)."""
    start = time.monotonic()
    result = await call_core("deep-analysis", target=target, request_id=request_id)
    duration_ms = int((time.monotonic() - start) * 1000)

    severity = "error" if result.get("status") == "error" else "info"

    mid = await memory.save_finding("deep_analysis_completed", severity, result, target)
    await memory.save_analysis_run(
        "deep-analysis", target, {"status": result.get("status")},
        duration_ms
    )

    return result, mid


async def execute_full_report(target: str = ".", request_id: Optional[str] = None) -> Tuple[dict, int]:
    """Reporte completo. Retorna (raw_result, memory_id)."""
    start = time.monotonic()
    result = await call_core("full-report", target=target, request_id=request_id)
    duration_ms = int((time.monotonic() - start) * 1000)

    severity = "error" if result.get("status") == "error" else "info"

    mid = await memory.save_finding("full_report_completed", severity, result, target)
    await memory.save_analysis_run(
        "full-report", target, {"status": result.get("status")},
        duration_ms
    )

    return result, mid


async def execute_validate_config(target: str = ".", request_id: Optional[str] = None) -> Tuple[dict, int]:
    """Validación de configuración. Retorna (raw_result, memory_id)."""
    result = await call_core("validate-config", target=target, request_id=request_id)

    severity = "error" if result.get("status") == "error" else "info"

    mid = await memory.save_finding("config_validated", severity, result, target)
    await memory.save_analysis_run(
        "validate-config", target, {"status": result.get("status")}
    )

    return result, mid


async def execute_analyze_stale(target: str = ".", request_id: Optional[str] = None) -> Tuple[dict, int]:
    """Análisis de archivos stale. Retorna (raw_result, memory_id)."""
    start = time.monotonic()
    result = await call_core("analyze-stale", target=target, request_id=request_id)
    duration_ms = int((time.monotonic() - start) * 1000)

    severity = "error" if result.get("status") == "error" else "warning"

    mid = await memory.save_finding("stale_analysis_completed", severity, result, target)
    await memory.save_analysis_run(
        "analyze-stale", target, {"status": result.get("status")},
        duration_ms
    )

    return result, mid


# ──────────────────────────────────────────────
# Mapa de acciones (usado en bridge)
# ──────────────────────────────────────────────

ACTION_MAP = {
    "lint": execute_lint,
    "analyze": execute_analyze,
    "check-circular": execute_check_circular,
    "check_circular": execute_check_circular,
    "deep-analysis": execute_deep_analysis,
    "deep_analysis": execute_deep_analysis,
    "full-report": execute_full_report,
    "full_report": execute_full_report,
    "validate-config": execute_validate_config,
    "validate_config": execute_validate_config,
    "analyze-stale": execute_analyze_stale,
    "analyze_stale": execute_analyze_stale,
}
