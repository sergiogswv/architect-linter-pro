/// Integration tests for multi-language parsers
///
/// These tests validate that each parser can correctly:
/// - Extract imports/dependencies from source code
/// - Detect architectural violations
/// - Handle language-specific syntax
///
/// Covers: TypeScript, JavaScript, Python, PHP
use architect_linter_pro::config::{ArchPattern, ForbiddenRule, Framework, LinterContext};
use architect_linter_pro::parsers::{ArchitectParser, Language};
use std::path::Path;

// ============================================================================
// Helper Functions
// ============================================================================

/// Create a minimal LinterContext for testing
fn create_test_context(rules: Vec<ForbiddenRule>) -> LinterContext {
    LinterContext {
        max_lines: 100,
        framework: Framework::Unknown,
        pattern: ArchPattern::MVC,
        forbidden_imports: rules,
        ignored_paths: vec![],
        ai_configs: vec![],
        ..Default::default()
    }
}

/// Create a forbidden rule
fn forbidden_rule(from: &str, to: &str) -> ForbiddenRule {
    ForbiddenRule {
        from: from.to_string(),
        to: to.to_string(),
        severity: None,
        reason: None,
    }
}

// ============================================================================
// Language Detection Tests
// ============================================================================

#[test]
fn test_language_from_extension_typescript() {
    assert_eq!(Language::from_extension("ts"), Some(Language::TypeScript));
    assert_eq!(Language::from_extension("tsx"), Some(Language::TypeScript));
}

#[test]
fn test_language_from_extension_javascript() {
    assert_eq!(Language::from_extension("js"), Some(Language::JavaScript));
    assert_eq!(Language::from_extension("jsx"), Some(Language::JavaScript));
}

#[test]
fn test_language_from_extension_python() {
    assert_eq!(Language::from_extension("py"), Some(Language::Python));
}

#[test]
fn test_language_from_extension_php() {
    assert_eq!(Language::from_extension("php"), Some(Language::Php));
}

#[test]
fn test_language_from_extension_unknown() {
    assert_eq!(Language::from_extension("unknown"), None);
    assert_eq!(Language::from_extension("txt"), None);
}

// ============================================================================
// TypeScript Parser Tests
// ============================================================================

#[test]
fn test_typescript_extract_imports_basic() {
    use architect_linter_pro::parsers::typescript::TypeScriptParser;

    let parser = TypeScriptParser::new();
    let source = r#"
        import { User } from './models/user';
        import { Product } from './models/product';
        import axios from 'axios';
    "#;

    let imports = parser
        .extract_imports(source, Path::new("test.ts"))
        .unwrap();

    assert_eq!(imports.len(), 3);
    assert!(imports.iter().any(|i| i.source == "./models/user"));
    assert!(imports.iter().any(|i| i.source == "./models/product"));
    assert!(imports.iter().any(|i| i.source == "axios"));
}

#[test]
fn test_typescript_extract_imports_various_formats() {
    use architect_linter_pro::parsers::typescript::TypeScriptParser;

    let parser = TypeScriptParser::new();
    let source = r#"
        import React from 'react';
        import type { User } from './types';
        import * as utils from './utils';
        import { a, b, c } from './helpers';
    "#;

    let imports = parser
        .extract_imports(source, Path::new("test.ts"))
        .unwrap();

    assert!(imports.len() >= 4);
}

#[test]
fn test_typescript_detect_violation() {
    use architect_linter_pro::parsers::typescript::TypeScriptParser;

    let parser = TypeScriptParser::new();
    let source = r#"
        import { UserRepository } from '../repository/user';
    "#;

    let context = create_test_context(vec![forbidden_rule("/controller/", "/repository/")]);

    let violations = parser
        .find_violations(
            source,
            Path::new("src/controller/user.controller.ts"),
            &context,
        )
        .unwrap();

    assert!(violations.len() > 0, "Should detect forbidden import");
}

