"""
bridge.py — Puente entre Cerebro ↔ Architect Core ↔ LLM.

FLUJO por comando:
  1. Cerebro envía OrchestratorCommand { action, target }
  2. El action ya está definido — no necesitamos un LLM para decidir qué ejecutar.
  3. Architect Core (Rust) ejecuta la acción y retorna raw JSON.
  4. La memoria SQLite persiste el resultado.
  5. El LLM (Gemini/Claude/OpenAI) recibe:
       - El resultado crudo
       - El contexto histórico (hot_files, hallazgos críticos recientes, patrones)
     Y produce una síntesis accionable en texto.
  6. El bridge reporta a Cerebro:
       - POST /api/events con el evento estructurado
       - CommandAck con { status, result: { raw, analysis, memory_id } }

CONTRATO MANTENIDO:
  Input:  OrchestratorCommand { action, target?, options?, request_id? }
  Output: CommandAck { request_id?, status, result?, error? }
"""

import uuid
import re
import httpx
from datetime import datetime, timezone
from typing import Optional

from .settings import settings
from . import memory
from .tools import ACTION_MAP, call_core
from .llm_client import analyze_result


# ──────────────────────────────────────────────
# Reporte al Cerebro
# ──────────────────────────────────────────────

async def report_to_cerebro(event_type: str, severity: str, payload: dict):
    """Envía un AgentEvent al endpoint POST /api/events del Cerebro."""
    event = {
        "id": str(uuid.uuid4()),
        "source": "architect",
        "type": event_type,
        "severity": severity,
        "timestamp": datetime.now(timezone.utc).isoformat(),
        "payload": payload,
    }
    try:
        async with httpx.AsyncClient(timeout=10.0) as client:
            resp = await client.post(f"{settings.cerebro_url}/api/events", json=event)
            if resp.status_code >= 400:
                print(f"⚠️  [Cerebro] Respuesta inesperada: {resp.status_code}")
            else:
                print(f"✅ [Cerebro] Evento ARCHITECT reportado: {event_type} ({severity})")
    except Exception as exc:
        print(f"⚠️  [Cerebro] No disponible: {exc}")


# ──────────────────────────────────────────────
# Handler principal de comandos
# ──────────────────────────────────────────────

async def _log_to_cerebro(message: str, level: str = "info"):
    """Envía un log como evento a Cerebro para visualización en Dashboard."""
    await report_to_cerebro(
        event_type=f"architect_log_{level}",
        severity=level,
        payload={
            "message": message,
            "timestamp": datetime.now(timezone.utc).isoformat(),
            "agent": "architect-adk"
        }
    )


