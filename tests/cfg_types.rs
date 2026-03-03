use architect_linter_pro::security::cfg_types::{CFG, CFGNode, CFGEdge, NodeType, EdgeType, NodeContext};

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