// ============================================================================
// JavaScript Parser Tests
// ============================================================================

#[test]
fn test_javascript_extract_imports_es6() {
    use architect_linter_pro::parsers::typescript::TypeScriptParser;

    let parser = TypeScriptParser::new();
    let source = r#"
        import express from 'express';
        import { Router } from 'express';
        import db from './db';
    "#;

    let imports = parser
        .extract_imports(source, Path::new("test.js"))
        .unwrap();

    assert_eq!(imports.len(), 3);
}

#[test]
fn test_javascript_extract_imports_commonjs() {
    use architect_linter_pro::parsers::typescript::TypeScriptParser;

    let parser = TypeScriptParser::new();
    // Note: Tree-sitter might not parse require() as import_statement
    // This test validates the parser doesn't crash on CommonJS syntax
    let source = r#"
        const express = require('express');
        const db = require('./db');
    "#;

    let result = parser.extract_imports(source, Path::new("test.js"));
    assert!(result.is_ok(), "Parser should handle CommonJS syntax");
}

// ============================================================================
// Python Parser Tests
// ============================================================================

#[test]
fn test_python_extract_imports_basic() {
    use architect_linter_pro::parsers::python::PythonParser;

    let parser = PythonParser::new();
    let source = r#"
import os
import sys
from typing import List, Dict
from models.user import User
from .local import helper
    "#;

    let imports = parser
        .extract_imports(source, Path::new("test.py"))
        .unwrap();

    assert!(imports.len() >= 3, "Should extract at least 3 imports");
    assert!(imports
        .iter()
        .any(|i| i.source.contains("os") || i.source.contains("sys")));
}

#[test]
fn test_python_extract_imports_various_formats() {
    use architect_linter_pro::parsers::python::PythonParser;

    let parser = PythonParser::new();
    let source = r#"
import numpy as np
from django.db import models
from ..parent import something
from . import sibling
    "#;

    let result = parser.extract_imports(source, Path::new("test.py"));
    assert!(
        result.is_ok(),
        "Python parser should handle various import formats"
    );
}

#[test]
fn test_python_detect_violation() {
    use architect_linter_pro::parsers::python::PythonParser;

    let parser = PythonParser::new();
    let source = r#"
from infrastructure.database import Database
    "#;

    let context = create_test_context(vec![forbidden_rule("/domain/", "/infrastructure/")]);

    let violations = parser
        .find_violations(source, Path::new("src/domain/user.py"), &context)
        .unwrap();

    assert!(violations.len() > 0, "Should detect forbidden import");
}

// ============================================================================
// PHP Parser Tests
// ============================================================================

#[test]
fn test_php_extract_imports_basic() {
    use architect_linter_pro::parsers::php::PhpParser;

    let parser = PhpParser::new();
    let source = r#"
<?php

use App\Models\User;
use App\Services\UserService;
use Illuminate\Support\Facades\DB;

class UserController {
}
    "#;

    let imports = parser
        .extract_imports(source, Path::new("test.php"))
        .unwrap();

    assert!(imports.len() >= 2, "Should extract at least 2 imports");
}

#[test]
fn test_php_extract_imports_various_formats() {
    use architect_linter_pro::parsers::php::PhpParser;

    let parser = PhpParser::new();
    let source = r#"
<?php

use App\Models\User as UserModel;
use App\Models\{Product, Category};
use function App\Helpers\format_date;
    "#;

    let imports = parser
        .extract_imports(source, Path::new("test.php"))
        .unwrap();

    assert!(imports.len() >= 1);
}

#[test]
fn test_php_detect_violation() {
    use architect_linter_pro::parsers::php::PhpParser;

    let parser = PhpParser::new();
    let source = r#"
<?php

namespace App\Domain;

use App\Infrastructure\Database;

class User {
}
    "#;

    let context = create_test_context(vec![forbidden_rule("/Domain/", "/Infrastructure/")]);

    let violations = parser
        .find_violations(source, Path::new("src/Domain/User.php"), &context)
        .unwrap();

    assert!(violations.len() > 0, "Should detect forbidden import");
}

