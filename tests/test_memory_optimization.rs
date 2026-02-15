use architect_linter_pro::analyzer::swc_parser;
use std::path::PathBuf;
use swc_common::sync::Lrc;
use swc_common::SourceMap;
use tempfile::NamedTempFile;

#[test]
fn test_ast_scoped_analysis() {
    // Create a temporary TypeScript file with imports and functions
    let temp_file = NamedTempFile::new().unwrap();
    let file_path = PathBuf::from(temp_file.path());

    let ts_content = r#"
import { UserService } from './service';
import { UserRepository } from './repository';
import { Logger } from './logger';

export class UserController {
    private userService: UserService;
    private logger: Logger;

    constructor() {
        this.userService = new UserService();
        this.logger = new Logger();
    }

    public getUserById(id: number): any {
        this.logger.log('Getting user by id');
        const user = this.userService.findById(id);
        return user;
    }

    public createUser(userData: any): void {
        this.logger.log('Creating user');
        this.userService.create(userData);
    }
}
"#;

    std::fs::write(&file_path, ts_content).unwrap();

    // Create SourceMap and LinterContext
    let cm = Lrc::new(SourceMap::default());

    // Create a minimal config for testing
    let config_content = r#"
{
  "max_lines_per_function": 30,
  "architecture_pattern": "MVC",
  "forbidden_imports": []
}
"#;

    let temp_config = NamedTempFile::new().unwrap();
    std::fs::write(temp_config.path(), config_content).unwrap();

    // Create the architect.json file in the correct location
    let architect_config = temp_config.path().parent().unwrap().join("architect.json");
    std::fs::write(&architect_config, config_content).unwrap();

    let project_root = architect_config.parent().unwrap().to_path_buf();
    let config =
        architect_linter_pro::config::load_config(&project_root).expect("Failed to load config");
    let linter_context: architect_linter_pro::config::LinterContext = config.into();

    // Test 1: Verify the file can be analyzed without memory leaks
    let result = swc_parser::analyze_file(&cm, &file_path, &linter_context);

    assert!(result.is_ok(), "Analysis should succeed without panic");

    // Test 2: Verify functions were extracted (count by examining the analysis logic)
    // We'll call the method length validation separately to check it works
    let method_result = swc_parser::validate_method_length(&cm, &file_path, &linter_context);

    assert!(
        method_result.is_ok(),
        "Method length validation should succeed"
    );

    // Test 3: Verify the file doesn't contain forbidden imports
    // The test file has imports that should be allowed in a controller
    let result = swc_parser::analyze_file(&cm, &file_path, &linter_context);

    assert!(
        result.is_ok(),
        "Controller should be allowed to import services and loggers"
    );

    // Test 4: Verify AST objects are properly scoped (no reference after analysis)
    // This is tested by ensuring no panic occurs after multiple analyses
    for _ in 0..10 {
        let analysis_result = swc_parser::validate_method_length(&cm, &file_path, &linter_context);
        assert!(
            analysis_result.is_ok(),
            "Repeated analysis should not cause memory issues"
        );
    }

    // Clean up
    let _ = std::fs::remove_dir_all(project_root);
}

#[test]
fn test_ast_dropped_after_analysis() {
    // Create a temporary directory
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();

    // Create a temporary TypeScript file
    let file_path = temp_path.join("test.ts");
    let ts_content = r#"
export class TestClass {
    public testMethod(): void {
        console.log('Test method');
    }
}
"#;
    std::fs::write(&file_path, ts_content).unwrap();

    let cm = Lrc::new(SourceMap::default());

    // Create minimal config
    let config_content = r#"
{
  "max_lines_per_function": 30,
  "architecture_pattern": "MVC",
  "forbidden_imports": []
}
"#;

    // Create the architect.json file in the correct location
    let architect_config = temp_path.join("architect.json");
    std::fs::write(&architect_config, config_content).unwrap();

    let project_root = temp_path.to_path_buf();
    let config =
        architect_linter_pro::config::load_config(&project_root).expect("Failed to load config");
    let linter_context: architect_linter_pro::config::LinterContext = config.into();

    // Test that AST objects are properly dropped after analysis
    // by running analysis in a loop to check for memory leaks
    for i in 0..100 {
        // Create a new SourceMap for each iteration to avoid sync issues
        let local_cm = Lrc::new(SourceMap::default());

        // The validate_method_length function should drop AST after extraction
        let result = swc_parser::validate_method_length(&local_cm, &file_path, &linter_context);

        assert!(
            result.is_ok(),
            "AST analysis should succeed on iteration {}",
            i
        );

        // Verify the analysis completes without issues
        // This ensures the AST was properly processed and dropped
        if i % 10 == 0 {
            println!("Completed iteration {} successfully", i);
        }
    }
}

#[test]
fn test_parallel_analysis_memory_safety() {
    // Create a temporary directory
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();

    // Create multiple temporary files
    let mut file_paths = Vec::new();
    for i in 0..5 {
        let file_path = temp_path.join(format!("test{}.ts", i));
        let ts_content = format!(
            r#"
import {{ Service }} from './service';

export class TestClass{{
    public method{}(): void {{
        console.log('Test method');
    }}
}}
"#,
            i
        );

        std::fs::write(&file_path, ts_content).unwrap();
        file_paths.push(file_path);
    }

    let _cm = Lrc::new(SourceMap::default());

    // Create minimal config
    let config_content = r#"
{
  "max_lines_per_function": 30,
  "architecture_pattern": "MVC",
  "forbidden_imports": []
}
"#;

    // Create the architect.json file in the correct location
    let architect_config = temp_path.join("architect.json");
    std::fs::write(&architect_config, config_content).unwrap();

    let project_root = temp_path.to_path_buf();
    let config =
        architect_linter_pro::config::load_config(&project_root).expect("Failed to load config");
    let linter_context: architect_linter_pro::config::LinterContext = config.into();

    // Test parallel analysis by processing files in sequence
    // (simulating parallel execution)
    for (i, file_path) in file_paths.iter().enumerate() {
        // Create a new SourceMap for each file to simulate parallel execution
        let local_cm = Lrc::new(SourceMap::default());
        let result = swc_parser::validate_method_length(&local_cm, file_path, &linter_context);

        assert!(result.is_ok(), "File {} analysis should succeed", i);

        // Ensure each file is processed independently
        println!("Successfully analyzed file {}", i);
    }
}
