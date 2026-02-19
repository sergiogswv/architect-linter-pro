//! File analysis collector with caching support

use crate::analysis_result::{AnalysisResult, CategorizedViolation, ViolationCategory};
use crate::cache::{self, AnalysisCache, FileCacheEntry};
use crate::config::{ArchPattern, LinterContext};
use crate::metrics::ComplexityStats;
use indicatif::{ProgressBar, ProgressStyle};
use miette::Result;
use rayon::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use swc_common::sync::Lrc;
use swc_common::SourceMap;

use super::metrics::{count_functions, count_imports, find_long_functions};
use super::swc_parser::collect_violations_from_file;

/// Result of analyzing a single file
struct FileAnalysis {
    violations: Vec<CategorizedViolation>,
    long_functions: Vec<crate::analysis_result::LongFunction>,
    import_count: usize,
    function_count: usize,
}

/// Analyzes all files and returns a complete AnalysisResult for scoring.
/// When a cache is provided, unchanged files are served from cache.
pub fn analyze_all_files(
    files: &[PathBuf],
    project_root: &Path,
    pattern: ArchPattern,
    ctx: &LinterContext,
    _cm: &SourceMap,
    mut analysis_cache: Option<&mut AnalysisCache>,
) -> Result<AnalysisResult> {
    tracing::info!("Starting file analysis for {} files", files.len());
    tracing::debug!("Project root: {}", project_root.display());

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

    // Initialize progress bar
    let pb = if files.len() > 10 {
        let pb = ProgressBar::new(files.len() as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} files ({eta})")
                .unwrap()
                .progress_chars("#>-"),
        );
        Some(pb)
    } else {
        None
    };

    // Prepare cache for thread-safe access
    // We clone the cache if it exists to use in parallel, then update it back
    let cache_enabled = analysis_cache.is_some();
    if cache_enabled {
        tracing::debug!("Analysis cache enabled");
    }

    let cache_mutex: Option<Mutex<AnalysisCache>> = analysis_cache
        .as_mut()
        .map(|cache| Mutex::new((**cache).clone()));

    // Process files in parallel
    let file_results: Vec<(PathBuf, Option<String>, FileAnalysis)> = files
        .par_iter()
        .filter_map(|file_path| {
            // Update progress bar
            if let Some(ref p) = pb {
                p.inc(1);
            }

            // Read file content for hashing
            let file_bytes = match fs::read(file_path) {
                Ok(b) => b,
                Err(_) => return None,
            };
            let content_hash = cache::hash_content(&file_bytes);
            let cache_key = AnalysisCache::normalize_path(file_path, project_root);

            // Try to get from cache (thread-safe)
            if let Some(ref mutex) = cache_mutex {
                if let Ok(guard) = mutex.lock() {
                    if let Some(entry) = guard.get(&cache_key, &content_hash) {
                        // Cache hit — use stored data
                        return Some((
                            file_path.clone(),
                            Some(cache_key),
                            FileAnalysis {
                                violations: entry.violations.clone(),
                                long_functions: entry.long_functions.clone(),
                                import_count: entry.import_count,
                                function_count: entry.function_count,
                            },
                        ));
                    }
                }
            }

            // Cache miss — run full analysis with thread-local SourceMap
            let cm = Lrc::new(SourceMap::default());

            let mut file_violations = Vec::new();
            if let Ok(violations) = collect_violations_from_file(&cm, file_path, ctx) {
                for violation in violations {
                    let category = match violation.rule.get_severity() {
                        crate::config::Severity::Error => ViolationCategory::Blocked,
                        crate::config::Severity::Warning => ViolationCategory::Warning,
                        crate::config::Severity::Info => ViolationCategory::Info,
                    };
                    let categorized = CategorizedViolation::new(violation, category);
                    file_violations.push(categorized);
                }
            }

            let mut file_long_functions = Vec::new();
            if let Ok(long_funcs) = find_long_functions(&cm, file_path, ctx.max_lines) {
                file_long_functions = long_funcs;
            }

            let import_count = count_imports(file_path).unwrap_or(0);
            let function_count = count_functions(&cm, file_path).unwrap_or(0);

            let analysis = FileAnalysis {
                violations: file_violations.clone(),
                long_functions: file_long_functions.clone(),
                import_count,
                function_count,
            };

            // Store in cache (thread-safe)
            if let Some(ref mutex) = cache_mutex {
                if let Ok(mut guard) = mutex.lock() {
                    guard.insert(
                        cache_key.clone(),
                        FileCacheEntry {
                            content_hash,
                            violations: file_violations,
                            long_functions: file_long_functions,
                            import_count,
                            function_count,
                        },
                    );
                }
            }

            Some((file_path.clone(), None, analysis))
        })
        .collect();

    // Finish progress bar
    if let Some(ref p) = pb {
        p.finish_with_message("Analysis complete");
    }

    tracing::info!(
        "File analysis complete. Processed {} files",
        file_results.len()
    );

    // Update the original cache from the mutex
    if let Some(mutex) = cache_mutex {
        if let Ok(guard) = mutex.lock() {
            if let Some(ref mut analysis_cache) = analysis_cache {
                **analysis_cache = (*guard).clone();
            }
        }
    }

    // Merge all file results into the main result
    for (_file_path, _cache_key, file_analysis) in file_results {
        for cv in file_analysis.violations {
            result.add_violation(cv);
        }
        for func in file_analysis.long_functions {
            result.add_long_function(func);
        }
        result.layer_stats.total_imports += file_analysis.import_count;
        result.complexity_stats.total_functions += file_analysis.function_count;
    }

    // Update blocked_violations count
    result.layer_stats.blocked_violations = result.blocked_count();

    Ok(result)
}
