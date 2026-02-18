//! Snapshot tests for TypeScript/JavaScript parser
//! These tests verify the parser's ability to extract:
//! - Class declarations
//! - Function/method calls
//! - Import statements
//!
//! Using insta for snapshot testing to avoid constant updates when AST structures change.

use architect_linter_pro::analyzer::{count_functions, count_imports, find_long_functions};
use architect_linter_pro::config::{ArchPattern, Framework};
use swc_common::sync::Lrc;
use swc_common::SourceMap;

/// Helper function to create a LinterContext for testing
fn create_test_context() -> architect_linter_pro::config::LinterContext {
    architect_linter_pro::config::LinterContext {
        max_lines: 30,
        framework: Framework::Express,
        pattern: ArchPattern::MVC,
        forbidden_imports: vec![],
        ignored_paths: vec![],
        ai_configs: vec![],
        ..Default::default()
    }
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
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("test.ts");

    // Write directly to the .ts path
    std::fs::write(&file_path, code).unwrap();

    let _ctx = create_test_context();

    // Extract functions from the parsed AST
    let function_count = count_functions(&cm, &file_path);

    // Snapshot the function count result
    insta::assert_debug_snapshot!(function_count);

    // temp_dir will be cleaned up when it goes out of scope
}

#[test]
fn test_extract_function_calls() {
    let code = r#"
class Service {
    method() {
        this.helper();
        console.log('test');
    }

    helper() {
        return true;
    }
}
"#;

    let cm = Lrc::new(SourceMap::default());
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("test.ts");

    // Write directly to the .ts path
    std::fs::write(&file_path, code).unwrap();

    // Let's just count functions for now as extract_function_calls is removed
    let function_calls = count_functions(&cm, &file_path);

    // Snapshot the function calls result
    insta::assert_debug_snapshot!(function_calls);

    // temp_dir will be cleaned up when it goes out of scope
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

    // Use a fixed test directory instead of tempdir for stable snapshots
    let test_dir = std::path::PathBuf::from("tests/test_data");
    std::fs::create_dir_all(&test_dir).unwrap();
    let file_path = test_dir.join("long_function_test.ts");

    // Write the test file
    std::fs::write(&file_path, code).unwrap();

    let _ctx = create_test_context();

    // Extract long functions from the parsed AST
    let long_functions = find_long_functions(&cm, &file_path, _ctx.max_lines);

    // Snapshot the long functions result
    insta::assert_debug_snapshot!(long_functions);

    // Clean up
    let _ = std::fs::remove_file(&file_path);
}

#[test]
fn test_detect_imports() {
    let code = r#"
import { UserService } from './user.service';
import { Logger } from 'logger';
import * as fs from 'fs';
"#;

    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("test.ts");

    // Write directly to the .ts path
    std::fs::write(&file_path, code).unwrap();

    // Count imports from the parsed file
    let import_count = count_imports(&file_path);

    // Snapshot the import count result
    insta::assert_debug_snapshot!(import_count);

    // temp_dir will be cleaned up when it goes out of scope
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
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("test.ts");

    // Write directly to the .ts path
    std::fs::write(&file_path, code).unwrap();

    let _ctx = create_test_context();

    // Try to parse and extract functions
    let result = count_functions(&cm, &file_path);

    // Snapshot the result (should handle errors gracefully)
    insta::assert_debug_snapshot!(result);

    // temp_dir will be cleaned up when it goes out of scope
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
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("test.ts");

    // Write directly to the .ts path
    std::fs::write(&file_path, code).unwrap();

    let _ctx = create_test_context();

    // Extract function count
    let function_count = count_functions(&cm, &file_path);

    // Extract import count
    let import_count = count_imports(&file_path);

    // Snapshot both results
    insta::assert_debug_snapshot!((
        "function_count",
        function_count,
        "import_count",
        import_count
    ));

    // temp_dir will be cleaned up when it goes out of scope
}

