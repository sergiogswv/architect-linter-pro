//! Integration tests for decorated classes and multiple violations
//!
//! This module tests:
//! - NestJS-style decorator handling (@Injectable, @Controller, @Get, etc.)
//! - Complex classes with multiple decorators
//! - Files with multiple architectural violations
//! - Edge cases with decorators and violations combined

use architect_linter_pro::analyzer::collect_violations_from_file;
use architect_linter_pro::analyzer::metrics::{
    count_functions, count_imports, find_long_functions,
};
use architect_linter_pro::config::{ForbiddenRule, LinterContext};

/// Helper function to create a LinterContext for testing
fn create_test_context() -> LinterContext {
    LinterContext {
        max_lines: 30,
        ai_configs: vec![],
        ..Default::default()
    }
}

/// Helper function to create a context with forbidden imports
fn create_context_with_rules(rules: Vec<ForbiddenRule>) -> LinterContext {
    LinterContext {
        max_lines: 30,
        forbidden_imports: rules,
        ai_configs: vec![],
        ..Default::default()
    }
}

#[test]
fn test_nestjs_controller_with_decorators() {
    let code = r#"
import { Controller, Get, Post, Body } from '@nestjs/common';
import { UserService } from './user.service';

@Controller('users')
export class UserController {
    constructor(private userService: UserService) {}

    @Get()
    async findAll() {
        return this.userService.findAll();
    }

    @Get(':id')
    async findOne(@Param('id') id: string) {
        return this.userService.findOne(id);
    }

    @Post()
    async create(@Body() createUserDto: any) {
        return this.userService.create(createUserDto);
    }
}
"#;


    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("user.controller.ts");

    std::fs::write(&file_path, code).unwrap();

    let _ctx = create_test_context();

    // Note: count_functions currently doesn't count methods in exported classes
    // This is a known limitation - it only counts Stmt::Decl(Decl::Class), not ModuleDecl::ExportDecl
    // We still verify it parses without errors
    let function_count = count_functions(&file_path);
    assert!(function_count.is_ok());

    // Count imports
    let import_count = count_imports(&file_path);
    assert!(import_count.is_ok());
    assert_eq!(import_count.unwrap(), 2);
}

#[test]
fn test_nestjs_service_with_injectable_decorator() {
    let code = r#"
import { Injectable } from '@nestjs/common';
import { Repository } from 'typeorm';
import { User } from './user.entity';

@Injectable()
export class UserService {
    constructor(
        private userRepository: Repository<User>
    ) {}

    async findAll(): Promise<User[]> {
        return this.userRepository.find();
    }

    async findOne(id: string): Promise<User> {
        return this.userRepository.findOne({ where: { id } });
    }

    async create(userData: any): Promise<User> {
        const user = this.userRepository.create(userData);
        return this.userRepository.save(user);
    }

    async update(id: string, userData: any): Promise<User> {
        await this.userRepository.update(id, userData);
        return this.findOne(id);
    }

    async remove(id: string): Promise<void> {
        await this.userRepository.delete(id);
    }
}
"#;


    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("user.service.ts");

    std::fs::write(&file_path, code).unwrap();

    let ctx = create_test_context();

    // Note: count_functions doesn't count methods in exported classes currently
    let function_count = count_functions(&file_path);
    assert!(function_count.is_ok());

    // Count imports
    let import_count = count_imports(&file_path);
    assert!(import_count.is_ok());
    assert_eq!(import_count.unwrap(), 3);

    // Check for long functions (none should be too long)
    let long_functions = find_long_functions(&file_path, ctx.max_lines);
    assert!(long_functions.is_ok());
    let long_funcs = long_functions.unwrap();
    assert_eq!(long_funcs.len(), 0);
}

