use architect_linter_pro::config::Framework;

#[test]
fn test_nestjs_hexagonal_template_has_rules() {
    let tmpl = architect_linter_pro::init::templates::get_template(&Framework::NestJS, "hexagonal");
    assert!(tmpl.is_some(), "NestJS hexagonal template must exist");
    let config = tmpl.unwrap();
    assert!(!config.forbidden_imports.is_empty(), "Must have at least one rule");
    assert!(config.forbidden_imports.iter().any(|r| r.from.contains("/domain/")));
}

#[test]
fn test_unknown_pattern_returns_none() {
    let tmpl = architect_linter_pro::init::templates::get_template(&Framework::NestJS, "nonexistent");
    assert!(tmpl.is_none());
}

#[test]
fn test_all_frameworks_have_templates() {
    use architect_linter_pro::config::Framework;
    let cases = vec![
        (Framework::NestJS, "hexagonal"),
        (Framework::NestJS, "clean"),
        (Framework::NestJS, "layered"),
        (Framework::React, "feature-based"),
        (Framework::React, "layered"),
        (Framework::Express, "mvc"),
        (Framework::Express, "hexagonal"),
        (Framework::Express, "feature-based"),
        (Framework::Django, "mvt"),
        (Framework::Django, "service-layer"),
    ];
    for (fw, pattern) in cases {
        let result = architect_linter_pro::init::templates::get_template(&fw, pattern);
        assert!(result.is_some(), "Missing template: {:?} / {}", fw, pattern);
    }
}