// ============================================================================
// Parser Factory Tests
// ============================================================================

#[test]
fn test_get_parser_for_typescript() {
    use architect_linter_pro::parsers::get_parser_for_file;

    let parser = get_parser_for_file(Path::new("test.ts"));
    assert!(parser.is_some(), "Should return parser for .ts");
}

#[test]
fn test_get_parser_for_javascript() {
    use architect_linter_pro::parsers::get_parser_for_file;

    let parser = get_parser_for_file(Path::new("test.js"));
    assert!(parser.is_some(), "Should return parser for .js");
}

#[test]
fn test_get_parser_for_python() {
    use architect_linter_pro::parsers::get_parser_for_file;

    let parser = get_parser_for_file(Path::new("test.py"));
    assert!(parser.is_some(), "Should return parser for .py");
}

#[test]
fn test_get_parser_for_php() {
    use architect_linter_pro::parsers::get_parser_for_file;

    let parser = get_parser_for_file(Path::new("test.php"));
    assert!(parser.is_some(), "Should return parser for .php");
}

#[test]
fn test_get_parser_for_unknown() {
    use architect_linter_pro::parsers::get_parser_for_file;

    let parser = get_parser_for_file(Path::new("test.txt"));
    assert!(parser.is_none(), "Should return None for unknown extension");
}

// ============================================================================
// Edge Cases and Error Handling
// ============================================================================

#[test]
fn test_typescript_empty_file() {
    use architect_linter_pro::parsers::typescript::TypeScriptParser;

    let parser = TypeScriptParser::new();
    let imports = parser.extract_imports("", Path::new("test.ts")).unwrap();

    assert_eq!(imports.len(), 0, "Empty file should have no imports");
}

#[test]
fn test_python_empty_file() {
    use architect_linter_pro::parsers::python::PythonParser;

    let parser = PythonParser::new();
    let imports = parser.extract_imports("", Path::new("test.py")).unwrap();

    assert_eq!(imports.len(), 0, "Empty file should have no imports");
}

#[test]
fn test_typescript_syntax_error_handling() {
    use architect_linter_pro::parsers::typescript::TypeScriptParser;

    let parser = TypeScriptParser::new();
    let source = "import { from '}"; // Invalid syntax

    // Parser should not panic on invalid syntax
    let result = parser.extract_imports(source, Path::new("test.ts"));
    assert!(
        result.is_ok(),
        "Parser should handle syntax errors gracefully"
    );
}

#[test]
fn test_python_no_violations_clean_code() {
    use architect_linter_pro::parsers::python::PythonParser;

    let parser = PythonParser::new();
    let source = r#"
from models.user import User

class UserService:
    pass
    "#;

    let context = create_test_context(vec![forbidden_rule("/controller/", "/repository/")]);

    let violations = parser
        .find_violations(source, Path::new("src/service/user_service.py"), &context)
        .unwrap();

    assert_eq!(violations.len(), 0, "Clean code should have no violations");
}

// ============================================================================
// Cross-Language Consistency Tests
// ============================================================================

#[test]
fn test_all_parsers_handle_empty_files() {
    use architect_linter_pro::parsers::get_parser_for_file;

    let test_files = vec![
        "test.ts",
        "test.js",
        "test.py",
        "test.php",
    ];

    for file in test_files {
        if let Some(parser) = get_parser_for_file(Path::new(file)) {
            let result = parser.extract_imports("", Path::new(file));
            assert!(result.is_ok(), "{} parser should handle empty files", file);
            let imports = result.unwrap();
            assert_eq!(
                imports.len(),
                0,
                "{} parser should have no imports in empty file",
                file
            );
        }
    }
}