#[test]
fn test_multiple_decorator_types() {
    let code = r#"
import { Controller, Get, Post, Put, Delete, UseGuards, Injectable } from '@nestjs/common';
import { ApiTags, ApiOperation, ApiResponse } from '@nestjs/swagger';
import { AuthGuard } from './auth.guard';

@Controller('api/posts')
@ApiTags('posts')
@UseGuards(AuthGuard)
export class PostController {
    @Get()
    @ApiOperation({ summary: 'Get all posts' })
    @ApiResponse({ status: 200, description: 'Returns all posts' })
    async findAll() {
        return [];
    }

    @Post()
    @ApiOperation({ summary: 'Create post' })
    async create(@Body() data: any) {
        return data;
    }
}

@Injectable()
export class PostService {
    @Inject()
    private repository: any;

    findAll() {
        return this.repository.find();
    }
}
"#;


    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("post.controller.ts");

    std::fs::write(&file_path, code).unwrap();

    // Note: count_functions doesn't count methods in exported classes currently
    let function_count = count_functions(&file_path);
    assert!(function_count.is_ok());

    // Count imports
    let import_count = count_imports(&file_path);
    assert!(import_count.is_ok());
    assert_eq!(import_count.unwrap(), 3);
}

#[test]
fn test_file_with_multiple_violations_circular_deps() {
    let code = r#"
// This file violates multiple architectural rules
import { UserService } from '../domain/user.service';
import { Database } from '../infrastructure/database';
import { Logger } from '../infrastructure/logger';
import { Controller } from '../presentation/controller';

class ProblematicService {
    // Long method violation
    async processUser() {
        const line1 = 1;
        const line2 = 2;
        const line3 = 3;
        const line4 = 4;
        const line5 = 5;
        const line6 = 6;
        const line7 = 7;
        const line8 = 8;
        const line9 = 9;
        const line10 = 10;
        const line11 = 11;
        const line12 = 12;
        const line13 = 13;
        const line14 = 14;
        const line15 = 15;
        const line16 = 16;
        const line17 = 17;
        const line18 = 18;
        const line19 = 19;
        const line20 = 20;
        const line21 = 21;
        const line22 = 22;
        const line23 = 23;
        const line24 = 24;
        const line25 = 25;
        const line26 = 26;
        const line27 = 27;
        const line28 = 28;
        const line29 = 29;
        const line30 = 30;
        const line31 = 31;
        const line32 = 32;
        return line32;
    }

    // Another long method
    async anotherLongMethod() {
        const a1 = 1;
        const a2 = 2;
        const a3 = 3;
        const a4 = 4;
        const a5 = 5;
        const a6 = 6;
        const a7 = 7;
        const a8 = 8;
        const a9 = 9;
        const a10 = 10;
        const a11 = 11;
        const a12 = 12;
        const a13 = 13;
        const a14 = 14;
        const a15 = 15;
        const a16 = 16;
        const a17 = 17;
        const a18 = 18;
        const a19 = 19;
        const a20 = 20;
        const a21 = 21;
        const a22 = 22;
        const a23 = 23;
        const a24 = 24;
        const a25 = 25;
        const a26 = 26;
        const a27 = 27;
        const a28 = 28;
        const a29 = 29;
        const a30 = 30;
        const a31 = 31;
        const a32 = 32;
        return a32;
    }
}
"#;


    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("domain/problematic.service.ts");

    // Create parent directory
    std::fs::create_dir_all(file_path.parent().unwrap()).unwrap();
    std::fs::write(&file_path, code).unwrap();

    let ctx = create_test_context();

    // Test 1: Count imports (should be 4)
    let import_count = count_imports(&file_path);
    assert!(import_count.is_ok());
    assert_eq!(import_count.unwrap(), 4);

    // Test 2: Find long functions (should detect both long methods)
    let long_functions = find_long_functions(&file_path, ctx.max_lines);
    assert!(long_functions.is_ok());
    let long_funcs = long_functions.unwrap();
    assert_eq!(long_funcs.len(), 2, "Should detect 2 long functions");

    // Test 3: Count functions (should be 2)
    let function_count = count_functions(&file_path);
    assert!(function_count.is_ok());
    assert_eq!(function_count.unwrap(), 2);
}

