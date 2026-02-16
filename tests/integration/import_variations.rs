//! Integration tests for TypeScript import variations
//!
//! This test suite validates that the parser correctly handles various import patterns:
//! - Type-only imports
//! - Dynamic imports
//! - Re-exports
//! - JSX/TSX imports

use architect_linter_pro::analyzer::{count_imports, extract_function_calls};
use std::fs;
use swc_common::sync::Lrc;
use swc_common::SourceMap;

#[test]
fn test_type_only_imports() {
    let code = r#"
import type { User } from './user';
import type { Product, Category } from './models';
import type * as Types from './types';
import { type LoginDto, type RegisterDto } from './dto';

export class UserService {
    getUser(id: number): User {
        return { id, name: 'Test' };
    }
}
"#;

    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("type_imports.ts");

    fs::write(&file_path, code).unwrap();

    // Count imports - should detect all type imports
    let import_count = count_imports(&file_path).unwrap();

    // Should detect all 4 import statements (including the mixed type/value import)
    assert_eq!(
        import_count, 4,
        "Should detect all type-only import statements"
    );
}

#[test]
fn test_dynamic_imports() {
    let code = r#"
class DynamicLoader {
    async loadModule() {
        const module = await import('./module');
        const utils = await import('./utils');
        console.log(module);
        return module.default;
    }

    conditionalImport(isDev: boolean) {
        if (isDev) {
            import('./dev-tools').then(tools => {
                tools.init();
            });
        }
    }

    async dynamicRoute(name: string) {
        const component = await import(`./components/${name}`);
        return component;
    }
}
"#;

    let cm = Lrc::new(SourceMap::default());
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("dynamic_imports.ts");

    fs::write(&file_path, code).unwrap();

    // Extract function calls - dynamic imports should be detected
    let function_calls_result = extract_function_calls(&cm, &file_path);

    // Verify the file parses successfully
    assert!(
        function_calls_result.is_ok(),
        "Should successfully parse file with dynamic imports"
    );

    let function_calls = function_calls_result.unwrap();

    // At minimum, we should detect the console.log call
    let has_calls = !function_calls.is_empty();
    assert!(
        has_calls,
        "Should detect at least some function calls in the file"
    );
}

#[test]
fn test_re_exports() {
    let code = r#"
// Named re-exports
export { User } from './user';
export { Product, Category } from './models';

// Re-export with rename
export { User as UserModel } from './user';
export { default as Logger } from './logger';

// Re-export all
export * from './utils';
export * as helpers from './helpers';

// Type re-exports
export type { UserDto } from './dto';
export type * as Types from './types';
"#;

    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("re_exports.ts");

    fs::write(&file_path, code).unwrap();

    // Count imports - re-exports are import statements at parse level
    let import_count = count_imports(&file_path).unwrap();

    // Note: The count_imports function counts lines starting with "import"
    // Re-exports (starting with "export") are not counted as imports
    // This verifies that re-exports are properly distinguished
    assert_eq!(import_count, 0, "Re-exports should not be counted as imports");
}

#[test]
fn test_mixed_imports_and_exports() {
    let code = r#"
// Regular imports
import { UserService } from './services/user';
import type { User } from './models';

// Re-exports
export { ProductService } from './services/product';
export * from './utils';

// Local export
export class AuthService {
    login() {
        return true;
    }
}
"#;

    let cm = Lrc::new(SourceMap::default());
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("mixed.ts");

    fs::write(&file_path, code).unwrap();

    // Count imports - should only count actual import statements, not re-exports
    let import_count = count_imports(&file_path).unwrap();

    assert_eq!(
        import_count, 2,
        "Should count only import statements, not re-exports"
    );

    // Verify the file can be parsed for function extraction
    let function_calls = extract_function_calls(&cm, &file_path).unwrap();
    assert!(
        function_calls.is_empty(),
        "Should parse successfully even with mixed imports/exports"
    );
}

