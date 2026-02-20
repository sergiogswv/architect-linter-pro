//! Control Flow Graph (CFG) para análisis estático multi-lenguaje
//!
//! Este módulo construye un grafo de flujo de control genérico que puede ser
//! alimentado por diferentes parsers (Tree-sitter, SWC, etc.)

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeType {
    Entry,
    Statement,
    Branch,
    Sink,    // Punto de salida peligroso (SQLQuery, eval, etc.)
    Source,  // Punto de entrada de usuario (req.body, params)
    Call,    // Llamada a función
    Exit,
}

#[derive(Debug, Clone)]
pub struct CFGNode {
    pub id: usize,
    pub node_type: NodeType,
    pub label: String,
    pub line: usize,
}

pub struct CFG {
    pub nodes: Vec<CFGNode>,
    pub edges: Vec<(usize, usize)>, // (de_id, a_id)
    pub var_map: HashMap<String, usize>, // Rastreo de definiciones de variables
}

impl CFG {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            var_map: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node_type: NodeType, label: String, line: usize) -> usize {
        let id = self.nodes.len();
        self.nodes.push(CFGNode {
            id,
            node_type,
            label,
            line,
        });
        id
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        self.edges.push((from, to));
    }

    /// Construye un CFG básico usando un árbol de Tree-sitter de forma genérica
    pub fn from_tree(tree: &tree_sitter::Tree, source_code: &str) -> Self {
        let mut cfg = Self::new();
        let root = tree.root_node();
        let start_node = cfg.add_node(NodeType::Entry, "START".to_string(), 1);

        // Usamos un cursor para recorrer el árbol
        let mut cursor = tree.walk();
        let mut last_id = start_node;

        Self::visit_nodes(&mut cursor, source_code, &mut cfg, &mut last_id);

        cfg.add_node(NodeType::Exit, "END".to_string(), root.end_position().row + 1);
        cfg
    }

    fn visit_nodes(
        cursor: &mut tree_sitter::TreeCursor,
        source_code: &str,
        cfg: &mut CFG,
        last_id: &mut usize,
    ) {
        let node = cursor.node();
        let name = node.utf8_text(source_code.as_bytes()).unwrap_or("");
        let line = node.start_position().row + 1;

        // Detección genérica basada en palabras clave en los nombres de los nodos
        // Esto es una simplificación; cada lenguaje podría tener sus propias palabras clave
        let node_type = if name.contains("query") || name.contains("execute") || name.contains("eval") {
            Some(NodeType::Sink)
        } else if name.contains("body") || name.contains("params") || name.contains("input") {
            Some(NodeType::Source)
        } else if node.kind().contains("call") {
            Some(NodeType::Call)
        } else {
            None
        };

        if let Some(nt) = node_type {
            let current_id = cfg.add_node(nt, name.to_string(), line);
            cfg.add_edge(*last_id, current_id);
            *last_id = current_id;
        }

        if cursor.goto_first_child() {
            Self::visit_nodes(cursor, source_code, cfg, last_id);
            cursor.goto_parent();
        }

        if cursor.goto_next_sibling() {
            Self::visit_nodes(cursor, source_code, cfg, last_id);
        }
    }
}
