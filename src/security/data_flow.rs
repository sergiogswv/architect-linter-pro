//! Taint Analysis (Análisis de Manchas)
//!
//! Este módulo rastrea el flujo de datos desde fuentes no confiables (Sources)
//! hasta funciones u operaciones peligrosas (Sinks).

use super::cfg::CFG;
use crate::autofix::Violation;
use crate::config::ForbiddenRule;
use std::path::Path;

pub struct TaintEngine {
    pub sources: Vec<String>,
    pub sinks: Vec<String>,
    pub sanitizers: Vec<String>,
}

impl TaintEngine {
    pub fn new() -> Self {
        Self {
            sources: vec![
                "req.body".to_string(),
                "req.query".to_string(),
                "req.params".to_string(),
                "process.env".to_string(),
                "request.form".to_string(), // Python Flask/Django
                "request.args".to_string(),
                "request.json".to_string(),
                "input(".to_string(),      // Python built-in input
            ],
            sinks: vec![
                "query".to_string(),
                "execute".to_string(),
                "eval".to_string(),
                "dangerouslySetInnerHTML".to_string(),
                "exec".to_string(),
                "spawn".to_string(),
                "subprocess".to_string(),  // Python subprocess
                "system".to_string(),      // os.system
            ],
            sanitizers: vec![
                "escape".to_string(),
                "parseInt".to_string(),
                "sanitize".to_string(),
            ],
        }
    }

    /// Analiza el CFG en busca de flujos de datos no seguros
    pub fn analyze(&self, cfg: &CFG, file_path: &Path, source_code: &str) -> Vec<Violation> {
        let mut violations = Vec::new();

        // Identificar nodos que son Sources y Sinks
        let source_nodes: Vec<_> = cfg.nodes.iter()
            .filter(|n| n.node_type == super::cfg::NodeType::Source || self.is_source(&n.label))
            .collect();

        let sink_nodes: Vec<_> = cfg.nodes.iter()
            .filter(|n| n.node_type == super::cfg::NodeType::Sink || self.is_sink(&n.label))
            .collect();

        for source in source_nodes {
            for sink in &sink_nodes {
                if self.has_unsafe_path(cfg, source.id, sink.id) {
                    violations.push(create_security_violation(
                        file_path,
                        source_code,
                        &sink.label,
                        &format!("Posible vulnerabilidad de inyección: flujo detectado desde '{}' hasta '{}'.", source.label, sink.label),
                        sink.line,
                    ));
                }
            }
        }

        violations
    }

    fn is_source(&self, label: &str) -> bool {
        self.sources.iter().any(|s| label.contains(s))
    }

    fn is_sink(&self, label: &str) -> bool {
        self.sinks.iter().any(|s| label.contains(s))
    }

    fn is_sanitizer(&self, label: &str) -> bool {
        self.sanitizers.iter().any(|s| label.contains(s))
    }

    /// Verifica si hay un camino desde source a sink que no pase por un sanitizer
    fn has_unsafe_path(&self, cfg: &CFG, start_id: usize, end_id: usize) -> bool {
        let mut visited = std::collections::HashSet::new();
        let mut stack = vec![start_id];

        while let Some(current) = stack.pop() {
            if current == end_id {
                return true;
            }

            if visited.contains(&current) {
                continue;
            }
            visited.insert(current);

            // Si el nodo actual es un sanitizer, detenemos la exploración por este camino
            if let Some(node) = cfg.nodes.get(current) {
                if self.is_sanitizer(&node.label) {
                    continue;
                }
            }

            // Añadir vecinos al stack
            for (from, to) in &cfg.edges {
                if *from == current {
                    stack.push(*to);
                }
            }
        }

        false
    }
}

/// Helper para crear violaciones de seguridad con severidad crítica
pub fn create_security_violation(
    file_path: &Path,
    source_code: &str,
    offensive_code: &str,
    message: &str,
    line: usize,
) -> Violation {
    Violation {
        file_path: file_path.to_path_buf(),
        file_content: source_code.to_string(),
        offensive_import: offensive_code.to_string(),
        rule: ForbiddenRule {
            from: "SecurityModule".to_string(),
            to: "InsecureSink".to_string(),
            severity: Some(crate::config::Severity::Error),
        },
        line_number: line,
    }
}