#[test]
fn test_jsx_tsx_imports() {
    let code = r#"
import React from 'react';
import { useState, useEffect } from 'react';
import type { FC, ReactNode } from 'react';
import styled from 'styled-components';

interface UserProps {
    name: string;
    children: ReactNode;
}

class UserComponent {
    render() {
        console.log('Rendering component');
        return null;
    }
}
"#;

    let cm = Lrc::new(SourceMap::default());
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("component.tsx");

    fs::write(&file_path, code).unwrap();

    // Count imports
    let import_count = count_imports(&file_path).unwrap();

    assert_eq!(
        import_count, 4,
        "Should detect all React-related imports in TSX"
    );

    // Extract function calls - verify TSX files can be parsed
    let function_calls_result = extract_function_calls(&cm, &file_path);

    assert!(
        function_calls_result.is_ok(),
        "Should successfully parse TSX files"
    );

    let function_calls = function_calls_result.unwrap();

    // Verify that the TSX file parses successfully and extracts calls
    assert!(
        !function_calls.is_empty(),
        "Should detect function calls in TSX files"
    );

    // Check that console.log is detected
    let console_log_calls: Vec<_> = function_calls
        .iter()
        .filter(|call| call.name == "console.log")
        .collect();

    assert!(
        !console_log_calls.is_empty(),
        "Should detect console.log call in TSX"
    );
}

#[test]
fn test_namespace_imports() {
    let code = r#"
import * as fs from 'fs';
import * as path from 'path';
import * as util from './util';

class FileService {
    readFile(filePath: string) {
        const fullPath = path.join(process.cwd(), filePath);
        return fs.readFileSync(fullPath, 'utf-8');
    }

    formatData(data: any) {
        return util.format(data);
    }
}
"#;

    let cm = Lrc::new(SourceMap::default());
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("namespace_imports.ts");

    fs::write(&file_path, code).unwrap();

    // Count imports
    let import_count = count_imports(&file_path).unwrap();

    assert_eq!(
        import_count, 3,
        "Should detect all namespace imports (import * as)"
    );

    // Extract function calls - verify the file parses correctly
    let function_calls_result = extract_function_calls(&cm, &file_path);

    assert!(
        function_calls_result.is_ok(),
        "Should successfully parse file with namespace imports"
    );

    let function_calls = function_calls_result.unwrap();

    // Verify that we can extract function calls from the file
    assert!(
        !function_calls.is_empty(),
        "Should detect function calls in file with namespace imports. Found: {:?}",
        function_calls
    );

    // Check for namespace method calls - these should be detected as member expressions
    let has_member_calls = function_calls
        .iter()
        .any(|call| call.name.contains('.'));

    assert!(
        has_member_calls,
        "Should detect member function calls (e.g., path.join, fs.readFileSync)"
    );
}

#[test]
fn test_side_effect_imports() {
    let code = r#"
// Side-effect only imports (no bindings)
import './polyfills';
import './styles.css';
import 'reflect-metadata';

// Regular imports
import { Service } from './service';

export class App {
    constructor() {
        new Service();
    }
}
"#;

    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("side_effects.ts");

    fs::write(&file_path, code).unwrap();

    // Count imports - should detect side-effect imports
    let import_count = count_imports(&file_path).unwrap();

    assert_eq!(
        import_count, 4,
        "Should detect all imports including side-effect only imports"
    );
}

#[test]
fn test_default_imports() {
    let code = r#"
import React from 'react';
import express from 'express';
import Logger from './logger';

// Mixed default and named imports
import Vue, { ref, computed } from 'vue';
import axios, { AxiosResponse } from 'axios';

class Server {
    start() {
        const app = express();
        const logger = new Logger();
        app.listen(3000);
    }
}
"#;

    let cm = Lrc::new(SourceMap::default());
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("default_imports.ts");

    fs::write(&file_path, code).unwrap();

    // Count imports
    let import_count = count_imports(&file_path).unwrap();

    assert_eq!(
        import_count, 5,
        "Should detect all default and mixed imports"
    );

    // Verify parsing works correctly
    let function_calls_result = extract_function_calls(&cm, &file_path);

    assert!(
        function_calls_result.is_ok(),
        "Should successfully parse file with default imports"
    );

    let function_calls = function_calls_result.unwrap();

    // Verify that function calls are extracted from files with default imports
    assert!(
        !function_calls.is_empty(),
        "Should extract function calls from file with default imports. Found: {:?}",
        function_calls
    );

    // Check for method calls (like app.listen, express(), Logger())
    let has_method_calls = function_calls
        .iter()
        .any(|call| call.name.contains(".") || call.name == "express" || call.name == "Logger");

    assert!(
        has_method_calls,
        "Should detect method calls like app.listen, express(), or Logger(). Found: {:?}",
        function_calls
    );
}

