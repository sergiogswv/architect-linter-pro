"""
main.py — Servidor FastAPI para el Agente ADK de Architect.
Mismo contrato que Warden para interoperabilidad en Cerebro.
"""

import sys
if sys.platform == "win32":
    sys.stdout.reconfigure(encoding="utf-8")
    sys.stderr.reconfigure(encoding="utf-8")

import uvicorn
from fastapi import FastAPI
from pydantic import BaseModel
from typing import Optional, Dict, Any

from .memory import init_db
from .bridge import handle_command, report_to_cerebro
from .settings import settings


app = FastAPI(
    title="Architect ADK Agent",
    description="Agente de arquitectura senior con memoria de diseño (Google ADK Mode)",
    version="6.0.0",
)


# ──────────────────────────────────────────────
# Modelos Skrymir (Standard Contract)
# ────────────────��─────────────────────────────

class OrchestratorCommand(BaseModel):
    action: str
    target: Optional[str] = None
    options: Optional[Dict[str, Any]] = None
    request_id: Optional[str] = None


class CommandAck(BaseModel):
    request_id: Optional[str] = None
    status: str
    result: Optional[Dict[str, Any]] = None
    error: Optional[str] = None


# ──────────────────────────────────────────────
# Eventos y Endpoints
# ──────────────────────────────────────────────

@app.on_event("startup")
async def on_startup():
    """Inicializa la DB de diseño y notifica a Cerebro."""
    await init_db()
    print(f"🏛️ Architect ADK Agent iniciado en puerto {settings.architect_adk_port}")
    await report_to_cerebro(
        event_type="architect_adk_ready",
        severity="info",
        payload={
            "message": "Architect ADK listo — modo consultoría estructural activo",
            "version": "6.0.0",
            "port": settings.architect_adk_port,
            "llm_provider": settings.llm_provider,
        },
    )


@app.post("/command", response_model=CommandAck)
async def command_endpoint(cmd: OrchestratorCommand) -> CommandAck:
    """
    Recibe un comando desde Cerebro y lo delega al bridge.
    """
    ack = await handle_command(
        action=cmd.action,
        target=cmd.target,
        request_id=cmd.request_id,
    )
    return CommandAck(**ack)


@app.get("/health")
async def health():
    """Health check básico."""
    return {
        "status": "ok",
        "agent": "architect-adk",
        "version": "6.0.0",
        "llm_provider": settings.llm_provider,
    }


@app.get("/memory/context")
async def get_memory_context():
    """Endpoint de inspección de memoria histórica de arquitectura."""
    from .memory import get_architecture_context, get_hot_files, get_recent_findings, get_learned_patterns

    ctx = await get_architecture_context()
    return {
        "ok": True,
        "data": ctx,
    }


@app.get("/memory/hot-files")
async def get_hot_files_endpoint(limit: int = 10):
    """Retorna los archivos con más problemas históricos."""
    from .memory import get_hot_files
    files = await get_hot_files(limit)
    return {"ok": True, "data": files}


@app.get("/memory/recent-findings")
async def get_recent_findings_endpoint(limit: int = 20, severity: Optional[str] = None):
    """Retorna los hallazgos más recientes."""
    from .memory import get_recent_findings
    findings = await get_recent_findings(limit, severity)
    return {"ok": True, "data": findings}


@app.get("/memory/learned-patterns")
async def get_learned_patterns_endpoint(limit: int = 20):
    """Retorna los patrones de arquitectura aprendidos."""
    from .memory import get_learned_patterns
    patterns = await get_learned_patterns(limit=limit)
    return {"ok": True, "data": patterns}


# ──────────────────────────────────────────────
# Iniciar Servidor (uvicorn)
# ──────────────────────────────────────────────

def start():
    uvicorn.run(
        "architect_ai.main:app",
        host="0.0.0.0",
        port=settings.architect_adk_port,
        reload=False,
    )


if __name__ == "__main__":
    start()
