//! Pre-built architect.json templates for popular frameworks and patterns.

use crate::config::{ConfigFile, Framework};

mod django;
mod express;
mod helpers;
mod nestjs;
mod nextjs;

/// Human-readable pattern name shown in the selection menu.
pub struct PatternOption {
    pub label: &'static str,
    pub description: &'static str,
    pub pattern: &'static str, // internal key passed to get_template
}

/// Returns the pattern options available for a given framework.
pub fn patterns_for_framework(framework: &Framework) -> Vec<PatternOption> {
    match framework {
        Framework::NestJS => vec![
            PatternOption {
                label: "Hexagonal",
                description: "domain/ application/ infrastructure/",
                pattern: "hexagonal",
            },
            PatternOption {
                label: "Clean Architecture",
                description: "entities/ use-cases/ adapters/ frameworks/",
                pattern: "clean",
            },
            PatternOption {
                label: "Layered",
                description: "controllers/ services/ repositories/",
                pattern: "layered",
            },
        ],
        Framework::React => vec![
            PatternOption {
                label: "Feature-based",
                description: "features/ con colocacion por dominio",
                pattern: "feature-based",
            },
            PatternOption {
                label: "Layered",
                description: "components/ lib/ utils/",
                pattern: "layered",
            },
        ],
        Framework::Express => vec![
            PatternOption {
                label: "MVC",
                description: "routes/ controllers/ models/ middleware/",
                pattern: "mvc",
            },
            PatternOption {
                label: "Hexagonal",
                description: "domain/ application/ infrastructure/",
                pattern: "hexagonal",
            },
            PatternOption {
                label: "Feature-based",
                description: "features/ con colocacion",
                pattern: "feature-based",
            },
        ],
        Framework::Django => vec![
            PatternOption {
                label: "MVT (Django standard)",
                description: "models/ views/ templates/ por app",
                pattern: "mvt",
            },
            PatternOption {
                label: "Service Layer",
                description: "models/ views/ services/ repositories/",
                pattern: "service-layer",
            },
        ],
        _ => vec![
            PatternOption {
                label: "MVC",
                description: "controllers/ services/ models/",
                pattern: "mvc",
            },
            PatternOption {
                label: "Hexagonal",
                description: "domain/ application/ infrastructure/",
                pattern: "hexagonal",
            },
        ],
    }
}

/// Retrieve the ConfigFile for a given framework + pattern combination.
/// Returns None if the combination is not supported.
pub fn get_template(framework: &Framework, pattern: &str) -> Option<ConfigFile> {
    match framework {
        Framework::NestJS => nestjs::get(pattern),
        Framework::React => nextjs::get(pattern),
        Framework::Express => express::get(pattern),
        Framework::Django => django::get(pattern),
        _ => None,
    }
}