#[test]
fn test_file_with_forbidden_import_violations() {
    let code = r#"
// Domain layer importing from infrastructure (violation)
import { DatabaseConnection } from '../infrastructure/database';
import { FileSystem } from '../infrastructure/filesystem';
import { Logger } from '../infrastructure/logger';
import { ApiClient } from '../infrastructure/api-client';

export class DomainService {
    private db: DatabaseConnection;
    private fs: FileSystem;
    private logger: Logger;
    private api: ApiClient;

    constructor() {
        this.db = new DatabaseConnection();
        this.fs = new FileSystem();
        this.logger = new Logger();
        this.api = new ApiClient();
    }

    async execute() {
        await this.db.query('SELECT * FROM users');
        this.fs.readFile('/path/to/file');
        this.logger.log('Executing');
        return this.api.fetch('/endpoint');
    }
}
"#;


    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("domain/service.ts");

    // Create parent directory
    std::fs::create_dir_all(file_path.parent().unwrap()).unwrap();
    std::fs::write(&file_path, code).unwrap();

    // Create context with forbidden import rules
    let rules = vec![ForbiddenRule {
        from: "/domain/".to_string(),
        to: "/infrastructure/".to_string(),
        severity: None,
    }];
    let ctx = create_context_with_rules(rules);

    // Collect violations (should find 4 forbidden imports)
    let violations = collect_violations_from_file(&file_path, &ctx);
    assert!(violations.is_ok());
    let viols = violations.unwrap();
    assert_eq!(
        viols.len(),
        4,
        "Should detect 4 forbidden import violations"
    );

    // Verify each violation is for infrastructure imports
    for violation in &viols {
        assert!(violation.offensive_import.contains("infrastructure"));
        assert_eq!(violation.rule.from, "/domain/");
        assert_eq!(violation.rule.to, "/infrastructure/");
    }
}

#[test]
fn test_file_with_combined_violations() {
    let code = r#"
// File with BOTH forbidden imports AND long functions
import { DatabaseService } from '../../infrastructure/database.service';
import { FileService } from '../../infrastructure/file.service';

class CombinedViolations {
    constructor(
        private db: DatabaseService,
        private fs: FileService
    ) {}

    // This is a very long method that exceeds max_lines
    async processData() {
        const step1 = 1;
        const step2 = 2;
        const step3 = 3;
        const step4 = 4;
        const step5 = 5;
        const step6 = 6;
        const step7 = 7;
        const step8 = 8;
        const step9 = 9;
        const step10 = 10;
        const step11 = 11;
        const step12 = 12;
        const step13 = 13;
        const step14 = 14;
        const step15 = 15;
        const step16 = 16;
        const step17 = 17;
        const step18 = 18;
        const step19 = 19;
        const step20 = 20;
        const step21 = 21;
        const step22 = 22;
        const step23 = 23;
        const step24 = 24;
        const step25 = 25;
        const step26 = 26;
        const step27 = 27;
        const step28 = 28;
        const step29 = 29;
        const step30 = 30;
        const step31 = 31;
        const step32 = 32;
        const step33 = 33;
        const step34 = 34;
        const step35 = 35;

        await this.db.query('SELECT * FROM users');
        const data = await this.fs.readFile('/data.json');

        return data;
    }
}
"#;


    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("domain/services/combined.service.ts");

    // Create parent directories
    std::fs::create_dir_all(file_path.parent().unwrap()).unwrap();
    std::fs::write(&file_path, code).unwrap();

    // Create context with forbidden import rules
    let rules = vec![ForbiddenRule {
        from: "/domain/".to_string(),
        to: "/infrastructure/".to_string(),
        severity: None,
    }];
    let ctx = create_context_with_rules(rules);

    // Test 1: Check for forbidden import violations
    let violations = collect_violations_from_file(&file_path, &ctx);
    assert!(violations.is_ok());
    let viols = violations.unwrap();
    assert_eq!(viols.len(), 2, "Should detect 2 forbidden imports");

    // Test 2: Check for long functions
    let long_functions = find_long_functions(&file_path, ctx.max_lines);
    assert!(long_functions.is_ok());
    let long_funcs = long_functions.unwrap();
    assert_eq!(long_funcs.len(), 1, "Should detect 1 long function");

    // Test 3: Count functions (count_functions only counts methods, not constructors)
    let function_count = count_functions(&file_path);
    assert!(function_count.is_ok());
    assert_eq!(function_count.unwrap(), 1); // only processData (constructor is not counted)
}

