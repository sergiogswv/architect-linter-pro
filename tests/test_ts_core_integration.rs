mod common;
use architect_linter_pro::analyzer;
use architect_linter_pro::config;
use architect_linter_pro::scoring;
use common::{forbidden_rule, join_rules, TestProject};

#[test]
fn test_ts_core_full_pipeline() {
    let project = TestProject::new();

    // 1. Create a forbidden rule: API cannot import INFRA directly
    let rules = vec![forbidden_rule("/api/", "/infra/")];
    project.create_config("MVC", 100, &join_rules(&rules));

    // 2. Create violation: An api component importing an infra component
    project.create_file("src/infra/db.ts", "export class DB { save() {} }");
    project.create_file(
        "src/api/user.ts",
        "import { DB } from '../infra/db';\nexport class UserController {}",
    );

    // 3. Load config and analyze
    let ctx = config::load_config(project.path()).expect("Failed to load config");

    let files = project.collect_ts_files();
    assert_eq!(files.len(), 2);

    let result =
        analyzer::analyze_all_files(&files, project.path(), ctx.pattern.clone(), &ctx, None)
            .expect("Analysis failed");

    // 4. Assert results
    for v in &result.violations {
        println!(
            "Violation: from {} to {} in {}",
            v.violation.rule.from,
            v.violation.rule.to,
            v.violation.file_path.display()
        );
    }
    assert_eq!(
        result.violations.len(),
        1,
        "Should have found exactly one violation"
    );
    let violation = &result.violations[0];
    assert!(violation
        .violation
        .file_path
        .to_string_lossy()
        .replace('\\', "/")
        .contains("api/user.ts"));
    assert!(violation
        .violation
        .offensive_import
        .replace('\\', "/")
        .contains("infra/db"));

    // 5. Check health score
    let score = scoring::calculate(&result);
    assert!(
        score.total < 100,
        "Score should be less than 100 due to violation"
    );
    assert!(
        score.grade == architect_linter_pro::metrics::HealthGrade::B
            || score.grade == architect_linter_pro::metrics::HealthGrade::C
            || score.grade == architect_linter_pro::metrics::HealthGrade::D,
        "Score grade should be reasonable"
    );
}

#[test]
fn test_ts_core_complexity_violation() {
    let project = TestProject::new();

    // Limit to 5 lines per function
    project.create_config("MVC", 5, "");

    // Create a file with a long function
    // We use a class method because find_long_functions currently targets methods
    let content = r#"
export class Utils {
    longMethod() {
        console.log("1");
        console.log("2");
        console.log("3");
        console.log("4");
        console.log("5");
        console.log("6");
        console.log("7");
    }
}
"#;
    project.create_file("src/utils.ts", content);

    let ctx = config::load_config(project.path()).expect("Failed to load config");

    let files = project.collect_ts_files();
    let result =
        analyzer::analyze_all_files(&files, project.path(), ctx.pattern.clone(), &ctx, None)
            .expect("Analysis failed");

    // Check for long functions
    assert!(
        !result.long_functions.is_empty(),
        "Should find a long function"
    );
    let long_func = &result.long_functions[0];
    assert_eq!(long_func.name, "longMethod");
    assert!(long_func.lines >= 7);
}
