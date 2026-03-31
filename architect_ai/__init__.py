"""
architect_ai — Agente ADK de Architect para Skrymir Suite.

Este paquete implementa el Consultor de Arquitectura con:
- Memoria persistente (SQLite)
- Análisis multi-proveedor LLM (Gemini, Claude, OpenAI, Ollama)
- Tracking de violaciones y health scores
- Iteraciones de auto-fix
- Patrones de arquitectura aprendidos
"""

__version__ = "6.0.0"
__agent__ = "architect-adk"

from .bridge import handle_command, report_to_cerebro
from .tools import ACTION_MAP
from .memory import init_db, get_architecture_context
from .llm_client import analyze_result, suggest_fix
from .settings import settings

__all__ = [
    "handle_command",
    "report_to_cerebro",
    "ACTION_MAP",
    "init_db",
    "get_architecture_context",
    "analyze_result",
    "suggest_fix",
    "settings",
]