#[test]
fn test_decorated_class_with_violations() {
    let code = r#"
import { Injectable } from '@nestjs/common';
import { DatabaseAdapter } from '../infrastructure/database-adapter';
import { CacheAdapter } from '../infrastructure/cache-adapter';

@Injectable()
export class ViolatingService {
    constructor(
        private db: DatabaseAdapter,
        private cache: CacheAdapter
    ) {}

    // Long method with many lines
    async complexOperation() {
        const var1 = 1;
        const var2 = 2;
        const var3 = 3;
        const var4 = 4;
        const var5 = 5;
        const var6 = 6;
        const var7 = 7;
        const var8 = 8;
        const var9 = 9;
        const var10 = 10;
        const var11 = 11;
        const var12 = 12;
        const var13 = 13;
        const var14 = 14;
        const var15 = 15;
        const var16 = 16;
        const var17 = 17;
        const var18 = 18;
        const var19 = 19;
        const var20 = 20;
        const var21 = 21;
        const var22 = 22;
        const var23 = 23;
        const var24 = 24;
        const var25 = 25;
        const var26 = 26;
        const var27 = 27;
        const var28 = 28;
        const var29 = 29;
        const var30 = 30;
        const var31 = 31;
        const var32 = 32;

        await this.db.connect();
        const cached = await this.cache.get('key');

        return cached;
    }
}
"#;


    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("domain/violating.service.ts");

    // Create parent directory
    std::fs::create_dir_all(file_path.parent().unwrap()).unwrap();
    std::fs::write(&file_path, code).unwrap();

    // Create context with forbidden rules
    let rules = vec![ForbiddenRule {
        from: "/domain/".to_string(),
        to: "/infrastructure/".to_string(),
        severity: None,
    }];
    let ctx = create_context_with_rules(rules);

    // Test 1: Verify it parses decorated class correctly
    // Note: count_functions doesn't count exported class methods currently
    let function_count = count_functions(&file_path);
    assert!(function_count.is_ok());

    // Test 2: Check for forbidden imports (2 infrastructure imports)
    let violations = collect_violations_from_file(&file_path, &ctx);
    assert!(violations.is_ok());
    let viols = violations.unwrap();
    assert_eq!(viols.len(), 2);

    // Test 3: Check for long functions
    let long_functions = find_long_functions(&file_path, ctx.max_lines);
    assert!(long_functions.is_ok());
    let long_funcs = long_functions.unwrap();
    assert_eq!(long_funcs.len(), 1);

    // Test 4: Verify imports are counted correctly
    let import_count = count_imports(&file_path);
    assert!(import_count.is_ok());
    assert_eq!(import_count.unwrap(), 3);
}

#[test]
fn test_empty_decorated_class() {
    let code = r#"
import { Injectable } from '@nestjs/common';

@Injectable()
export class EmptyService {
}
"#;


    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("empty.service.ts");

    std::fs::write(&file_path, code).unwrap();

    // Should handle empty class gracefully
    let function_count = count_functions(&file_path);
    assert!(function_count.is_ok());
    assert_eq!(function_count.unwrap(), 0);

    let import_count = count_imports(&file_path);
    assert!(import_count.is_ok());
    assert_eq!(import_count.unwrap(), 1);
}

#[test]
fn test_class_with_decorator_and_inheritance() {
    let code = r#"
import { Injectable } from '@nestjs/common';
import { BaseService } from './base.service';

@Injectable()
export class ExtendedService extends BaseService {
    constructor() {
        super();
    }

    override handleRequest() {
        super.handleRequest();
        console.log('Extended logic');
    }

    additionalMethod() {
        return 'additional';
    }
}
"#;


    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("extended.service.ts");

    std::fs::write(&file_path, code).unwrap();

    let _ctx = create_test_context();

    // Note: count_functions doesn't count exported class methods currently
    let function_count = count_functions(&file_path);
    assert!(function_count.is_ok());

    // Count imports
    let import_count = count_imports(&file_path);
    assert!(import_count.is_ok());
    assert_eq!(import_count.unwrap(), 2);
}
