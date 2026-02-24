use crate::config::{ArchPattern, ConfigFile, ForbiddenRule, Severity};

/// Build a ForbiddenRule with a reason string.
pub fn rule(from: &str, to: &str, severity: &str, reason: &str) -> ForbiddenRule {
    let sev = match severity {
        "warning" => Some(Severity::Warning),
        "info" => Some(Severity::Info),
        _ => Some(Severity::Error),
    };
    ForbiddenRule {
        from: from.to_string(),
        to: to.to_string(),
        severity: sev,
        reason: Some(reason.to_string()),
    }
}

/// Build a ConfigFile with `pattern: Custom(pattern)` and the given rules.
pub fn base_config(pattern: &str, rules: Vec<ForbiddenRule>) -> ConfigFile {
    use crate::config::default_ignored_paths;
    ConfigFile {
        architecture_pattern: ArchPattern::Custom(pattern.to_string()),
        forbidden_imports: rules,
        max_lines_per_function: 40,
        ignored_paths: default_ignored_paths(),
        build_command: None,
        ai_fix_retries: 3,
    }
}
