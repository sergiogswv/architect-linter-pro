use crate::config::Framework;
use std::fs;
use std::path::Path;

/// Result of a framework detection attempt
#[derive(Debug, Clone, PartialEq)]
pub struct DetectionResult {
    pub framework: Framework,
    pub confidence: f32,
}

impl DetectionResult {
    /// Create a new detection result
    pub fn new(framework: Framework, confidence: f32) -> Self {
        DetectionResult { framework, confidence }
    }
}

/// Trait for detecting frameworks in a project
pub trait FrameworkDetector {
    /// Detect frameworks in a project directory
    fn detect(&self, project_path: &Path) -> Result<Vec<DetectionResult>, String>;

    /// Get the name of this detector
    fn name(&self) -> &str;
}

/// Detects TypeScript/JavaScript frameworks
pub struct TypeScriptDetector;

impl FrameworkDetector for TypeScriptDetector {
    fn detect(&self, project_path: &Path) -> Result<Vec<DetectionResult>, String> {
        let mut results = Vec::new();
        let pkg_path = project_path.join("package.json");

        match fs::read_to_string(&pkg_path) {
            Ok(content) => {
                // Check for NestJS (highest priority)
                if content.contains("@nestjs/core") || content.contains("@nestjs/") {
                    results.push(DetectionResult::new(Framework::NestJS, 0.95));
                }

                // Check for Next.js
                if content.contains("\"next\"") || content.contains("'next'") {
                    results.push(DetectionResult::new(Framework::NextJS, 0.95));
                }

                // Check for React
                if content.contains("\"react\"") || content.contains("'react'") {
                    results.push(DetectionResult::new(Framework::React, 0.90));
                }

                // Check for Express
                if content.contains("\"express\"") || content.contains("'express'") {
                    results.push(DetectionResult::new(Framework::Express, 0.90));
                }

                // Check for Vue
                if content.contains("\"vue\"") || content.contains("'vue'") {
                    results.push(DetectionResult::new(Framework::Vue, 0.90));
                }

                // Check for Svelte
                if content.contains("\"svelte\"") || content.contains("'svelte'") {
                    results.push(DetectionResult::new(Framework::Svelte, 0.90));
                }

                // Check for Remix
                if content.contains("@remix-run") {
                    results.push(DetectionResult::new(Framework::Remix, 0.95));
                }

                // Check for Solid.js
                if content.contains("\"solid-js\"") || content.contains("'solid-js'") {
                    results.push(DetectionResult::new(Framework::SolidJS, 0.90));
                }

                Ok(results)
            }
            Err(e) => Err(format!("Failed to read package.json: {}", e)),
        }
    }

    fn name(&self) -> &str {
        "TypeScript/JavaScript"
    }
}

/// Detects Python frameworks
pub struct PythonDetector;

impl FrameworkDetector for PythonDetector {
    fn detect(&self, project_path: &Path) -> Result<Vec<DetectionResult>, String> {
        let mut results = Vec::new();

        // Check requirements.txt
        let requirements_path = project_path.join("requirements.txt");
        if let Ok(content) = fs::read_to_string(&requirements_path) {
            let content_lower = content.to_lowercase();
            if content_lower.contains("django") {
                results.push(DetectionResult::new(Framework::Django, 0.95));
            }
            if content_lower.contains("fastapi") {
                results.push(DetectionResult::new(Framework::FastAPI, 0.95));
            }
            if content_lower.contains("flask") {
                results.push(DetectionResult::new(Framework::Flask, 0.95));
            }

            if !results.is_empty() {
                return Ok(results);
            }
        }

        // Check pyproject.toml
        let pyproject_path = project_path.join("pyproject.toml");
        if let Ok(content) = fs::read_to_string(&pyproject_path) {
            let content_lower = content.to_lowercase();
            if content_lower.contains("django") {
                results.push(DetectionResult::new(Framework::Django, 0.95));
            }
            if content_lower.contains("fastapi") {
                results.push(DetectionResult::new(Framework::FastAPI, 0.95));
            }
            if content_lower.contains("flask") {
                results.push(DetectionResult::new(Framework::Flask, 0.95));
            }

            if !results.is_empty() {
                return Ok(results);
            }
        }

        // Check Pipfile
        let pipfile_path = project_path.join("Pipfile");
        if let Ok(content) = fs::read_to_string(&pipfile_path) {
            let content_lower = content.to_lowercase();
            if content_lower.contains("django") {
                results.push(DetectionResult::new(Framework::Django, 0.95));
            }
            if content_lower.contains("fastapi") {
                results.push(DetectionResult::new(Framework::FastAPI, 0.95));
            }
            if content_lower.contains("flask") {
                results.push(DetectionResult::new(Framework::Flask, 0.95));
            }
        }

        Ok(results)
    }

