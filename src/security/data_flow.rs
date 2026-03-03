use super::cfg_builder::get_builder_for_language;
use super::cfg_types::{CFG, CFGNode, NodeType};
use crate::autofix::Violation;
use crate::config::ForbiddenRule;
use std::collections::HashSet;
use std::path::PathBuf;

pub struct TaintEngine {
    sources: Vec<String>,
    sinks: Vec<String>,
    sanitizers: Vec<String>,
}

impl TaintEngine {
    /// Create a new TaintEngine for a specific language
    pub fn new(language: &str) -> Self {
        let builder = get_builder_for_language(language);
        Self {
            sources: builder.extract_sources(),
            sinks: builder.extract_sinks(),
            sanitizers: builder.extract_sanitizers(),
        }
    }

    /// Analyze CFG for unsafe data flows (taint analysis)
    pub fn analyze(&self, cfg: &CFG) -> Vec<Violation> {
        let mut violations = Vec::new();

        // Find all source nodes
        let source_nodes: Vec<&CFGNode> = cfg
            .nodes()
            .filter(|n| n.node_type == NodeType::Source || self.is_source(&n.label))
            .collect();

        // Find all sink nodes
        let sink_nodes: Vec<&CFGNode> = cfg
            .nodes()
            .filter(|n| n.node_type == NodeType::Sink || self.is_sink(&n.label))
            .collect();

        // Check for unsafe paths from sources to sinks
        for source in &source_nodes {
            for sink in &sink_nodes {
                if let Some(path) = self.find_path_with_taint(cfg, source.id, sink.id) {
                    if !self.path_is_sanitized(cfg, &path) {
                        violations.push(self.create_violation(source, sink, &path));
                    }
                }
            }
        }

        violations
    }

    /// Check if label is a source pattern (exact match or known prefix)
    fn is_source(&self, label: &str) -> bool {
        self.sources.iter().any(|s| {
            label == s
                || label.starts_with(&format!("{}.", s))
                || label.contains(&format!(".{}", s))
        })
    }

    /// Check if label is a sink pattern (exact match)
    fn is_sink(&self, label: &str) -> bool {
        self.sinks.iter().any(|s| {
            label == s
                || label.starts_with(&format!("{}(", s))
                || label.contains(&format!(".{}(", s))
                || label.contains(&format!(".{}", s))
        })
    }

    /// Check if any node in path is a sanitizer
    fn path_is_sanitized(&self, cfg: &CFG, path: &[usize]) -> bool {
        for node_id in path {
            if let Some(node) = cfg.get_node(*node_id) {
                if self.sanitizers
                    .iter()
                    .any(|s| node.label.contains(s.as_str()) || node.label == *s)
                {
                    return true;
                }
            }
        }
        false
    }

    /// Find a path from start to end node using DFS
    fn find_path_with_taint(&self, cfg: &CFG, start: usize, end: usize) -> Option<Vec<usize>> {
        let mut visited = HashSet::new();
        let mut path = Vec::new();
        self.dfs(cfg, start, end, &mut visited, &mut path).then_some(path)
    }

    /// DFS helper for pathfinding
    fn dfs(
        &self,
        cfg: &CFG,
        current: usize,
        target: usize,
        visited: &mut HashSet<usize>,
        path: &mut Vec<usize>,
    ) -> bool {
        if current == target {
            path.push(current);
            return true;
        }

        if visited.contains(&current) {
            return false;
        }

        visited.insert(current);
        path.push(current);

        for next_id in cfg.successor_ids(current) {
            if self.dfs(cfg, next_id, target, visited, path) {
                return true;
            }
        }

        path.pop();
        false
    }

    /// Create a violation for an unsafe data flow
    fn create_violation(
        &self,
        source: &CFGNode,
        sink: &CFGNode,
        _path: &[usize],
    ) -> Violation {
        Violation {
            file_path: PathBuf::from("unknown"),
            file_content: String::new(),
            offensive_import: sink.label.clone(),
            rule: ForbiddenRule {
                from: source.label.clone(),
                to: sink.label.clone(),
                severity: Some(crate::config::Severity::Error),
                reason: Some(
                    format!(
                        "Possible injection vulnerability: unsanitized data flow from '{}' (line {}) to '{}' (line {})",
                        source.label, source.line, sink.label, sink.line
                    )
                ),
            },
            line_number: sink.line,
        }
    }
}
