/// Control Flow Graph (CFG) builder abstraction
///
/// This module provides a language-agnostic trait for extracting security-relevant patterns
/// (sources, sinks, sanitizers) from source code, along with language-specific implementations.

use std::path::Path;

/// Language-agnostic CFG builder abstraction
///
/// Implementations extract security-critical patterns from source code:
/// - Sources: entry points where untrusted data enters the system
/// - Sinks: dangerous operations that could lead to vulnerabilities
/// - Sanitizers: functions that process/validate data safely
pub trait CFGBuilder {
    /// Extract source patterns (user input entry points)
    ///
    /// Sources are entry points where untrusted or external data enters the system.
    /// Examples: HTTP request parameters, environment variables, file inputs
    fn extract_sources(&self) -> Vec<String>;

    /// Extract sink patterns (dangerous operations)
    ///
    /// Sinks are operations that could lead to vulnerabilities if used with untrusted data.
    /// Examples: SQL queries, command execution, code evaluation
    fn extract_sinks(&self) -> Vec<String>;

    /// Extract sanitizer patterns (trusted processing)
    ///
    /// Sanitizers are functions that safely process or validate data,
    /// reducing the risk of vulnerabilities.
    /// Examples: HTML escaping, input validation, parameterized queries
    fn extract_sanitizers(&self) -> Vec<String>;
}

/// TypeScript/JavaScript CFG Builder
///
/// Handles source, sink, and sanitizer extraction for TypeScript and JavaScript code.
pub struct TypeScriptCFGBuilder;

impl CFGBuilder for TypeScriptCFGBuilder {
    fn extract_sources(&self) -> Vec<String> {
        vec![
            "req.body".to_string(),
            "req.query".to_string(),
            "req.params".to_string(),
            "request.form".to_string(),
            "process.env".to_string(),
        ]
    }

    fn extract_sinks(&self) -> Vec<String> {
        vec![
            "db.query".to_string(),
            "db.execute".to_string(),
            "eval".to_string(),
            "dangerouslySetInnerHTML".to_string(),
            "exec".to_string(),
            "spawn".to_string(),
        ]
    }

    fn extract_sanitizers(&self) -> Vec<String> {
        vec![
            "escape".to_string(),
            "parseInt".to_string(),
            "sanitize".to_string(),
            "htmlspecialchars".to_string(),
        ]
    }
}

/// Python CFG Builder
///
/// Handles source, sink, and sanitizer extraction for Python code.
pub struct PythonCFGBuilder;

impl CFGBuilder for PythonCFGBuilder {
    fn extract_sources(&self) -> Vec<String> {
        vec![
            "request.form".to_string(),
            "request.args".to_string(),
            "request.json".to_string(),
            "input".to_string(),
            "sys.argv".to_string(),
        ]
    }

    fn extract_sinks(&self) -> Vec<String> {
        vec![
            "subprocess.run".to_string(),
            "subprocess.call".to_string(),
            "subprocess.Popen".to_string(),
            "os.system".to_string(),
            "eval".to_string(),
            "exec".to_string(),
            "conn.execute".to_string(),
        ]
    }

    fn extract_sanitizers(&self) -> Vec<String> {
        vec![
            "escape".to_string(),
            "int".to_string(),
            "sanitize".to_string(),
            "htmlspecialchars".to_string(),
        ]
    }
}

/// PHP CFG Builder
///
/// Handles source, sink, and sanitizer extraction for PHP code.
pub struct PHPCFGBuilder;

impl CFGBuilder for PHPCFGBuilder {
    fn extract_sources(&self) -> Vec<String> {
        vec![
            "$_GET".to_string(),
            "$_POST".to_string(),
            "$_REQUEST".to_string(),
            "$_SERVER".to_string(),
        ]
    }

    fn extract_sinks(&self) -> Vec<String> {
        vec![
            "query".to_string(),
            "execute".to_string(),
            "eval".to_string(),
            "shell_exec".to_string(),
            "exec".to_string(),
            "system".to_string(),
        ]
    }

