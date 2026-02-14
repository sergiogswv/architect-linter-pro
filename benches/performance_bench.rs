//! Performance benchmarks for Architect Linter Pro
//!
//! This module benchmarks key performance characteristics:
//! - Parallel vs sequential file analysis
//! - Cache hit/miss performance
//!
//! Run with: cargo bench

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use std::path::{Path, PathBuf};
use tempfile::TempDir;
use std::fs;
use std::sync::Arc;
use std::time::Instant;

// Import our modules for benchmarking
use architect_linter_pro::analyzer::collector::analyze_all_files;
use architect_linter_pro::cache::{AnalysisCache, FileCacheEntry};
use architect_linter_pro::config::{ArchPattern, LinterContext, ForbiddenRule, Framework, AIConfig};
use architect_linter_pro::memory_cache::MemoryCache;
use architect_linter_pro::discovery;
use swc_common::sync::Lrc;
use swc_common::SourceMap;

/// Create a temporary directory with test TypeScript/JavaScript files
fn create_test_files(count: usize, avg_lines: usize) -> TempDir {
    let temp_dir = TempDir::new().unwrap();
    let _files = (0..count).map(|i| {
        let file_path = temp_dir.path().join(format!("file_{}.ts", i));
        let content = generate_ts_file(avg_lines, i);
        fs::write(&file_path, content).unwrap();
        file_path
    }).collect::<Vec<PathBuf>>();

    temp_dir
}

/// Generate a TypeScript file with specified number of lines
fn generate_ts_file(lines: usize, file_index: usize) -> String {
    let mut content = String::new();
    content.push_str(&format!("// File {} with {} lines\n", file_index, lines));

    if lines > 10 {
        content.push_str("import { Component, Injectable } from '@angular/core';\n");
        content.push_str("import { UserService } from './user.service';\n\n");
    }

    for i in 0..(lines / 5) {
        content.push_str(&format!("export class Component{} {{\n", i));
        content.push_str("  constructor(private service: UserService) {}\n");
        content.push_str("\n  public method1() {\n");
        content.push_str("    return this.service.getData();\n");
        content.push_str("  }\n\n");
        content.push_str("  public method2() {\n");
        content.push_str("    const data = [];\n");
        for j in 0..10 {
            content.push_str(&format!("    data.push('item{}');\n", j));
        }
        content.push_str("    return data;\n");
        content.push_str("  }\n");
        content.push_str("}\n\n");
    }

    content
}

/// Setup function for benchmarks
fn setup_linter_context() -> LinterContext {
    LinterContext {
        max_lines: 50,
        framework: Framework::Unknown,
        pattern: ArchPattern::Clean,
        forbidden_imports: vec![
            ForbiddenRule {
                from: "*".to_string(),
                to: "app/components".to_string(),
            }
        ],
        ignored_paths: vec![],
        ai_configs: vec![],
    }
}

/// Benchmark parallel vs sequential file analysis
fn bench_sequential_vs_parallel(c: &mut Criterion) {
    let file_counts = vec![10, 50, 100];
    let ctx = setup_linter_context();

    for file_count in file_counts {
        let temp_dir = create_test_files(file_count, 20);
        let files: Vec<PathBuf> = (0..file_count)
            .map(|i| temp_dir.path().join(format!("file_{}.ts", i)))
            .collect();

        // Parallel analysis (current implementation)
        let mut par_group = c.benchmark_group("parallel_analysis");

        par_group.bench_with_input(
            BenchmarkId::new("parallel", file_count),
            &(&files, &temp_dir.path(), &ctx),
            |b, &(ref files, ref project_root, ref ctx)| {
                let cm = Lrc::new(SourceMap::default());
                b.iter(|| {
                    black_box(analyze_all_files(
                        files,
                        project_root,
                        ctx.pattern.clone(),
                        ctx,
                        &cm,
                        Some(&mut AnalysisCache::new("test_config_hash".to_string())),
                    )).unwrap();
                });
            },
        );
        par_group.finish();

        // Drop temp directory after benchmark
        drop(temp_dir);
    }
}

