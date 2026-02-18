//! Parsing performance benchmarks for Architect Linter Pro
//!
//! This module benchmarks the parsing performance for different file counts.
//! It establishes performance baselines for detecting regressions.
//!
//! Run with: cargo bench --bench parsing_bench

use architect_linter_pro::analyzer::swc_parser;
use architect_linter_pro::config::{ArchPattern, Framework, LinterContext};
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::path::{Path, PathBuf};
use swc_common::sync::Lrc;
use swc_common::SourceMap;
use tempfile::TempDir;

/// Sample TypeScript code for benchmarking
const TS_CODE: &str = r#"
export class Service {
    constructor(private id: number) {}

    getData(): Data {
        return new Data();
    }

    processData(input: string): number {
        let result = 0;
        for (let i = 0; i < input.length; i++) {
            result += input.charCodeAt(i);
        }
        return result;
    }
}

export class Data {
    constructor() {}

    validate(): boolean {
        return true;
    }

    transform(value: string): string {
        return value.trim().toLowerCase();
    }
}

export class Controller {
    constructor(private service: Service) {}

    handleRequest(input: string): number {
        const data = this.service.getData();
        return this.service.processData(input);
    }
}
"#;

/// Create TypeScript files for benchmarking
fn create_ts_files(n: usize, temp_dir: &Path) {
    for i in 0..n {
        let file_path = temp_dir.join(format!("test{}.ts", i));
        std::fs::write(&file_path, TS_CODE).unwrap();
    }
}

/// Create a test LinterContext for benchmarking
fn create_test_context() -> LinterContext {
    LinterContext {
        max_lines: 50,
        framework: Framework::Unknown,
        pattern: ArchPattern::MVC,
        forbidden_imports: vec![],
        ignored_paths: vec![],
        ai_configs: vec![],
        ..Default::default()
    }
}

/// Benchmark parsing 10 files
fn bench_parse_10_files(c: &mut Criterion) {
    let temp_dir = TempDir::new().unwrap();
    create_ts_files(10, temp_dir.path());
    let parser_context = create_test_context();
    let cm = Lrc::new(SourceMap::default());
    let files: Vec<PathBuf> = (0..10)
        .map(|i| temp_dir.path().join(format!("test{}.ts", i)))
        .collect();

    c.bench_function("parse_10_files", |b| {
        b.iter(|| {
            for file in &files {
                black_box(
                    swc_parser::analyze_file(&cm, file, &parser_context)
                        .unwrap_or_else(|e| panic!("Parse failed: {}", e)),
                );
            }
        });
    });
}

/// Benchmark parsing 100 files
fn bench_parse_100_files(c: &mut Criterion) {
    let temp_dir = TempDir::new().unwrap();
    create_ts_files(100, temp_dir.path());
    let parser_context = create_test_context();
    let cm = Lrc::new(SourceMap::default());
    let files: Vec<PathBuf> = (0..100)
        .map(|i| temp_dir.path().join(format!("test{}.ts", i)))
        .collect();

    c.bench_function("parse_100_files", |b| {
        b.iter(|| {
            for file in &files {
                black_box(
                    swc_parser::analyze_file(&cm, file, &parser_context)
                        .unwrap_or_else(|e| panic!("Parse failed: {}", e)),
                );
            }
        });
    });
}

/// Benchmark parsing with different file counts
fn bench_parse_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse_scaling");
    let file_counts = vec![10, 50, 100];

    for file_count in file_counts {
        let temp_dir = TempDir::new().unwrap();
        create_ts_files(file_count, temp_dir.path());
        let parser_context = create_test_context();
        let cm = Lrc::new(SourceMap::default());
        let files: Vec<PathBuf> = (0..file_count)
            .map(|i| temp_dir.path().join(format!("test{}.ts", i)))
            .collect();

        group.bench_with_input(
            BenchmarkId::from_parameter(file_count),
            &file_count,
            |b, &_file_count| {
                b.iter(|| {
                    for file in &files {
                        black_box(
                            swc_parser::analyze_file(&cm, file, &parser_context)
                                .unwrap_or_else(|e| panic!("Parse failed: {}", e)),
                        );
                    }
                });
            },
        );

        drop(temp_dir);
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_parse_10_files,
    bench_parse_100_files,
    bench_parse_scaling
);
criterion_main!(benches);
