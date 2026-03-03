use architect_linter_pro::security::{CFG, CFGNode, CFGEdge, NodeType, EdgeType, TaintEngine};

#[test]
fn test_taint_engine_no_false_positives() {
    // Test: sanitized input should NOT trigger violation
    let mut cfg = CFG::new();

    // req.body → escape() → db.query()
    cfg.add_node(CFGNode {
        id: 1,
        node_type: NodeType::Source,
        label: "req.body".to_string(),
        line: 10,
        context: Default::default(),
    });

    cfg.add_node(CFGNode {
        id: 2,
        node_type: NodeType::FunctionCall,
        label: "escape".to_string(),
        line: 11,
        context: Default::default(),
    });

    cfg.add_node(CFGNode {
        id: 3,
        node_type: NodeType::Sink,
        label: "db.query".to_string(),
        line: 12,
        context: Default::default(),
    });

    cfg.add_edge(CFGEdge { from: 1, to: 2, edge_type: EdgeType::Sequential });
    cfg.add_edge(CFGEdge { from: 2, to: 3, edge_type: EdgeType::Sequential });

    let engine = TaintEngine::new("typescript");
    let violations = engine.analyze(&cfg);

    // Should be safe because escape() sanitizes
    assert!(violations.is_empty());
}

#[test]
fn test_taint_engine_detects_unsafe_flow() {
    // Test: unsanitized input SHOULD trigger violation
    let mut cfg = CFG::new();

    // req.body → db.query() (NO sanitizer)
    cfg.add_node(CFGNode {
        id: 1,
        node_type: NodeType::Source,
        label: "req.body".to_string(),
        line: 10,
        context: Default::default(),
    });

    cfg.add_node(CFGNode {
        id: 2,
        node_type: NodeType::Sink,
        label: "db.query".to_string(),
        line: 12,
        context: Default::default(),
    });

    cfg.add_edge(CFGEdge { from: 1, to: 2, edge_type: EdgeType::Sequential });

    let engine = TaintEngine::new("typescript");
    let violations = engine.analyze(&cfg);

    // Should be unsafe
    assert!(!violations.is_empty());
}
