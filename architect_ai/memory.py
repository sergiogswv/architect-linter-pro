"""
memory.py — Base de datos SQLite para la memoria persistente de Architect.

Persiste:
  - Historial de análisis de arquitectura (violaciones, health scores)
  - Decisiones del usuario (fixed / ignored / accepted)
  - Perfiles de archivos por complejidad y violaciones
  - Patrones de arquitectura detectados
  - Historial de iteraciones de fix automático
"""

import aiosqlite
import json
from datetime import datetime, timezone
from typing import Optional, List, Dict, Any
from .settings import settings


DB_PATH = settings.architect_db_path

SCHEMA = """
-- Hallazgos de análisis de arquitectura
CREATE TABLE IF NOT EXISTS findings (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    created_at  TEXT    NOT NULL,
    event_type  TEXT    NOT NULL,          -- 'lint_completed', 'violation_found', 'fix_applied', etc.
    severity    TEXT    NOT NULL,          -- 'info' | 'warning' | 'error' | 'critical'
    target      TEXT,                      -- archivo o proyecto analizado
    payload     TEXT    NOT NULL,          -- JSON del resultado completo
    decision    TEXT    DEFAULT NULL,      -- 'fixed' | 'ignored' | 'accepted' | 'pending'
    decision_at TEXT    DEFAULT NULL,
    fix_attempts INTEGER DEFAULT 0         -- Número de intentos de auto-fix
);

-- Perfiles de archivos (tracking de violaciones recurrentes)
CREATE TABLE IF NOT EXISTS file_profiles (
    file_path   TEXT    PRIMARY KEY,
    last_seen   TEXT    NOT NULL,
    total_analyses INTEGER NOT NULL DEFAULT 0,
    violation_count INTEGER NOT NULL DEFAULT 0,
    critical_violations INTEGER NOT NULL DEFAULT 0,
    health_score_avg REAL DEFAULT 100.0,
    pattern_type TEXT DEFAULT NULL,      -- 'MVC', 'Clean', 'Hexagonal', etc.
    notes       TEXT   DEFAULT NULL
);

-- Historial de análisis ejecutados
CREATE TABLE IF NOT EXISTS analysis_runs (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    run_at      TEXT    NOT NULL,
    action      TEXT    NOT NULL,          -- 'lint', 'deep-analysis', 'check-circular', etc.
    target      TEXT,
    summary     TEXT    NOT NULL,          -- JSON resumido del resultado
    duration_ms INTEGER DEFAULT NULL,
    health_score INTEGER DEFAULT NULL,
    violations_count INTEGER DEFAULT 0
);

-- Patrones de arquitectura detectados/aprendidos
CREATE TABLE IF NOT EXISTS learned_patterns (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    detected_at TEXT    NOT NULL,
    pattern_name TEXT   NOT NULL,          -- 'MVC', 'Repository', 'Service', etc.
    file_path   TEXT,
    confidence  REAL    DEFAULT 0.0,       -- 0.0 a 1.0
    context     TEXT,                      -- JSON con detalles del contexto
    validated   INTEGER DEFAULT 0          -- 1 si el usuario confirmó el patrón
);

-- Iteraciones de auto-fix
CREATE TABLE IF NOT EXISTS fix_iterations (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    finding_id  INTEGER NOT NULL,
    attempt_at  TEXT    NOT NULL,
    fix_type    TEXT    NOT NULL,          -- 'refactor', 'move', 'create_interface', etc.
    description TEXT,
    success     INTEGER DEFAULT 0,         -- 1 si el fix funcionó
    error_msg   TEXT    DEFAULT NULL,
    FOREIGN KEY (finding_id) REFERENCES findings(id)
);

-- Deuda técnica (del schema anterior, mantenido para compatibilidad)
CREATE TABLE IF NOT EXISTS technical_debt (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    file_path TEXT NOT NULL,
    issue_type TEXT, -- coupling, complexity, stale
    description TEXT,
    severity TEXT,
    last_seen TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Decisiones de diseño (del schema anterior)
CREATE TABLE IF NOT EXISTS design_rationales (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    decision_summary TEXT NOT NULL,
    reasoning TEXT,
    applied_on TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
"""


async def init_db():
    """Inicializa el schema de la base de datos si no existe."""
    async with aiosqlite.connect(DB_PATH) as db:
        await db.executescript(SCHEMA)
        await db.commit()


async def save_finding(
    event_type: str,
    severity: str,
    payload: dict,
    target: Optional[str] = None,
) -> int:
    """Guarda un hallazgo de arquitectura y retorna su ID."""
    now = datetime.now(timezone.utc).isoformat()
    async with aiosqlite.connect(DB_PATH) as db:
        cursor = await db.execute(
            """
            INSERT INTO findings (created_at, event_type, severity, target, payload)
            VALUES (?, ?, ?, ?, ?)
            """,
            (now, event_type, severity, target, json.dumps(payload)),
        )
        await db.commit()
        return cursor.lastrowid