    fn name(&self) -> &str {
        "Python"
    }
}

/// Detects PHP frameworks
pub struct PHPDetector;

impl FrameworkDetector for PHPDetector {
    fn detect(&self, project_path: &Path) -> Result<Vec<DetectionResult>, String> {
        let mut results = Vec::new();
        let composer_path = project_path.join("composer.json");

        match fs::read_to_string(&composer_path) {
            Ok(content) => {
                // Check for Laravel (highest priority)
                if content.contains("laravel/framework") {
                    results.push(DetectionResult::new(Framework::Laravel, 0.95));
                }

                // Check for Symfony
                if content.contains("symfony/framework-bundle")
                    || content.contains("symfony/flex")
                    || content.contains("\"symfony/")
                {
                    results.push(DetectionResult::new(Framework::Symfony, 0.95));
                }

                Ok(results)
            }
            Err(e) => Err(format!("Failed to read composer.json: {}", e)),
        }
    }

    fn name(&self) -> &str {
        "PHP"
    }
}

/// Detects all frameworks in a project by running all available detectors
pub fn detect_all_frameworks(project_path: &Path) -> Result<Vec<DetectionResult>, String> {
    let mut all_results = Vec::new();

    // Create detector instances
    let detectors: Vec<Box<dyn FrameworkDetector>> = vec![
        Box::new(TypeScriptDetector),
        Box::new(PythonDetector),
        Box::new(PHPDetector),
    ];

    // Run all detectors
    for detector in detectors {
        match detector.detect(project_path) {
            Ok(results) => {
                all_results.extend(results);
            }
            Err(_) => {
                // Silently ignore errors from individual detectors
                // They might not apply to this project
            }
        }
    }

    // Sort by confidence (highest first)
    all_results.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());

    Ok(all_results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_typescript_detector_detects_nestjs() {
        let temp_dir = TempDir::new().unwrap();
        let package_json = r#"{"name": "test", "dependencies": {"@nestjs/core": "^10.0.0"}}"#;
        fs::write(temp_dir.path().join("package.json"), package_json).unwrap();

        let detector = TypeScriptDetector;
        let results = detector.detect(temp_dir.path()).unwrap();

        assert!(!results.is_empty());
        assert_eq!(results[0].framework, Framework::NestJS);
        assert_eq!(results[0].confidence, 0.95);
    }

    #[test]
    fn test_python_detector_detects_django() {
        let temp_dir = TempDir::new().unwrap();
        let requirements = "Django==4.2.0\npsycopg2==2.9.0";
        fs::write(
            temp_dir.path().join("requirements.txt"),
            requirements,
        )
        .unwrap();

        let detector = PythonDetector;
        let results = detector.detect(temp_dir.path()).unwrap();

        assert!(!results.is_empty());
        assert_eq!(results[0].framework, Framework::Django);
        assert_eq!(results[0].confidence, 0.95);
    }

    #[test]
    fn test_php_detector_detects_laravel() {
        let temp_dir = TempDir::new().unwrap();
        let composer_json = r#"{"require": {"laravel/framework": "^11.0"}}"#;
        fs::write(
            temp_dir.path().join("composer.json"),
            composer_json,
        )
        .unwrap();

        let detector = PHPDetector;
        let results = detector.detect(temp_dir.path()).unwrap();

        assert!(!results.is_empty());
        assert_eq!(results[0].framework, Framework::Laravel);
        assert_eq!(results[0].confidence, 0.95);
    }

    #[test]
    fn test_detect_all_frameworks_returns_multiple() {
        let temp_dir = TempDir::new().unwrap();

        // Setup TypeScript project
        let package_json = r#"{"name": "test", "dependencies": {"@nestjs/core": "^10.0.0"}}"#;
        fs::write(temp_dir.path().join("package.json"), package_json).unwrap();

        // Setup Python project
        let requirements = "Django==4.2.0";
        fs::write(
            temp_dir.path().join("requirements.txt"),
            requirements,
        )
        .unwrap();

        let results = detect_all_frameworks(temp_dir.path()).unwrap();

        // Should have detected both NestJS and Django
        assert!(results.iter().any(|r| r.framework == Framework::NestJS));
        assert!(results.iter().any(|r| r.framework == Framework::Django));
    }
}
