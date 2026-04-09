import os
import json
from typing import Literal, Optional
from pydantic import field_validator
from pydantic_settings import BaseSettings


class ArchitectADKSettings(BaseSettings):
    # Architect Core (Rust server)
    architect_core_url: str = "http://localhost:4002"

    # Cerebro (Orquestador Central)
    cerebro_url: str = "http://localhost:8000"

    # Architect ADK Agent port
    architect_adk_port: int = 4012

    # LLM Configuration (Defaults)
    llm_provider: Literal["gemini", "claude", "openai", "ollama"] = "gemini"
    google_api_key: str = ""
    gemini_model: str = "gemini-2.0-flash"

    anthropic_api_key: str = ""
    claude_model: str = "claude-3-5-sonnet-latest"

    openai_api_key: str = ""
    openai_model: str = "gpt-4o"

    # Ollama (Local LLM)
    ollama_base_url: str = "http://localhost:11434"
    ollama_model: str = "qwen3:8b"

    # Persistent Memory
    architect_db_path: str = "./architect_memory.db"

    def sync_from_cerebro_config(self):
        """
        Lee la configuración global de Cerebro (~/.cerebro/global.config.json)
        para mantenerse sincronizado con la config del dashboard.
        """
        # Buscar en el directorio HOME del usuario
        home_dir = os.path.expanduser("~")
        cerebro_config_path = os.path.join(home_dir, ".cerebro", "global.config.json")

        if os.path.exists(cerebro_config_path):
            try:
                with open(cerebro_config_path, "r", encoding="utf-8") as f:
                    data = json.load(f)

                    # Buscar configuración de AI
                    ai_config = data.get("ai", {})
                    provider = ai_config.get("provider", "").lower()

                    if provider:
                        if provider == "gemini":
                            self.llm_provider = "gemini"
                            self.google_api_key = ai_config.get("api_key", self.google_api_key)
                            self.gemini_model = ai_config.get("model", self.gemini_model)
                        elif provider == "claude":
                            self.llm_provider = "claude"
                            self.anthropic_api_key = ai_config.get("api_key", self.anthropic_api_key)
                            self.claude_model = ai_config.get("model", self.claude_model)
                        elif provider == "openai":
                            self.llm_provider = "openai"
                            self.openai_api_key = ai_config.get("api_key", self.openai_api_key)
                            self.openai_model = ai_config.get("model", self.openai_model)
                        elif provider == "ollama":
                            self.llm_provider = "ollama"
                            self.ollama_base_url = ai_config.get("ollama_base_url", self.ollama_base_url)
                            self.ollama_model = ai_config.get("ollama_model", self.ollama_model)

                        print(f"✅ [Architect-ADK] Config sincronizada desde Cerebro: {provider}")
                        return
            except Exception as e:
                print(f"⚠️ [Architect-ADK] No se pudo leer config de Cerebro: {e}")

        # Fallback: Intentar leer .architect.ai.json (config local)
        self.sync_from_core_config()

    def sync_from_core_config(self):
        """
        Intenta leer la configuración de IA que genera el dashboard (.architect.ai.json)
        para mantenerse sincronizado con lo configurado en la UI.
        """
        # Buscar en el directorio HOME de architect (uno arriba de este archivo)
        core_config_path = os.path.join(os.path.dirname(os.path.dirname(__file__)), ".architect.ai.json")
        if os.path.exists(core_config_path):
            try:
                with open(core_config_path, "r", encoding="utf-8") as f:
                    data = json.load(f)
                    # Mapear campos de Architect Core -> ADK Settings
                    provider = data.get("provider", "").lower()
                    if provider == "gemini":
                        self.llm_provider = "gemini"
                        self.google_api_key = data.get("api_key", self.google_api_key)
                        self.gemini_model = data.get("model", self.gemini_model)
                    elif provider == "claude":
                        self.llm_provider = "claude"
                        self.anthropic_api_key = data.get("api_key", self.anthropic_api_key)
                        self.claude_model = data.get("model", self.claude_model)
                    elif provider == "openai":
                        self.llm_provider = "openai"
                        self.openai_api_key = data.get("api_key", self.openai_api_key)
                        self.openai_model = data.get("model", self.openai_model)
                    print(f"✅ [Architect-ADK] Configuración sincronizada desde {core_config_path}")
            except Exception as e:
                print(f"⚠️ [Architect-ADK] No se pudo leer la config de core: {e}")

    @field_validator("llm_provider", mode="before")
    @classmethod
    def validate_llm_provider(cls, v):
        """Si el valor está vacío, retorna el default 'gemini'."""
        if not v or v == "":
            return "gemini"
        return v

    model_config = {
        "env_file": os.path.join(os.path.dirname(__file__), ".env"),
        "env_file_encoding": "utf-8",
        "extra": "ignore"
    }


settings = ArchitectADKSettings()
# Cargar config de Cerebro al arrancar (con fallback a config local)
settings.sync_from_cerebro_config()