async def handle_command(
    action: str,
    target: Optional[str],
    request_id: Optional[str] = None,
) -> dict:
    """
    Procesa un OrchestratorCommand completo.

    Retorna un CommandAck con:
      - raw:      resultado crudo del Core Rust
      - analysis: síntesis del LLM
      - memory_id: ID del hallazgo guardado en SQLite
    """
    print(f"🏛️ [Architect] Procesando: action='{action}' target='{target}'")
    print(f"🏛️ [Architect] Config: provider={settings.llm_provider}, core={settings.architect_core_url}")

    # ── 1. Status — no requiere Core ni LLM ──────────────────────────
    if action == "status":
        ctx = await memory.get_hot_files(5)
        recent = await memory.get_recent_findings(5)
        patterns = await memory.get_learned_patterns(limit=5)
        result_payload = {
            "agent": "architect-adk",
            "version": "6.0.0",
            "llm_provider": settings.llm_provider,
            "core_url": settings.architect_core_url,
            "hot_files_tracked": len(ctx),
            "recent_findings": len(recent),
            "learned_patterns": len(patterns),
        }
        await report_to_cerebro("architect_status", "info", result_payload)
        return {
            "request_id": request_id,
            "status": "completed",
            "result": result_payload,
            "error": None,
        }

    # ── 2. Acciones desconocidas ──────────────────────────────────────
    executor = ACTION_MAP.get(action)
    if not executor:
        # Intentar pasar directo al Core por si es una acción nueva
        print(f"⚠️  Acción '{action}' no en ACTION_MAP — enviando directo al Core")
        raw = await call_core(action, target=target)
        return {
            "request_id": request_id,
            "status": raw.get("status", "error"),
            "result": raw.get("result"),
            "error": raw.get("error"),
        }

    # ── 3. Ejecutar en Core Rust ──────────────────────────────────────
    await _log_to_cerebro(f"🔄 Ejecutando {action} en Core Rust...", "info")
    try:
        raw_result, memory_id = await executor(target or ".", request_id)
        res_data = raw_result.get("result", {})
        findings = res_data.get("findings_count", 0)
        health = res_data.get("health_score", "N/A")
        await _log_to_cerebro(f"✅ Core completado: {findings} violaciones, health={health}", "info")
    except Exception as exc:
        error_msg = f"Error ejecutando '{action}': {exc}"
        print(f"❌ {error_msg}")
        await _log_to_cerebro(error_msg, "error")
        await report_to_cerebro(f"architect_{action}_error", "error", {"error": error_msg, "action": action})
        return {"request_id": request_id, "status": "error", "result": None, "error": error_msg}

    # ── 4. Recuperar contexto histórico de memoria ────────────────────
    mem_context = None
    try:
        mem_context = await memory.get_architecture_context(target)
    except Exception:
        pass  # La memoria falla silenciosamente, no bloquea el análisis

    # ── 5. LLM analiza el resultado crudo + contexto ──────────────────
    analysis = ""
    try:
        await _log_to_cerebro(f"🤖 Llamando a LLM ({settings.llm_provider}) para análisis...", "info")
        print(f"🤖 [Architect] Llamando a LLM ({settings.llm_provider}) para análisis...")
        print(f"📝 [Architect] Raw result tiene {len(str(raw_result))} chars")
        print(f"📝 [Architect] Memory context: {mem_context is not None}")

        analysis = await analyze_result(
            action=action,
            raw_result=raw_result,
            memory_context=mem_context,
        )
        print(f"✅ [Architect] LLM respondió: {len(analysis)} chars")
        await _log_to_cerebro(f"✅ LLM respondió: {len(analysis)} caracteres", "info")
        # Enviar preview del análisis
        preview = analysis[:200] + "..." if len(analysis) > 200 else analysis
        await _log_to_cerebro(f"📝 Preview: {preview}", "info")
    except Exception as exc:
        analysis = f"[Análisis LLM no disponible: {exc}]"
        print(f"⚠️  [Architect] LLM falló: {exc}")
        await _log_to_cerebro(f"⚠️ LLM falló: {exc}", "warning")

    # GARANTÍA: summary nunca vacío (requerido por Cerebro Proactivo)
    if not analysis or not analysis.strip():
        f_count = len(raw_result.get("result", {}).get("findings", []))
        analysis = f"Análisis de arquitectura completado para {target}. {f_count} hallazgos encontrados."
        print(f"⚠️  [Architect] summary vacío — usando fallback de contenido")

    # ── 6. Determinar severidad final para el evento ──────────────────
    severity = _infer_severity(action, raw_result, analysis)

    # ── 7. Actualizar decisión del hallazgo si es necesario ────────────
    try:
        if severity in ["error", "critical"]:
            await memory.set_finding_decision(memory_id, "pending")
    except Exception:
        pass

    # ── 8. Extraer info para Auto-Fix ──────────────────────────────────
    # Extraer finding/recomendación del LLM para el Auto-Fix
    res_data = raw_result.get("result", {})
    findings_list = res_data.get("findings", [])
    health_score = res_data.get("health_score", 100)

    # Construir descripción del problema para Cerebro
    finding_desc = f"Análisis {action}: {len(findings_list)} violaciones encontradas, health score: {health_score}"
    if raw_result.get("status") == "error":
        finding_desc = f"Error en análisis {action}: {raw_result.get('error', 'Unknown error')}"

    # Extraer primer archivo afectado si existe
    affected_file = None
    if findings_list and len(findings_list) > 0:
        first_finding = findings_list[0]
        if isinstance(first_finding, dict):
            affected_file = first_finding.get("file") or first_finding.get("path")
        elif isinstance(first_finding, str):
            # Parsear string tipo "Error: 'path' (file:line)"
            import re
            match = re.search(r'\(([^)]+)\)', first_finding)
            if match:
                affected_file = match.group(1)

    # ── 9. Reportar a Cerebro ─────────────────────────────────────────
    await report_to_cerebro(
        event_type=f"architect_{action.replace('-', '_')}_completed",
        severity=severity,
        payload={
            "action": action,
            "target": target,
            "summary": analysis,                # Siempre tiene contenido (garantía aplicada arriba)
            "memory_id": memory_id,
            "raw_status": raw_result.get("status"),
            # Campos estandarizados para Cerebro Proactivo
            "finding": finding_desc,
            "recommendation": analysis[:2000] if analysis else "Revisar violaciones de arquitectura detectadas",
            "file": affected_file or target,
            "issues_count": len(findings_list),   # ✅ Estandarizado
            "findings_count": len(findings_list),  # Backward compat
            "health_score": health_score,
        },
    )

    # ── 9. Retornar CommandAck completo ───────────────────────────────
    return {
        "request_id": request_id,
        "status": "completed",
        "result": {
            "action": action,
            "target": target,
            "raw": raw_result,          # Resultado crudo del Core Rust (para Dashboard)
            "analysis": analysis,       # Síntesis del LLM (para Telegram/Dashboard)
            "memory_id": memory_id,
            "severity": severity,
        },
        "error": None
    }


def _infer_severity(action: str, raw_result: dict, analysis: str) -> str:
    """
    Determina la severidad del evento reportado a Cerebro.
    Prioridad: error del Core → palabras clave en análisis → action por defecto.
    """
    if raw_result.get("status") == "error":
        return "error"

    res = raw_result.get("result", {})
    health_score = res.get("health_score", 100)
    has_cycles = res.get("has_cycles", False)
    findings_count = res.get("findings_count", 0)

    # Reglas de severidad específicas de Architect
    if has_cycles:
        return "critical"
    if health_score < 60:
        return "critical"
    if health_score < 80:
        return "error"
    if findings_count > 0:
        return "warning"

    analysis_lower = analysis.lower()
    if any(w in analysis_lower for w in ("crítico", "crítica", "critical", "circular", "dependencia circular")):
        return "critical"
    if any(w in analysis_lower for w in ("alto riesgo", "violación", "advertencia", "warning")):
        return "error"

    return "info"
