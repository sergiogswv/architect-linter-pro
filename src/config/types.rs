#![allow(unused_assignments)]
use miette::{Diagnostic, SourceSpan};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum Framework {
    NestJS,
    React,
    Angular,
    Express,
    // Python frameworks
    Django,
    Flask,
    FastAPI,
    // Go frameworks
    Gin,
    Echo,
    // Java frameworks
    Spring,
    // PHP frameworks
    Laravel,
    Symfony,
    #[default]
    Unknown,
}

impl Framework {
    pub fn as_str(&self) -> &str {
        match self {
            Framework::NestJS => "NestJS",
            Framework::React => "React",
            Framework::Angular => "Angular",
            Framework::Express => "Express",
            // Python
            Framework::Django => "Django",
            Framework::Flask => "Flask",
            Framework::FastAPI => "FastAPI",
            // Go
            Framework::Gin => "Gin",
            Framework::Echo => "Echo",
            // Java
            Framework::Spring => "Spring",
            // PHP
            Framework::Laravel => "Laravel",
            Framework::Symfony => "Symfony",
            Framework::Unknown => "Unknown",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum ArchPattern {
    Hexagonal,
    Clean,
    MVC,
    #[default]
    Ninguno,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ForbiddenRule {
    pub from: String,
    pub to: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AIProvider {
    Claude,
    Gemini,
    OpenAI,
    Groq,
    Ollama,
    Kimi,
    DeepSeek,
}

impl AIProvider {
    pub fn as_str(&self) -> &str {
        match self {
            AIProvider::Claude => "Claude",
            AIProvider::Gemini => "Gemini",
            AIProvider::OpenAI => "OpenAI",
            AIProvider::Groq => "Groq",
            AIProvider::Ollama => "Ollama",
            AIProvider::Kimi => "Kimi",
            AIProvider::DeepSeek => "DeepSeek",
        }
    }
}

/// Configuración de IA para análisis arquitectónico
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIConfig {
    pub name: String,
    pub provider: AIProvider,
    pub api_url: String,
    pub api_key: String,
    pub model: String,
}

impl Default for AIConfig {
    fn default() -> Self {
        Self {
            name: "Default Claude".to_string(),
            provider: AIProvider::Claude,
            api_url: "https://api.anthropic.com".to_string(),
            api_key: String::new(),
            model: "claude-3-7-sonnet-20250219".to_string(),
        }
    }
}

#[derive(Default)]
pub struct LinterContext {
    pub max_lines: usize,
    #[allow(dead_code)]
    pub framework: Framework,
    #[allow(dead_code)]
    pub pattern: ArchPattern,
    pub forbidden_imports: Vec<ForbiddenRule>,
    pub ignored_paths: Vec<String>,
    #[allow(dead_code)]
    pub ai_configs: Vec<AIConfig>,
    pub build_command: Option<String>,
    pub ai_fix_retries: usize,
}

#[derive(Error, Debug, Diagnostic)]
#[error("Violación de Arquitectura")]
#[diagnostic(code(arch::violation), severity(error))]
pub struct ArchError {
    #[source_code]
    pub src: String,
    #[label("{message}")]
    pub span: SourceSpan,
    pub message: String,
}
