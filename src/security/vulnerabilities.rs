//! Vulnerability detection for pro features

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VulnerabilityType {
    // TypeScript/JavaScript
    SqlInjection,
    CrossSiteScripting,
    InsecureCrypto,
    HardcodedSecrets,

    // Python
    PickleUsage,
    EvalUsage,
    InsecureYaml,

    // General
    DeprecatedDependency,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
}

impl VulnerabilityType {
    pub fn severity(&self) -> Severity {
        match self {
            Self::SqlInjection => Severity::Critical,
            Self::EvalUsage => Severity::Critical,
            Self::CrossSiteScripting => Severity::Critical,
            Self::InsecureCrypto => Severity::High,
            Self::HardcodedSecrets => Severity::High,
            Self::PickleUsage => Severity::High,
            Self::InsecureYaml => Severity::High,
            Self::DeprecatedDependency => Severity::Medium,
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::SqlInjection => "Potential SQL injection vulnerability",
            Self::CrossSiteScripting => "Potential XSS vulnerability",
            Self::InsecureCrypto => "Use of insecure cryptography",
            Self::HardcodedSecrets => "Hardcoded secrets detected",
            Self::PickleUsage => "Unsafe pickle usage in Python",
            Self::EvalUsage => "Use of eval() is dangerous",
            Self::InsecureYaml => "Use of unsafe YAML loading",
            Self::DeprecatedDependency => "Dependency has known vulnerabilities",
        }
    }
}

pub struct VulnerabilityDetector;

impl VulnerabilityDetector {
    /// Detect hardcoded secrets in code
    pub fn detect_hardcoded_secrets(code: &str) -> Vec<(usize, VulnerabilityType)> {
        let mut findings = Vec::new();

        // Pattern: API keys, tokens, passwords
        let patterns = [
            "api_key",
            "apikey",
            "api-key",
            "secret",
            "password",
            "token",
            "aws_",
            "github_token",
        ];

        for (line_num, line) in code.lines().enumerate() {
            let line_lower = line.to_lowercase();

            // Skip test and example files
            if line_lower.contains("example") || line_lower.contains("test") {
                continue;
            }

            for pattern in &patterns {
                if line_lower.contains(pattern)
                    && line_lower.contains("=")
                    && line.len() > 20
                {
                    findings.push((line_num + 1, VulnerabilityType::HardcodedSecrets));
                    break;
                }
            }
        }

        findings
    }

    /// Detect SQL injection patterns
    pub fn detect_sql_injection(code: &str) -> Vec<(usize, VulnerabilityType)> {
        let mut findings = Vec::new();

        for (line_num, line) in code.lines().enumerate() {
            // Pattern: query with template literals or string concat
            if (line.contains("SELECT") || line.contains("query"))
                && (line.contains("${") || line.contains("+ "))
            {
                findings.push((line_num + 1, VulnerabilityType::SqlInjection));
            }
        }

        findings
    }

    /// Detect Python unsafe operations
    pub fn detect_python_unsafe(code: &str) -> Vec<(usize, VulnerabilityType)> {
        let mut findings = Vec::new();

        for (line_num, line) in code.lines().enumerate() {
            if line.contains("pickle.load") {
                findings.push((line_num + 1, VulnerabilityType::PickleUsage));
            }
            if line.contains("eval(") && !line.trim().starts_with("#") {
                findings.push((line_num + 1, VulnerabilityType::EvalUsage));
            }
            if line.contains("yaml.load") && !line.contains("safe_load") {
                findings.push((line_num + 1, VulnerabilityType::InsecureYaml));
            }
        }

        findings
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(VulnerabilityType::HardcodedSecrets.severity(), Severity::High);
        assert_eq!(VulnerabilityType::DeprecatedDependency.severity(), Severity::Medium);
    }
}
