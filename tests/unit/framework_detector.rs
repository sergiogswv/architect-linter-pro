use architect_linter_pro::config::Framework;
use architect_linter_pro::detection::{
    detect_all_frameworks, DetectionResult, FrameworkDetector, PHPDetector, PythonDetector,
    TypeScriptDetector,
};
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
fn test_typescript_detector_detects_react() {
    let temp_dir = TempDir::new().unwrap();
    let package_json = r#"{"name": "test", "dependencies": {"react": "^18.0.0", "react-dom": "^18.0.0"}}"#;
    fs::write(temp_dir.path().join("package.json"), package_json).unwrap();

    let detector = TypeScriptDetector;
    let results = detector.detect(temp_dir.path()).unwrap();

    assert!(!results.is_empty());
    assert!(results.iter().any(|r| r.framework == Framework::React));
}

#[test]
fn test_typescript_detector_detects_multiple_frameworks() {
    let temp_dir = TempDir::new().unwrap();
    let package_json = r#"{"name": "test", "dependencies": {"react": "^18.0.0", "express": "^4.18.0"}}"#;
    fs::write(temp_dir.path().join("package.json"), package_json).unwrap();

    let detector = TypeScriptDetector;
    let results = detector.detect(temp_dir.path()).unwrap();

    assert_eq!(results.len(), 2);
    assert!(results.iter().any(|r| r.framework == Framework::React));
    assert!(results.iter().any(|r| r.framework == Framework::Express));
}

#[test]
fn test_typescript_detector_handles_missing_package_json() {
    let temp_dir = TempDir::new().unwrap();

    let detector = TypeScriptDetector;
    let result = detector.detect(temp_dir.path());

    assert!(result.is_err());
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
fn test_python_detector_detects_fastapi_from_pyproject() {
    let temp_dir = TempDir::new().unwrap();
    let pyproject = r#"[tool.poetry.dependencies]
python = "^3.9"
fastapi = "^0.104.0"
"#;
    fs::write(
        temp_dir.path().join("pyproject.toml"),
        pyproject,
    )
    .unwrap();

    let detector = PythonDetector;
    let results = detector.detect(temp_dir.path()).unwrap();

    assert!(!results.is_empty());
    assert_eq!(results[0].framework, Framework::FastAPI);
}

#[test]
fn test_python_detector_detects_flask_from_pipfile() {
    let temp_dir = TempDir::new().unwrap();
    let pipfile = r#"[packages]
flask = "*"
"#;
    fs::write(
        temp_dir.path().join("Pipfile"),
        pipfile,
    )
    .unwrap();

    let detector = PythonDetector;
    let results = detector.detect(temp_dir.path()).unwrap();

    assert!(!results.is_empty());
    assert_eq!(results[0].framework, Framework::Flask);
}

#[test]
fn test_python_detector_returns_empty_on_no_frameworks() {
    let temp_dir = TempDir::new().unwrap();
    let requirements = "numpy==1.24.0\npandas==2.0.0";
    fs::write(
        temp_dir.path().join("requirements.txt"),
        requirements,
    )
    .unwrap();

    let detector = PythonDetector;
    let results = detector.detect(temp_dir.path()).unwrap();

    assert!(results.is_empty());
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
fn test_php_detector_detects_symfony() {
    let temp_dir = TempDir::new().unwrap();
    let composer_json = r#"{"require": {"symfony/framework-bundle": "^7.0"}}"#;
    fs::write(
        temp_dir.path().join("composer.json"),
        composer_json,
    )
    .unwrap();

    let detector = PHPDetector;
    let results = detector.detect(temp_dir.path()).unwrap();

    assert!(!results.is_empty());
    assert_eq!(results[0].framework, Framework::Symfony);
}

#[test]
fn test_php_detector_handles_missing_composer_json() {
    let temp_dir = TempDir::new().unwrap();

    let detector = PHPDetector;
    let result = detector.detect(temp_dir.path());

    assert!(result.is_err());
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

#[test]
fn test_detect_all_frameworks_sorts_by_confidence() {
    let temp_dir = TempDir::new().unwrap();

    // Setup TypeScript project with multiple frameworks
    let package_json = r#"{"name": "test", "dependencies": {"react": "^18.0.0", "express": "^4.18.0", "vue": "^3.0.0"}}"#;
    fs::write(temp_dir.path().join("package.json"), package_json).unwrap();

    let results = detect_all_frameworks(temp_dir.path()).unwrap();

    // Results should be sorted by confidence (all 0.90 in this case)
    for i in 0..results.len() - 1 {
        assert!(results[i].confidence >= results[i + 1].confidence);
    }
}

#[test]
fn test_typescript_detector_name() {
    let detector = TypeScriptDetector;
    assert_eq!(detector.name(), "TypeScript/JavaScript");
}

#[test]
fn test_python_detector_name() {
    let detector = PythonDetector;
    assert_eq!(detector.name(), "Python");
}

#[test]
fn test_php_detector_name() {
    let detector = PHPDetector;
    assert_eq!(detector.name(), "PHP");
}

#[test]
fn test_detection_result_equality() {
    let result1 = DetectionResult::new(Framework::NestJS, 0.95);
    let result2 = DetectionResult::new(Framework::NestJS, 0.95);
    let result3 = DetectionResult::new(Framework::React, 0.95);

    assert_eq!(result1, result2);
    assert_ne!(result1, result3);
}