/// Benchmark cache performance - hit vs miss
fn bench_cache_performance(c: &mut Criterion) {
    let temp_dir = create_test_files(50, 20);
    let files: Vec<PathBuf> = (0..50)
        .map(|i| temp_dir.path().join(format!("file_{}.ts", i)))
        .collect();

    let ctx = setup_linter_context();
    let project_root = temp_dir.path();

    // Create and populate cache
    let cache_key = "test".to_string();
    let mut cache = AnalysisCache::new("test_config_hash".to_string());

    // Pre-populate cache with some entries
    for i in 0..25 {
        let file_path = project_root.join(format!("file_{}.ts", i));
        let content = fs::read(&file_path).unwrap();
        let content_hash = architect_linter_pro::cache::hash_content(&content);

        cache.insert(
            architect_linter_pro::cache::AnalysisCache::normalize_path(&file_path, project_root),
            FileCacheEntry {
                content_hash,
                violations: vec![],
                long_functions: vec![],
                import_count: 5,
                function_count: 10,
            },
        );
    }

    // Benchmark cache hits
    let mut cache_hit_group = c.benchmark_group("cache_hits");

    cache_hit_group.bench_function("cache_hit", |b| {
        b.iter(|| {
            // Hit the cache for existing files
            for i in 0..25 {
                let file_path = project_root.join(format!("file_{}.ts", i));
                let normalized_key = architect_linter_pro::cache::AnalysisCache::normalize_path(&file_path, project_root);
                let content_hash = format!("{:016x}", i as u64); // Simulated content hash

                black_box(cache.get(&normalized_key, &content_hash));
            }
        });
    });
    cache_hit_group.finish();

    // Benchmark cache misses
    let mut cache_miss_group = c.benchmark_group("cache_misses");

    cache_miss_group.bench_function("cache_miss", |b| {
        b.iter(|| {
            // Try to get non-existent files
            for i in 25..50 {
                let file_path = project_root.join(format!("file_{}.ts", i));
                let normalized_key = architect_linter_pro::cache::AnalysisCache::normalize_path(&file_path, project_root);
                let content_hash = format!("{:016x}", i as u64);

                black_box(cache.get(&normalized_key, &content_hash));
            }
        });
    });
    cache_miss_group.finish();

    // Benchmark memory cache performance
    let mut mem_cache_group = c.benchmark_group("memory_cache");

    let memory_cache = Arc::new(MemoryCache::new(100));

    // Populate memory cache
    for i in 0..50 {
        let file_path = project_root.join(format!("file_{}.ts", i));
        memory_cache.put(file_path, format!("hash_{}", i));
    }

    mem_cache_group.bench_function("memory_cache_get", |b| {
        b.iter(|| {
            for i in 0..50 {
                let file_path = project_root.join(format!("file_{}.ts", i));
                black_box(memory_cache.get(&file_path));
            }
        });
    });
    mem_cache_group.finish();

    // Drop temp directory
    drop(temp_dir);
}

/// Benchmark memory usage with different file counts
fn bench_memory_usage(c: &mut Criterion) {
    let mut memory_group = c.benchmark_group("memory_usage");

    let file_counts = vec![10, 25, 50];

    for file_count in file_counts {
        memory_group.bench_with_input(
            BenchmarkId::new("analyze_files", file_count),
            &file_count,
            |b, &file_count| {
                let temp_dir = create_test_files(file_count, 20);
                let files: Vec<PathBuf> = (0..file_count)
                    .map(|i| temp_dir.path().join(format!("file_{}.ts", i)))
                    .collect();

                let ctx = setup_linter_context();
                let project_root = temp_dir.path();

                b.iter(|| {
                    let start = Instant::now();
                    let result = analyze_all_files(
                        &files,
                        project_root,
                        ctx.pattern.clone(),
                        &ctx,
                        &Lrc::new(SourceMap::default()),
                        Some(&mut AnalysisCache::new("test_config_hash".to_string())),
                    ).unwrap();
                    let duration = start.elapsed();

                    // Include result size in benchmark to prevent optimization
                    black_box(result);
                    black_box(duration);
                });

                drop(temp_dir);
            },
        );
    }

    memory_group.finish();
}

criterion_group!(
    benches,
    bench_sequential_vs_parallel,
    bench_cache_performance,
    bench_memory_usage
);
criterion_main!(benches);