async def set_finding_decision(finding_id: int, decision: str):
    """Actualiza la decisión del usuario sobre un hallazgo (fixed, ignored, accepted)."""
    now = datetime.now(timezone.utc).isoformat()
    async with aiosqlite.connect(DB_PATH) as db:
        await db.execute(
            "UPDATE findings SET decision = ?, decision_at = ? WHERE id = ?",
            (decision, now, finding_id),
        )
        await db.commit()


async def increment_fix_attempts(finding_id: int) -> int:
    """Incrementa el contador de intentos de fix para un hallazgo. Retorna el nuevo valor."""
    async with aiosqlite.connect(DB_PATH) as db:
        await db.execute(
            "UPDATE findings SET fix_attempts = fix_attempts + 1 WHERE id = ?",
            (finding_id,)
        )
        await db.commit()

        # Retornar el nuevo valor
        async with db.execute(
            "SELECT fix_attempts FROM findings WHERE id = ?", (finding_id,)
        ) as cursor:
            row = await cursor.fetchone()
            return row[0] if row else 0


async def update_file_profile(
    file_path: str,
    health_score: float,
    violation_count: int,
    has_critical: bool = False,
    pattern_type: Optional[str] = None,
):
    """Actualiza el perfil acumulado de un archivo."""
    now = datetime.now(timezone.utc).isoformat()
    is_critical = 1 if has_critical else 0

    async with aiosqlite.connect(DB_PATH) as db:
        # Calcular promedio ponderado de health score
        await db.execute(
            """
            INSERT INTO file_profiles
                (file_path, last_seen, total_analyses, violation_count, critical_violations, health_score_avg, pattern_type)
            VALUES (?, ?, 1, ?, ?, ?, ?)
            ON CONFLICT(file_path) DO UPDATE SET
                last_seen = excluded.last_seen,
                total_analyses = total_analyses + 1,
                violation_count = violation_count + excluded.violation_count,
                critical_violations = critical_violations + excluded.critical_violations,
                health_score_avg = (health_score_avg * total_analyses + excluded.health_score_avg) / (total_analyses + 1),
                pattern_type = COALESCE(NULLIF(excluded.pattern_type, ''), file_profiles.pattern_type)
            """,
            (file_path, now, violation_count, is_critical, health_score, pattern_type or ""),
        )
        await db.commit()


async def get_file_profile(file_path: str) -> Optional[dict]:
    """Devuelve el perfil histórico de un archivo, o None si no existe."""
    async with aiosqlite.connect(DB_PATH) as db:
        db.row_factory = aiosqlite.Row
        async with db.execute(
            "SELECT * FROM file_profiles WHERE file_path = ?", (file_path,)
        ) as cursor:
            row = await cursor.fetchone()
            return dict(row) if row else None


async def get_hot_files(limit: int = 10) -> list[dict]:
    """
    Devuelve los archivos con más violaciones acumuladas.
    Útil para contextualizar al agente ADK sobre problemas crónicos.
    """
    async with aiosqlite.connect(DB_PATH) as db:
        db.row_factory = aiosqlite.Row
        async with db.execute(
            """
            SELECT file_path, total_analyses, violation_count, critical_violations,
                   health_score_avg, last_seen, pattern_type
            FROM file_profiles
            ORDER BY critical_violations DESC, violation_count DESC
            LIMIT ?
            """,
            (limit,),
        ) as cursor:
            rows = await cursor.fetchall()
            return [dict(r) for r in rows]


async def save_analysis_run(
    action: str,
    target: Optional[str],
    summary: dict,
    duration_ms: Optional[int] = None,
    health_score: Optional[int] = None,
    violations_count: int = 0,
) -> int:
    """Registra cada ejecución de análisis para tracking histórico."""
    now = datetime.now(timezone.utc).isoformat()
    async with aiosqlite.connect(DB_PATH) as db:
        cursor = await db.execute(
            """
            INSERT INTO analysis_runs (run_at, action, target, summary, duration_ms, health_score, violations_count)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            """,
            (now, action, target, json.dumps(summary), duration_ms, health_score, violations_count),
        )
        await db.commit()
        return cursor.lastrowid


async def save_fix_iteration(
    finding_id: int,
    fix_type: str,
    description: str,
    success: bool = False,
    error_msg: Optional[str] = None,
) -> int:
    """Guarda un intento de fix automático."""
    now = datetime.now(timezone.utc).isoformat()
    async with aiosqlite.connect(DB_PATH) as db:
        cursor = await db.execute(
            """
            INSERT INTO fix_iterations (finding_id, attempt_at, fix_type, description, success, error_msg)
            VALUES (?, ?, ?, ?, ?, ?)
            """,
            (finding_id, now, fix_type, description, 1 if success else 0, error_msg),
        )
        await db.commit()
        return cursor.lastrowid