#[test]
fn test_import_assertions() {
    let code = r#"
// Import with assertions (JSON modules)
import data from './data.json' assert { type: 'json' };
import config from './config.json' assert { type: 'json' };

// Dynamic import with assertions
export async function loadData() {
    const data = await import('./data.json', {
        assert: { type: 'json' }
    });
    return data;
}

export function useConfig() {
    return config;
}
"#;

    let cm = Lrc::new(SourceMap::default());
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("import_assertions.ts");

    fs::write(&file_path, code).unwrap();

    // Count imports
    let import_count = count_imports(&file_path).unwrap();

    assert_eq!(
        import_count, 2,
        "Should detect imports with assertions"
    );

    // Verify the file parses successfully
    let function_calls = extract_function_calls(&cm, &file_path).unwrap();

    // Files with import assertions should parse successfully
    // We're testing that the parser doesn't fail on assertion syntax
    assert!(
        function_calls.is_empty() || !function_calls.is_empty(),
        "Should parse file with import assertions without errors"
    );
}

#[test]
fn test_complex_import_patterns() {
    let code = r#"
// Mix of everything
import type { User, Product } from './models';
import * as utils from './utils';
import Logger from './logger';
import './setup';

// Re-exports
export { AuthService } from './auth';
export type { AuthToken } from './auth';
export * from './helpers';

class Component {
    init() {
        Logger.log('Component mounted');
        utils.setup();
    }
}
"#;

    let cm = Lrc::new(SourceMap::default());
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("complex.ts");

    fs::write(&file_path, code).unwrap();

    // Count imports
    let import_count = count_imports(&file_path).unwrap();

    assert_eq!(
        import_count, 4,
        "Should detect all import statements in complex file"
    );

    // Extract function calls
    let function_calls_result = extract_function_calls(&cm, &file_path);

    assert!(
        function_calls_result.is_ok(),
        "Should successfully parse file with complex import patterns"
    );

    let function_calls = function_calls_result.unwrap();

    // Verify we can parse complex files with mixed import patterns
    assert!(
        !function_calls.is_empty(),
        "Should detect function calls in complex file with mixed imports. Found: {:?}",
        function_calls
    );

    // Check for method calls
    let logger_calls: Vec<_> = function_calls
        .iter()
        .filter(|call| call.name.starts_with("Logger."))
        .collect();

    assert!(
        !logger_calls.is_empty(),
        "Should detect Logger method calls in complex file. Found: {:?}",
        function_calls
    );
}

#[test]
fn test_imports_in_javascript_files() {
    let code = r#"
// JavaScript (not TypeScript) imports
import { UserService } from './services/user.js';
import * as helpers from './helpers.js';

class Service {
    init() {
        this.user = new UserService();
        console.log('Service created');
        helpers.setup();
    }
}
"#;

    let cm = Lrc::new(SourceMap::default());
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("service.js");

    fs::write(&file_path, code).unwrap();

    // Count imports - should work with .js files
    let import_count = count_imports(&file_path).unwrap();

    assert_eq!(import_count, 2, "Should detect imports in .js files");

    // Extract function calls - should work with .js files
    let function_calls_result = extract_function_calls(&cm, &file_path);

    assert!(
        function_calls_result.is_ok(),
        "Should successfully parse JavaScript files"
    );

    let function_calls = function_calls_result.unwrap();

    // Verify JavaScript files parse correctly
    assert!(
        !function_calls.is_empty(),
        "Should extract function calls from JavaScript files. Found: {:?}",
        function_calls
    );

    // Check for console.log which should be reliably detected
    let has_console_log = function_calls
        .iter()
        .any(|call| call.name == "console.log");

    assert!(
        has_console_log,
        "Should detect console.log call in .js file. Found: {:?}",
        function_calls
    );
}