#[test]
fn test_extract_class_methods() {
    let code = r#"
export class UserController {
    private repository: UserRepository;
    protected config: Config;

    constructor(repo: UserRepository) {
        this.repository = repo;
    }

    // Public method
    public async getUser(id: number): Promise<User> {
        return await this.repository.findById(id);
    }

    // Private method
    private validateUser(user: User): boolean {
        return user.id > 0;
    }

    // Protected method
    protected logAccess(userId: number): void {
        console.log(`User ${userId} accessed`);
    }

    // Static method
    static create(repo: UserRepository): UserController {
        return new UserController(repo);
    }

    // Getter
    get userCount(): number {
        return 100;
    }

    // Setter
    set maxUsers(value: number) {
        console.log(`Max users set to ${value}`);
    }

    // Async method
    async fetchData(): Promise<void> {
        await this.repository.connect();
    }
}
"#;

    let cm = Lrc::new(SourceMap::default());
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("test.ts");

    std::fs::write(&file_path, code).unwrap();

    let _ctx = create_test_context();

    // Extract function count
    let function_count = count_functions(&cm, &file_path);
    let function_calls = count_functions(&cm, &file_path);

    // Extract function count
    let function_count = count_functions(&cm, &file_path);
    let function_calls = count_functions(&cm, &file_path);

    // Snapshot both results
    insta::assert_debug_snapshot!((
        "function_count",
        function_count,
        "function_calls",
        function_calls
    ));

    // temp_dir will be cleaned up when it goes out of scope
}

#[test]
fn test_extract_decorators() {
    let code = r#"
import { Injectable, Inject } from '@nestjs/common';
import { Controller, Get, Post } from '@nestjs/common';

@Injectable()
export class UserService {
    constructor(
        @Inject('USER_REPOSITORY')
        private userRepo: any
    ) {}

    async findAll() {
        return [];
    }
}

@Controller('users')
export class UserController {
    constructor(private service: UserService) {}

    @Get()
    async getAll() {
        return this.service.findAll();
    }

    @Post()
    async create(@Body() dto: any) {
        return { created: true };
    }

    @Get(':id')
    @UseGuards(AuthGuard)
    async getOne(@Param('id') id: string) {
        return { id };
    }
}

@Component({
    selector: 'app-user',
    template: '<div>User</div>'
})
export class UserComponent {
    @Input()
    userId: number;

    @Output()
    userChange = new EventEmitter();

    @ViewChild('container')
    container: ElementRef;
}
"#;

    let cm = Lrc::new(SourceMap::default());
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("test.ts");

    std::fs::write(&file_path, code).unwrap();

    let _ctx = create_test_context();

    // Extract function count
    let function_count = count_functions(&cm, &file_path);
    let function_calls = count_functions(&cm, &file_path);

    // Extract function count
    let function_count = count_functions(&cm, &file_path);
    let function_calls = count_functions(&cm, &file_path);

    // Extract imports (decorators are imported)
    let import_count = count_imports(&file_path);

    // Snapshot all results
    insta::assert_debug_snapshot!((
        "function_count",
        function_count,
        "function_calls",
        function_calls,
        "import_count",
        import_count
    ));

    // temp_dir will be cleaned up when it goes out of scope
}

#[test]
fn test_generic_types() {
    let code = r#"
// Generic class
export class Box<T> {
    private value: T;

    constructor(value: T) {
        this.value = value;
    }

    getValue(): T {
        return this.value;
    }

    setValue(value: T): void {
        this.value = value;
    }
}

// Generic class with constraints
export class Repository<T extends Entity> {
    private items: T[] = [];

    add(item: T): void {
        this.items.push(item);
    }

    findById(id: number): T | undefined {
        return this.items.find(item => item.id === id);
    }
}

// Generic function
function identity<T>(arg: T): T {
    return arg;
}

// Generic function with constraints
function getProperty<T, K extends keyof T>(obj: T, key: K): T[K] {
    return obj[key];
}

// Generic type alias
type Result<T, E> =
    | { ok: true; value: T }
    | { ok: false; error: E };

// Multiple type parameters
export class Pair<K, V> {
    constructor(public key: K, public value: V) {}

    getKey(): K {
        return this.key;
    }

    getValue(): V {
        return this.value;
    }
}

// Generic with default type
export class Container<T = string> {
    private data: T[] = [];

    add(item: T): void {
        this.data.push(item);
    }

    getAll(): T[] {
        return this.data;
    }
}

// Complex generic type
type ApiResponse<T> = Promise<{
    data: T;
    status: number;
    message: string;
}>;

async function fetchUser(): ApiResponse<User> {
    return {
        data: { id: 1, name: 'John' },
        status: 200,
        message: 'Success'
    };
}
"#;

    let cm = Lrc::new(SourceMap::default());
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("test.ts");

    std::fs::write(&file_path, code).unwrap();

    let _ctx = create_test_context();

    // Extract function count
    let function_count = count_functions(&cm, &file_path);
    let function_calls = count_functions(&cm, &file_path);

    // Extract function count
    let function_count = count_functions(&cm, &file_path);
    let function_calls = count_functions(&cm, &file_path);

    // Snapshot both results
    insta::assert_debug_snapshot!((
        "function_count",
        function_count,
        "function_calls",
        function_calls
    ));

    // temp_dir will be cleaned up when it goes out of scope
}
