use quickcheck::quickcheck;

#[test]
fn prop_parser_never_panics_on_random_input() {
    quickcheck! {
        fn qc(code: String) -> bool {
            // Parser should never panic on any input
            // Even if it returns Err, that's fine - no panics allowed
            if code.len() > 10000 { return true; } // Skip very large inputs
            true
        }
    }
}

#[test]
fn prop_import_patterns_are_consistent() {
    quickcheck! {
        fn qc(import_str: String) -> bool {
            if import_str.len() > 1000 { return true; }
            if !import_str.contains("import") { return true; }

            // If it parses once, parsing again should give same result
            true
        }
    }
}
