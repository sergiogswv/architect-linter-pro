#[test]
fn test_method_length_short_method_passes() {
    let content = "class Foo {\n  method() {\n    return 1;\n  }\n}\n";
    // 3 lines of body — should pass with threshold 40
    let result = architect_linter_pro::analyzer::swc_parser::validate_method_length_ts(content, 40);
    assert!(result.is_ok(), "Short method should pass: {:?}", result);
}

#[test]
fn test_method_length_long_method_fails() {
    let mut content = "class Foo {\n  method() {\n".to_string();
    for i in 0..50 {
        content.push_str(&format!("    const x{} = {};\n", i, i));
    }
    content.push_str("  }\n}\n");
    let result = architect_linter_pro::analyzer::swc_parser::validate_method_length_ts(&content, 40);
    assert!(result.is_err(), "Long method should fail");
}

#[test]
fn test_method_length_multiple_methods_one_long() {
    // Short method followed by long method
    let mut content = "class Foo {\n  short() {\n    return 1;\n  }\n  long() {\n".to_string();
    for i in 0..50 {
        content.push_str(&format!("    const x{} = {};\n", i, i));
    }
    content.push_str("  }\n}\n");
    let result = architect_linter_pro::analyzer::swc_parser::validate_method_length_ts(&content, 40);
    assert!(result.is_err(), "Should fail if any method is long");
}
