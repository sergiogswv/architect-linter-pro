/// Architecture Pattern Detection Module
/// Detects common architecture patterns in codebases and provides suggestions

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArchitecturePattern {
    Hexagonal,
    CleanArchitecture,
    LayeredMvc,
    MicroservicesReady,
    MonolithicMud,
}

pub struct PatternDetector;

impl PatternDetector {
    /// Detects the architecture pattern based on file structure
    pub fn detect_pattern(files: &[&str]) -> ArchitecturePattern {
        let has_domain = files.iter().any(|f| f.contains("domain"));
        let has_application = files.iter().any(|f| f.contains("application"));
        let has_infrastructure = files.iter().any(|f| f.contains("infrastructure"));
        let has_controllers = files.iter().any(|f| f.contains("controller"));
        let has_services = files.iter().any(|f| f.contains("service"));
        let has_models = files.iter().any(|f| f.contains("model"));
        let _has_routes = files.iter().any(|f| f.contains("route"));

        // Hexagonal pattern has domain, application, and infrastructure layers
        if has_domain && has_application && has_infrastructure {
            return ArchitecturePattern::Hexagonal;
        }

        // Layered MVC: controllers, services, and models
        if has_controllers && has_services && has_models {
            return ArchitecturePattern::LayeredMvc;
        }

        // Monolithic structures (small number of files)
        if files.len() < 20 {
            return ArchitecturePattern::MonolithicMud;
        }

        // Default to Clean Architecture for larger projects
        ArchitecturePattern::CleanArchitecture
    }

    /// Provides architecture-specific suggestions
    pub fn suggest_pattern(detected: &ArchitecturePattern) -> String {
        match detected {
            ArchitecturePattern::Hexagonal => {
                "Good! Hexagonal pattern detected. Maintain domain/application/infrastructure separation."
                    .to_string()
            }
            ArchitecturePattern::CleanArchitecture => {
                "Consider adopting explicit layer names (domain, application, infrastructure)."
                    .to_string()
            }
            ArchitecturePattern::LayeredMvc => {
                "Standard MVC detected. Ensure models are not bloated with business logic.".to_string()
            }
            ArchitecturePattern::MonolithicMud => {
                "Project is small. Define clear architecture patterns before it grows.".to_string()
            }
            ArchitecturePattern::MicroservicesReady => {
                "Project structure supports microservices. Define clear service boundaries.".to_string()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detects_hexagonal_pattern() {
        let files = vec!["src/domain/user.rs", "src/application/service.rs", "src/infrastructure/db.rs"];
        let pattern = PatternDetector::detect_pattern(&files);
        assert_eq!(pattern, ArchitecturePattern::Hexagonal);
    }

    #[test]
    fn test_detects_mvc_pattern() {
        let files = vec!["src/controllers/user.rs", "src/services/user.rs", "src/models/user.rs"];
        let pattern = PatternDetector::detect_pattern(&files);
        assert_eq!(pattern, ArchitecturePattern::LayeredMvc);
    }

    #[test]
    fn test_detects_monolithic_small_codebase() {
        let files = vec!["src/main.rs", "src/lib.rs"];
        let pattern = PatternDetector::detect_pattern(&files);
        assert_eq!(pattern, ArchitecturePattern::MonolithicMud);
    }

    #[test]
    fn test_suggestion_for_hexagonal_pattern() {
        let suggestion = PatternDetector::suggest_pattern(&ArchitecturePattern::Hexagonal);
        assert!(suggestion.contains("Hexagonal"));
    }

    #[test]
    fn test_suggestion_for_mvc_pattern() {
        let suggestion = PatternDetector::suggest_pattern(&ArchitecturePattern::LayeredMvc);
        assert!(suggestion.contains("MVC"));
    }

    #[test]
    fn test_suggestion_for_monolithic_pattern() {
        let suggestion = PatternDetector::suggest_pattern(&ArchitecturePattern::MonolithicMud);
        assert!(suggestion.contains("small"));
    }

    #[test]
    fn test_suggestion_for_clean_architecture() {
        let suggestion = PatternDetector::suggest_pattern(&ArchitecturePattern::CleanArchitecture);
        assert!(suggestion.contains("explicit layer names"));
    }

    #[test]
    fn test_clean_architecture_default_for_large_projects() {
        let mut files = vec!["src/main.rs"];
        // Add 20+ files to trigger clean architecture detection
        for _ in 0..25 {
            files.push("src/module.rs");
        }
        let pattern = PatternDetector::detect_pattern(&files);
        assert_eq!(pattern, ArchitecturePattern::CleanArchitecture);
    }
}
