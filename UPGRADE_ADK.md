# 🏛️ Plan de Upgrade: ARCHITECT — Consultor de Diseño con ADK

> **Objetivo:** Evolucionar a Architect de un linter estático (Rust/Tree-sitter) a un **Consultor de Arquitectura Senior** que proponga patrones y detecte desviaciones estructurales con memoria de diseño, usando el **Google Agent Development Kit (ADK)**.

---

## 🏗️ Nueva Arquitectura (ADK Architecture Consultant)

Architect se convertirá en un Agente que **Razona sobre el diseño sistémico** del proyecto.

- **Framework:** `google-adk` (Python Sidecar)
- **Motor de Análisis:** `architect-core` (Rust - existente para parsing Tree-sitter)
- **Rol:** Agente de Diseño, Patrones y Coherencia Estructural.

### 1. Sistema de Memoria Persistente (Design Memory)
Architect recordará las decisiones de diseño tomadas para todo el proyecto.
- **Registro de Patrones (Long-term):** Almacenará qué patrones de diseño (`Repository`, `Factory`, `Middleware`) se han usado en el proyecto.
- **Grafo de Dependencias Histórico:** 
    - **Vector DB de Reglas IA:** Guardará las reglas generadas dinámicamente (`architect.json` dinámico) para aplicarlas en nuevos módulos.
    - **Lecciones de Refactorización:** Recordar qué archivos fueron refactorizados recientemente y por qué (acoplamiento, complejidad).

### 2. Conversión a Agente ADK
```python
from google.adk.agents import LlmAgent
from google.adk.tools import FunctionTool

# Architect se define como el Consultor de Diseño
architect_agent = LlmAgent(
    name="Skrymir-Architect",
    model="gemini-2.0-flash",
    instruction="""
        Eres el Agente Architect de Skrymir Suite. 
        Tu misión es asegurar la coherencia arquitectónica del código.
        Tienes acceso a la memoria de decisiones de diseño del proyecto.
        Cuando un archivo se analiza, detecta si rompe los patrones establecidos.
        Si detectas código 'stale' o 'God Objects', propone un plan de refactorización basado en tu conocimiento histórico.
    """,
    tools=[...], # Wrappers del servidor Rust (lint, deep-analysis, check-circular)
    memory=DesignMemoryService() # Persistencia de patrones y deuda técnica
)
```

---

## 🛠️ Pasos de Implementación

### Fase 1: Creación del Agente de Razonamiento (P0)
1. Iniciar un módulo Python `architect_ai/`.
2. Crear herramientas que llamen a los endpoints de `localhost:4002`.

### Fase 2: Implementación de la Memoria de Patrones (P0)
- Guardar en **SQLite/ChromaDB** una representación semántica de cada clase/módulo analizado para detectar inconsistencias de naming y estructura a lo largo de todo el proyecto.
- Permitirá a Architect decir: "Este nuevo servicio `OrderService` no sigue el patrón de los demás servicios inyectados en el controlador".

### Fase 3: Generación Dinámica de Reglas (P1)
- Architect usará el LLM para actualizar automáticamente `architect.json` conforme el proyecto evoluciona, guardando el "Racional de la Regla" en su memoria de larga duración.

### Fase 4: Consultoría Proactiva (P1)
- Integrar con Cerebro para que, ante cambios masivos detectados por Sentinel, Architect proponga automáticamente un diagrama de la nueva estructura necesaria.

---

## ✅ Beneficios del Upgrade
- **Coherencia Global:** Asegura que todo el equipo (o el usuario solo) mantenga el mismo estándar de diseño.
- **Documentación Viva:** Architect puede generar explicaciones de "por qué este diseño" basadas en su memoria histórica.
- **Detección Temprana de Acoplamiento:** Al tener el historial de commits y cambios, puede alertar sobre degradación arquitectónica antes de que sea un problema.
