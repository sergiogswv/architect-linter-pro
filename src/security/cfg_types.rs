/// Control Flow Graph (CFG) type system
///
/// This module defines the core data structures for building and analyzing
/// control flow graphs, which are used for security analysis and data flow tracking.

use std::collections::HashMap;

/// Types of nodes in a control flow graph
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeType {
    /// Data source (user input, external data)
    Source,
    /// Data sink (output, execute, query)
    Sink,
    /// Variable assignment
    Assignment,
    /// Conditional branch (if, switch)
    Condition,
    /// Function call
    FunctionCall,
    /// Generic code block
    Block,
    /// Return statement
    Return,
}

/// Types of edges in a control flow graph
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeType {
    /// Sequential execution
    Sequential,
    /// Conditional branch taken
    Conditional,
    /// Loop back edge
    Loop,
    /// Function call edge
    FunctionCall,
    /// Return from function
    Return,
}

/// Context information for a CFG node
#[derive(Debug, Clone, Default)]
pub struct NodeContext {
    /// Name of the variable being assigned/used
    pub variable_name: Option<String>,
    /// Name of the function containing this node
    pub function_name: Option<String>,
    /// ID of the parent node (for nested structures)
    pub parent_id: Option<usize>,
}

/// A node in the control flow graph
#[derive(Debug, Clone)]
pub struct CFGNode {
    /// Unique identifier for this node
    pub id: usize,
    /// Type of this node
    pub node_type: NodeType,
    /// Human-readable label (e.g., source code snippet)
    pub label: String,
    /// Line number in source code
    pub line: usize,
    /// Additional context information
    pub context: NodeContext,
}

/// An edge connecting two nodes in the control flow graph
#[derive(Debug, Clone)]
pub struct CFGEdge {
    /// ID of the source node
    pub from: usize,
    /// ID of the destination node
    pub to: usize,
    /// Type of this edge
    pub edge_type: EdgeType,
}

/// A complete control flow graph
#[derive(Debug, Clone)]
pub struct CFG {
    /// All nodes in the graph, indexed by ID
    nodes: HashMap<usize, CFGNode>,
    /// All edges in the graph
    edges: Vec<CFGEdge>,
    /// Counter for generating unique node IDs
    next_id: usize,
}

impl Default for CFG {
    fn default() -> Self {
        Self::new()
    }
}

impl CFG {
    /// Creates a new empty control flow graph
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: Vec::new(),
            next_id: 1,
        }
    }

    /// Adds a node to the graph
    pub fn add_node(&mut self, node: CFGNode) {
        self.nodes.insert(node.id, node);
    }

    /// Adds an edge to the graph
    pub fn add_edge(&mut self, edge: CFGEdge) {
        self.edges.push(edge);
    }

    /// Gets a node by its ID
    pub fn get_node(&self, id: usize) -> Option<&CFGNode> {
        self.nodes.get(&id)
    }

    /// Returns the IDs of all successor nodes for the given node
    pub fn successor_ids(&self, node_id: usize) -> Vec<usize> {
        self.edges
            .iter()
            .filter(|e| e.from == node_id)
            .map(|e| e.to)
            .collect()
    }

    /// Returns the IDs of all predecessor nodes for the given node
    pub fn predecessor_ids(&self, node_id: usize) -> Vec<usize> {
        self.edges
            .iter()
            .filter(|e| e.to == node_id)
            .map(|e| e.from)
            .collect()
    }

    /// Returns the number of nodes in the graph
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Returns true if the graph contains no nodes
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Returns all nodes in the graph
    pub fn nodes(&self) -> impl Iterator<Item = &CFGNode> {
        self.nodes.values()
    }

    /// Returns all edges in the graph
    pub fn edges(&self) -> impl Iterator<Item = &CFGEdge> {
        self.edges.iter()
    }

    /// Generates a new unique node ID
    pub fn next_node_id(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cfg_node_creation() {
        let context = NodeContext {
            variable_name: Some("x".to_string()),
            function_name: Some("main".to_string()),
            parent_id: None,
        };

        let node = CFGNode {
            id: 1,
            node_type: NodeType::Assignment,
            label: "x = 5".to_string(),
            line: 10,
            context,
        };

        assert_eq!(node.id, 1);
        assert!(matches!(node.node_type, NodeType::Assignment));
        assert_eq!(node.label, "x = 5");
        assert_eq!(node.line, 10);
        assert_eq!(node.context.variable_name, Some("x".to_string()));
        assert_eq!(node.context.function_name, Some("main".to_string()));
        assert_eq!(node.context.parent_id, None);
    }

    #[test]
    fn test_cfg_edge_creation() {
        let edge = CFGEdge {
            from: 1,
            to: 2,
            edge_type: EdgeType::Sequential,
        };

        assert_eq!(edge.from, 1);
        assert_eq!(edge.to, 2);
        assert!(matches!(edge.edge_type, EdgeType::Sequential));
    }

    #[test]
    fn test_cfg_graph_creation() {
        let mut cfg = CFG::new();

        let node = CFGNode {
            id: 1,
            node_type: NodeType::Block,
            label: "entry".to_string(),
            line: 1,
            context: Default::default(),
        };

        cfg.add_node(node);

        assert_eq!(cfg.len(), 1);
        assert!(cfg.get_node(1).is_some());
        assert_eq!(cfg.get_node(1).unwrap().label, "entry");
    }
}
