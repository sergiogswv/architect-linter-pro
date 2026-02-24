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

#[derive(Debug, Clone, PartialEq, Default)]
pub enum ArchPattern {
    Hexagonal,
    Clean,
    MVC,
    #[default]
    Ninguno,
    Custom(String),
}

impl serde::Serialize for ArchPattern {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            ArchPattern::Hexagonal => serializer.serialize_str("Hexagonal"),
            ArchPattern::Clean => serializer.serialize_str("Clean"),
            ArchPattern::MVC => serializer.serialize_str("MVC"),
            ArchPattern::Ninguno => serializer.serialize_str("Ninguno"),
            ArchPattern::Custom(s) => serializer.serialize_str(s),
        }
    }
}

impl<'de> serde::Deserialize<'de> for ArchPattern {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.to_lowercase().as_str() {
            "hexagonal" => Ok(ArchPattern::Hexagonal),
            "clean" => Ok(ArchPattern::Clean),
            "mvc" => Ok(ArchPattern::MVC),
            "ninguno" | "none" => Ok(ArchPattern::Ninguno),
            _ => Ok(ArchPattern::Custom(s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default, Copy)]
pub enum Severity {
    #[serde(rename = "error")]
    #[default]
    Error,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "info")]
    Info,
}

impl Severity {
    pub fn as_str(&self) -> &str {
        match self {
            Severity::Error => "error",
            Severity::Warning => "warning",
            Severity::Info => "info",
        }
    }

    pub fn emoji(&self) -> &str {
        match self {
            Severity::Error => "❌",
            Severity::Warning => "⚠️",
            Severity::Info => "ℹ️",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ForbiddenRule {
    pub from: String,
    pub to: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<Severity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl ForbiddenRule {
    pub fn get_severity(&self) -> Severity {
        self.severity.unwrap_or(Severity::Error)
    }
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