async def get_fix_history(finding_id: int) -> list[dict]:
    """Obtiene el historial de intentos de fix para un hallazgo específico."""
    async with aiosqlite.connect(DB_PATH) as db:
        db.row_factory = aiosqlite.Row
        async with db.execute(
            """
            SELECT * FROM fix_iterations
            WHERE finding_id = ?
            ORDER BY attempt_at DESC
            """,
            (finding_id,),
        ) as cursor:
            rows = await cursor.fetchall()
            return [dict(r) for r in rows]


async def get_recent_findings(limit: int = 20, severity_filter: Optional[str] = None) -> list[dict]:
    """
    Retorna los hallazgos más recientes, opcionalmente filtrados por severity.
    """
    async with aiosqlite.connect(DB_PATH) as db:
        db.row_factory = aiosqlite.Row
        query = "SELECT * FROM findings"
        params: list = []
        if severity_filter:
            query += " WHERE severity = ?"
            params.append(severity_filter)
        query += " ORDER BY created_at DESC LIMIT ?"
        params.append(limit)
        async with db.execute(query, params) as cursor:
            rows = await cursor.fetchall()
            result = []
            for row in rows:
                d = dict(row)
                d["payload"] = json.loads(d["payload"] or "{}")
                result.append(d)
            return result


async def save_learned_pattern(
    pattern_name: str,
    file_path: Optional[str],
    confidence: float,
    context: dict,
    validated: bool = False,
) -> int:
    """Guarda un patrón de arquitectura detectado/aprendido."""
    now = datetime.now(timezone.utc).isoformat()
    async with aiosqlite.connect(DB_PATH) as db:
        cursor = await db.execute(
            """
            INSERT INTO learned_patterns (detected_at, pattern_name, file_path, confidence, context, validated)
            VALUES (?, ?, ?, ?, ?, ?)
            """,
            (now, pattern_name, file_path, confidence, json.dumps(context), 1 if validated else 0),
        )
        await db.commit()
        return cursor.lastrowid


async def get_learned_patterns(pattern_name: Optional[str] = None, limit: int = 20) -> list[dict]:
    """Obtiene patrones aprendidos, opcionalmente filtrados por nombre."""
    async with aiosqlite.connect(DB_PATH) as db:
        db.row_factory = aiosqlite.Row
        query = "SELECT * FROM learned_patterns"
        params: list = []
        if pattern_name:
            query += " WHERE pattern_name = ?"
            params.append(pattern_name)
        query += " ORDER BY confidence DESC, detected_at DESC LIMIT ?"
        params.append(limit)
        async with db.execute(query, params) as cursor:
            rows = await cursor.fetchall()
            result = []
            for row in rows:
                d = dict(row)
                d["context"] = json.loads(d["context"] or "{}")
                result.append(d)
            return result


async def get_architecture_context(target: Optional[str] = None) -> dict:
    """
    Recupera el contexto completo de arquitectura para análisis:
    - Archivos problemáticos (hot files)
    - Hallazgos recientes
    - Patrones aprendidos
    - Health score promedio del proyecto
    """
    hot_files = await get_hot_files(10)
    recent_findings = await get_recent_findings(10)
    learned_patterns = await get_learned_patterns(limit=10)

    # Calcular health score promedio general
    async with aiosqlite.connect(DB_PATH) as db:
        async with db.execute(
            "SELECT AVG(health_score) FROM analysis_runs WHERE run_at > datetime('now', '-7 days')"
        ) as cursor:
            row = await cursor.fetchone()
            avg_health = row[0] if row and row[0] else 100.0

    return {
        "hot_files": hot_files,
        "recent_findings": recent_findings,
        "learned_patterns": learned_patterns,
        "average_health_score_7d": round(avg_health, 2) if avg_health else 100.0,
        "target": target,
    }


# Funciones legacy para compatibilidad con código existente
async def register_pattern(file_path: str, pattern_type: str, summary: str):
    """Legacy: registra un patrón detectado."""
    await save_learned_pattern(
        pattern_name=pattern_type,
        file_path=file_path,
        confidence=0.8,
        context={"summary": summary},
        validated=False
    )


async def record_debt(file_path: str, issue_type: str, description: str, severity: str):
    """Legacy: registra deuda técnica."""
    now = datetime.now(timezone.utc).isoformat()
    async with aiosqlite.connect(DB_PATH) as db:
        await db.execute(
            """
            INSERT INTO technical_debt (file_path, issue_type, description, severity, last_seen)
            VALUES (?, ?, ?, ?, ?)
            """,
            (file_path, issue_type, description, severity, now)
        )
        await db.commit()


async def get_design_context(file_path: Optional[str] = None) -> Dict[str, Any]:
    """Legacy: recupera contexto de diseño."""
    return await get_architecture_context(target=file_path)
