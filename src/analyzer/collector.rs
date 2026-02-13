//! File analysis collector with caching support

use crate::analysis_result::{
    AnalysisResult, CategorizedViolation, ViolationCategory,
};
use crate::cache::{self, AnalysisCache, FileCacheEntry};
use crate::config::{ArchPattern, LinterContext};
use crate::metrics::{ComplexityStats, LayerStats};
use miette::Result;
use std::fs;
use std::path::{Path, PathBuf};
use swc_common::SourceMap;

use super::metrics::{count_functions, count_imports, find_long_functions};
use super::swc_parser::collect_violations_from_file;

/// Analyzes all files and returns a complete AnalysisResult for scoring.
/// When a cache is provided, unchanged files are served from cache.
pub fn analyze_all_files(
    files: &[PathBuf],
    project_root: &Path,
    pattern: ArchPattern,
    ctx: &LinterContext,
    cm: &SourceMap,
    mut analysis_cache: Option<&mut AnalysisCache>,
) -> Result<AnalysisResult> {
    // Get project name from directory
    let project_name = project_root
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("project")
        .to_string();

    let mut result = AnalysisResult::new(project_name, pattern);
    result.files_analyzed = files.len();

    // Initialize complexity stats with threshold
    result.complexity_stats = ComplexityStats {
        total_functions: 0,
        long_functions: 0,
        max_lines_threshold: ctx.max_lines,
    };

    let mut total_imports = 0usize;

    for file_path in files {
        // Read file content for hashing
        let file_bytes = match fs::read(file_path) {
            Ok(b) => b,
            Err(_) => continue,
        };
        let content_hash = cache::hash_content(&file_bytes);
        let cache_key = AnalysisCache::normalize_path(file_path, project_root);

        // Check cache
        if let Some(ref analysis_cache) = analysis_cache {
            if let Some(entry) = analysis_cache.get(&cache_key, &content_hash) {
                // Cache hit — use stored data
                for cv in &entry.violations {
                    result.add_violation(cv.clone());
                }
                for func in &entry.long_functions {
                    result.add_long_function(func.clone());
                }
                total_imports += entry.import_count;
                result.complexity_stats.total_functions += entry.function_count;
                continue;
            }
        }

        // Cache miss — run full analysis for this file
        let mut file_violations = Vec::new();
        if let Ok(violations) = collect_violations_from_file(cm, file_path, ctx) {
            for violation in violations {
                let categorized = CategorizedViolation::new(violation, ViolationCategory::Blocked);
                file_violations.push(categorized);
            }
        }

        let mut file_long_functions = Vec::new();
        if let Ok(long_funcs) = find_long_functions(cm, file_path, ctx.max_lines) {
            file_long_functions = long_funcs;
        }

        let import_count = count_imports(file_path).unwrap_or(0);
        let function_count = count_functions(cm, file_path).unwrap_or(0);

        // Store in cache
        if let Some(ref mut analysis_cache) = analysis_cache {
            analysis_cache.insert(
                cache_key,
                FileCacheEntry {
                    content_hash,
                    violations: file_violations.clone(),
                    long_functions: file_long_functions.clone(),
                    import_count,
                    function_count,
                },
            );
        }

        // Accumulate into result
        for cv in file_violations {
            result.add_violation(cv);
        }
        for func in file_long_functions {
            result.add_long_function(func);
        }
        total_imports += import_count;
        result.complexity_stats.total_functions += function_count;
    }

    // Set layer stats
    result.layer_stats = LayerStats {
        total_imports,
        blocked_violations: result.blocked_count(),
    };

    Ok(result)
}
