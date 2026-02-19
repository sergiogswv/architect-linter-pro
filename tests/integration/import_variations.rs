//! Integration tests for TypeScript import variations
//!
//! This test suite validates that the parser correctly handles various import patterns:
//! - Type-only imports
//! - Dynamic imports
//! - Re-exports
//! - JSX/TSX imports

use architect_linter_pro::analyzer::metrics::count_imports;
use std::fs;

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

    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("dynamic_imports.ts");

    fs::write(&file_path, code).unwrap();
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
    assert_eq!(
        import_count, 0,
        "Re-exports should not be counted as imports"
    );
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

    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("mixed.ts");

    fs::write(&file_path, code).unwrap();

    // Count imports - should only count actual import statements, not re-exports
    let import_count = count_imports(&file_path).unwrap();

    assert_eq!(
        import_count, 2,
        "Should count only import statements, not re-exports"
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

    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("component.tsx");

    fs::write(&file_path, code).unwrap();

    // Count imports
    let import_count = count_imports(&file_path).unwrap();

    assert_eq!(
        import_count, 4,
        "Should detect all React-related imports in TSX"
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

    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("namespace_imports.ts");

    fs::write(&file_path, code).unwrap();

    // Count imports
    let import_count = count_imports(&file_path).unwrap();

    assert_eq!(
        import_count, 3,
        "Should detect all namespace imports (import * as)"
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

    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("default_imports.ts");

    fs::write(&file_path, code).unwrap();

    // Count imports
    let import_count = count_imports(&file_path).unwrap();

    assert_eq!(
        import_count, 5,
        "Should detect all default and mixed imports"
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

    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("import_assertions.ts");

    fs::write(&file_path, code).unwrap();

    // Count imports
    let import_count = count_imports(&file_path).unwrap();

    assert_eq!(import_count, 2, "Should detect imports with assertions");
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

    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("complex.ts");

    fs::write(&file_path, code).unwrap();

    // Count imports
    let import_count = count_imports(&file_path).unwrap();

    assert_eq!(
        import_count, 4,
        "Should detect all import statements in complex file"
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

    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("service.js");

    fs::write(&file_path, code).unwrap();

    // Count imports - should work with .js files
    let import_count = count_imports(&file_path).unwrap();

    assert_eq!(import_count, 2, "Should detect imports in .js files");
}
