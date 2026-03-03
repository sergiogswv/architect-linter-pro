use architect_linter_pro::security::CFGBuilder;

#[test]
fn test_cfg_builder_trait_exists() {
    // This is a compile-time test - trait must be defined
    fn accepts_cfg_builder<T: CFGBuilder>(_: &T) {}
    // If trait doesn't exist, this won't compile
}

#[test]
fn test_cfg_builder_returns_cfg() {
    // Mock implementation to verify trait structure
    struct MockBuilder;
    impl CFGBuilder for MockBuilder {
        fn extract_sources(&self) -> Vec<String> {
            vec!["req.body".to_string()]
        }
        fn extract_sinks(&self) -> Vec<String> {
            vec!["db.query".to_string()]
        }
        fn extract_sanitizers(&self) -> Vec<String> {
            vec!["escape".to_string()]
        }
    }
    let builder = MockBuilder;
    assert!(!builder.extract_sources().is_empty());
    assert!(!builder.extract_sinks().is_empty());
}