    fn extract_sanitizers(&self) -> Vec<String> {
        vec![
            "htmlspecialchars".to_string(),
            "intval".to_string(),
            "sanitize".to_string(),
            "escape".to_string(),
        ]
    }
}

/// Factory function to get the appropriate CFGBuilder for a language
///
/// # Arguments
/// * `language` - Language identifier (e.g., "typescript", "python", "php")
///
/// # Returns
/// A boxed trait object implementing CFGBuilder for the specified language.
/// Defaults to TypeScriptCFGBuilder for unknown languages.
pub fn get_builder_for_language(language: &str) -> Box<dyn CFGBuilder> {
    match language {
        "typescript" | "javascript" => Box::new(TypeScriptCFGBuilder),
        "python" => Box::new(PythonCFGBuilder),
        "php" => Box::new(PHPCFGBuilder),
        _ => Box::new(TypeScriptCFGBuilder), // default
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_typescript_builder_sources() {
        let builder = TypeScriptCFGBuilder;
        let sources = builder.extract_sources();
        assert!(!sources.is_empty());
        assert!(sources.contains(&"req.body".to_string()));
        assert!(sources.contains(&"req.query".to_string()));
    }

    #[test]
    fn test_typescript_builder_sinks() {
        let builder = TypeScriptCFGBuilder;
        let sinks = builder.extract_sinks();
        assert!(!sinks.is_empty());
        assert!(sinks.contains(&"db.query".to_string()));
        assert!(sinks.contains(&"eval".to_string()));
    }

    #[test]
    fn test_typescript_builder_sanitizers() {
        let builder = TypeScriptCFGBuilder;
        let sanitizers = builder.extract_sanitizers();
        assert!(!sanitizers.is_empty());
        assert!(sanitizers.contains(&"escape".to_string()));
    }

    #[test]
    fn test_python_builder_sources() {
        let builder = PythonCFGBuilder;
        let sources = builder.extract_sources();
        assert!(!sources.is_empty());
        assert!(sources.contains(&"request.form".to_string()));
        assert!(sources.contains(&"input".to_string()));
    }

    #[test]
    fn test_python_builder_sinks() {
        let builder = PythonCFGBuilder;
        let sinks = builder.extract_sinks();
        assert!(!sinks.is_empty());
        assert!(sinks.contains(&"os.system".to_string()));
        assert!(sinks.contains(&"eval".to_string()));
    }

    #[test]
    fn test_python_builder_sanitizers() {
        let builder = PythonCFGBuilder;
        let sanitizers = builder.extract_sanitizers();
        assert!(!sanitizers.is_empty());
        assert!(sanitizers.contains(&"escape".to_string()));
    }

    #[test]
    fn test_php_builder_sources() {
        let builder = PHPCFGBuilder;
        let sources = builder.extract_sources();
        assert!(!sources.is_empty());
        assert!(sources.contains(&"$_GET".to_string()));
        assert!(sources.contains(&"$_POST".to_string()));
    }

    #[test]
    fn test_php_builder_sinks() {
        let builder = PHPCFGBuilder;
        let sinks = builder.extract_sinks();
        assert!(!sinks.is_empty());
        assert!(sinks.contains(&"eval".to_string()));
        assert!(sinks.contains(&"system".to_string()));
    }

    #[test]
    fn test_php_builder_sanitizers() {
        let builder = PHPCFGBuilder;
        let sanitizers = builder.extract_sanitizers();
        assert!(!sanitizers.is_empty());
        assert!(sanitizers.contains(&"htmlspecialchars".to_string()));
    }

    #[test]
    fn test_factory_function_typescript() {
        let builder = get_builder_for_language("typescript");
        assert!(!builder.extract_sources().is_empty());
    }

    #[test]
    fn test_factory_function_python() {
        let builder = get_builder_for_language("python");
        assert!(!builder.extract_sources().is_empty());
    }

    #[test]
    fn test_factory_function_php() {
        let builder = get_builder_for_language("php");
        assert!(!builder.extract_sources().is_empty());
    }

    #[test]
    fn test_factory_function_default() {
        let builder = get_builder_for_language("unknown_language");
        // Should default to TypeScript builder
        assert!(!builder.extract_sources().is_empty());
    }
}
