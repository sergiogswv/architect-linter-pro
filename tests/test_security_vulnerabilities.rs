use architect_linter_pro::security::{Severity, VulnerabilityDetector, VulnerabilityType};

#[test]
fn test_detect_hardcoded_secrets() {
    let code = r#"const API_KEY = "sk-1234567890abcdefghijklmnop";"#;
    let findings = VulnerabilityDetector::detect_hardcoded_secrets(code);
    assert!(!findings.is_empty());
    assert_eq!(findings[0].1, VulnerabilityType::HardcodedSecrets);
}

#[test]
fn test_detect_sql_injection() {
    let code = r#"const query = `SELECT * FROM users WHERE id = ${userId}`;"#;
    let findings = VulnerabilityDetector::detect_sql_injection(code);
    assert!(!findings.is_empty());
    assert_eq!(findings[0].1, VulnerabilityType::SqlInjection);
}

#[test]
fn test_detect_pickle_usage() {
    let code = "data = pickle.load(open('file.pkl'))";
    let findings = VulnerabilityDetector::detect_python_unsafe(code);
    assert!(!findings.is_empty());
    assert_eq!(findings[0].1, VulnerabilityType::PickleUsage);
}

#[test]
fn test_vulnerability_severity() {
    assert_eq!(VulnerabilityType::SqlInjection.severity(), Severity::Critical);
    assert_eq!(
        VulnerabilityType::HardcodedSecrets.severity(),
        Severity::High
    );
    assert_eq!(
        VulnerabilityType::DeprecatedDependency.severity(),
        Severity::Medium
    );
}

#[test]
fn test_detect_eval_usage() {
    let code = "result = eval(user_input)";
    let findings = VulnerabilityDetector::detect_python_unsafe(code);
    assert!(!findings.is_empty());
    assert_eq!(findings[0].1, VulnerabilityType::EvalUsage);
}

#[test]
fn test_detect_insecure_yaml() {
    let code = "data = yaml.load(open('config.yml'))";
    let findings = VulnerabilityDetector::detect_python_unsafe(code);
    assert!(!findings.is_empty());
    assert_eq!(findings[0].1, VulnerabilityType::InsecureYaml);
}

#[test]
fn test_safe_yaml_not_detected() {
    let code = "data = yaml.safe_load(open('config.yml'))";
    let findings = VulnerabilityDetector::detect_python_unsafe(code);
    assert!(findings.is_empty());
}

#[test]
fn test_vulnerability_descriptions() {
    assert!(!VulnerabilityType::SqlInjection.description().is_empty());
    assert!(!VulnerabilityType::CrossSiteScripting.description().is_empty());
    assert!(!VulnerabilityType::HardcodedSecrets.description().is_empty());
    assert!(!VulnerabilityType::DeprecatedDependency.description().is_empty());
}
