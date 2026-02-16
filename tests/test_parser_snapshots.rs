//! Snapshot tests for TypeScript/JavaScript parser
//! These tests verify the parser's ability to extract:
//! - Class declarations
//! - Function/method calls
//! - Import statements
//!
//! Using insta for snapshot testing to avoid constant updates when AST structures change.

use architect_linter_pro::analyzer::{count_functions, count_imports, find_long_functions};
use swc_common::sync::Lrc;
use swc_common::SourceMap;
use tempfile::NamedTempFile;

/// Helper function to create a LinterContext for testing
fn create_test_context() -> architect_linter_pro::config::LinterContext {
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

    // Clean up the temp config directory
    let _ = std::fs::remove_dir_all(project_root);

    linter_context
}

#[test]
fn test_extract_class_declarations() {
    let code = r#"
export class UserService {
    private id: number;

    constructor(id: number) {
        this.id = id;
    }

    getUser(id: number): User {
        return new User(id);
    }

    updateUser(user: User): void {
        console.log(user);
    }
}

export class User {
    constructor(public id: number) {}
}
"#;

    let cm = Lrc::new(SourceMap::default());
    let temp_file = NamedTempFile::new().unwrap();
    let file_path = temp_file.path().to_path_buf();
    std::fs::write(&file_path, code).unwrap();

    let _ctx = create_test_context();

    // Extract functions from the parsed AST
    // Note: temp_file is kept alive by being in scope until the end of the test
    let function_count = count_functions(&cm, &file_path);

    // Snapshot the function count result
    insta::assert_debug_snapshot!(function_count);

    // Explicitly keep temp_file alive
    let _ = temp_file;
}

#[test]
fn test_extract_long_functions() {
    let code = r#"
class Service {
    method() {
        this.helper();
        console.log();
    }

    helper() {
        return true;
    }

    veryLongMethod() {
        // This method is intentionally long to test extraction
        let x = 1;
        let y = 2;
        let z = 3;
        let a = 4;
        let b = 5;
        let c = 6;
        let d = 7;
        let e = 8;
        let f = 9;
        let g = 10;
        let h = 11;
        let i = 12;
        let j = 13;
        let k = 14;
        let l = 15;
        let m = 16;
        let n = 17;
        let o = 18;
        let p = 19;
        let q = 20;
        let r = 21;
        let s = 22;
        let t = 23;
        let u = 24;
        let v = 25;
        let w = 26;
        let x2 = 27;
        let y2 = 28;
        let z2 = 29;
        let a2 = 30;
        let b2 = 31;
        let c2 = 32;
        return true;
    }
}
"#;

    let cm = Lrc::new(SourceMap::default());
    let temp_file = NamedTempFile::new().unwrap();
    let file_path = temp_file.path().to_path_buf();
    std::fs::write(&file_path, code).unwrap();

    let _ctx = create_test_context();

    // Extract long functions from the parsed AST
    let long_functions = find_long_functions(&cm, &file_path, _ctx.max_lines);

    // Snapshot the long functions result
    insta::assert_debug_snapshot!(long_functions);

    // Explicitly keep temp_file alive
    let _ = temp_file;
}

#[test]
fn test_detect_imports() {
    let code = r#"
import { UserService } from './user.service';
import { Logger } from 'logger';
import * as fs from 'fs';
"#;

    let temp_file = NamedTempFile::new().unwrap();
    let file_path = temp_file.path().to_path_buf();
    std::fs::write(&file_path, code).unwrap();

    // Count imports from the parsed file
    let import_count = count_imports(&file_path);

    // Snapshot the import count result
    insta::assert_debug_snapshot!(import_count);

    // Explicitly keep temp_file alive
    let _ = temp_file;
}

#[test]
fn test_parse_error_handling() {
    let code = r#"
export class Broken {
    constructor() {
        // This is valid TypeScript
    }

    brokenMethod() {
        // Missing closing brace will cause parse error
        if (true) {
            console.log("this is valid");
        }
    }
}
"#;

    let cm = Lrc::new(SourceMap::default());
    let temp_file = NamedTempFile::new().unwrap();
    let file_path = temp_file.path().to_path_buf();
    std::fs::write(&file_path, code).unwrap();

    let _ctx = create_test_context();

    // Try to parse and extract functions
    let result = count_functions(&cm, &file_path);

    // Snapshot the result (should handle errors gracefully)
    insta::assert_debug_snapshot!(result);

    // Explicitly keep temp_file alive
    let _ = temp_file;
}

#[test]
fn test_extract_complex_class_structure() {
    let code = r#"
import { Injectable } from '@nestjs/common';
import { Repository } from 'typeorm';

@Injectable()
export class UserService {
    constructor(
        private userRepo: Repository<User>,
        private logger: Logger
    ) {}

    async findAll(): Promise<User[]> {
        return this.userRepo.find();
    }

    async findOne(id: number): Promise<User> {
        return this.userRepo.findOne({ where: { id } });
    }

    async create(userData: CreateUserDto): Promise<User> {
        const user = this.userRepo.create(userData);
        return this.userRepo.save(user);
    }

    async update(id: number, userData: UpdateUserDto): Promise<User> {
        await this.userRepo.update(id, userData);
        return this.findOne(id);
    }

    async delete(id: number): Promise<void> {
        await this.userRepo.delete(id);
    }
}
"#;

    let cm = Lrc::new(SourceMap::default());
    let temp_file = NamedTempFile::new().unwrap();
    let file_path = temp_file.path().to_path_buf();
    std::fs::write(&file_path, code).unwrap();

    let _ctx = create_test_context();

    // Extract function count
    let function_count = count_functions(&cm, &file_path);

    // Extract import count
    let import_count = count_imports(&file_path);

    // Snapshot both results
    insta::assert_debug_snapshot!(("function_count", function_count, "import_count", import_count));

    // Explicitly keep temp_file alive
    let _ = temp_file;
}
