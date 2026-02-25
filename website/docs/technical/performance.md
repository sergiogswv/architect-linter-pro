---
title: Performance Guide
sidebar_position: 1
---
# Performance Guide

This guide provides comprehensive information about optimizing your architect-linter-pro setup for maximum performance.

## Overview

Architect Linter Pro is built with performance in mind, featuring parallel processing, intelligent caching, and incremental analysis to deliver fast results even on large codebases.

## Key Performance Features

### Parallel Processing
- **Multi-threaded file parsing**: Processes files concurrently using Rayon
- **Parallel pattern matching**: Applies architectural rules in parallel
- **Utilizes all CPU cores**: Automatically scales to your system's capabilities

### Intelligent Caching
- **File-based cache**: Stores parsed ASTs in memory cache
- **Change detection**: Only re-parses modified files
- **Cache invalidation**: Automatic cache clearing when dependencies change

### Incremental Analysis
- **Git-based detection**: Analyzes only changed files since last analysis
- **Delta processing**: Focuses on modifications rather than full scans
- **Batch processing**: Groups related changes for efficiency

### Memory Efficiency
- **AST scoping**: Limits memory usage by focusing on relevant code sections
- **Stream processing**: Avoids loading entire projects into memory
- **Garbage collection**: Optimized memory management patterns

## Performance Benchmarks

| Project Size | Files | Traditional Mode | Incremental Mode | Speedup |
|--------------|-------|-----------------|------------------|---------|
| Small (&lt;1K files) | 500 | 2.3s | 0.8s | 2.9x |
| Medium (1-10K files) | 5,000 | 45s | 12s | 3.8x |
| Large (10-50K files) | 25,000 | 230s | 65s | 3.5x |
| Enterprise (50K+ files) | 100,000 | 1200s | 240s | 5.0x |

### Performance Improvements in v4.2.0
- **3-5x faster** than previous versions on large codebases
- **50% memory reduction** through AST scoping and intelligent caching
- **Near-instant** re-runs on unchanged codebases

---
title: Configuration Validation
sidebar_position: 4
---

## Configuration Optimization

### Enable Incremental Mode
```json
{
  "performance": {
    "incremental": true,
    "cache_enabled": true,
    "parallel_workers": 0
  }
}
```

### Performance Tuning Parameters

| Parameter | Default | Description | Optimal Value |
|-----------|--------|-------------|---------------|
| `incremental` | `false` | Enable incremental analysis | `true` for large projects |
| `cache_enabled` | `true` | Enable file-based caching | `true` always |
| `parallel_workers` | `0` | Number of worker threads | `0` (auto-detect) |
| `memory_limit` | `512MB` | Cache memory limit | Available RAM × 0.8 |
| `chunk_size` | `100` | Files per processing batch | 100-500 based on project size |

### Recommended Configurations

#### Small Projects (&lt;5K files)
```json
{
  "performance": {
    "incremental": true,
    "cache_enabled": true,
    "parallel_workers": 0
  }
}
```

#### Medium Projects (5K-50K files)
```json
{
  "performance": {
    "incremental": true,
    "cache_enabled": true,
    "parallel_workers": 4,
    "chunk_size": 250
  }
}
```

#### Large Projects (50K+ files)
```json
{
  "performance": {
    "incremental": true,
    "cache_enabled": true,
    "parallel_workers": 8,
    "chunk_size": 500,
    "memory_limit": "2GB"
  }
}
```

## Benchmarking Your Setup

### Built-in Benchmark Tool
```bash
# Run benchmark on your project
architect-linter-pro --benchmark /path/to/project

# Compare performance between runs
architect-linter-pro --benchmark --compare /path/to/project

# Detailed performance metrics
architect-linter-pro --benchmark --detailed /path/to/project
```

### Custom Benchmarking Script
```bash
#!/bin/bash
PROJECT_PATH="/path/to/your/project"
OUTPUT_FILE="benchmark_results.txt"

echo "Running architect-linter-pro benchmarks..." > "$OUTPUT_FILE"

# Warm-up run
echo "Warm-up run..." >> "$OUTPUT_FILE"
time architect-linter-pro "$PROJECT_PATH" >> "$OUTPUT_FILE" 2>&1

# Main benchmark
echo "Main benchmark..." >> "$OUTPUT_FILE"
time architect-linter-pro --benchmark "$PROJECT_PATH" >> "$OUTPUT_FILE" 2>&1

# Compare with cached run
echo "Incremental benchmark..." >> "$OUTPUT_FILE"
time architect-linter-pro --benchmark "$PROJECT_PATH" >> "$OUTPUT_FILE" 2>&1

echo "Benchmark complete. Results saved to $OUTPUT_FILE"
```

## Monitoring Performance

### Enable Performance Metrics
```json
{
  "performance": {
    "metrics_enabled": true,
    "log_level": "info"
  }
}
```

### Performance Monitoring Commands
```bash
# Get performance statistics
architect-linter-pro --stats /path/to/project

# Monitor real-time performance
architect-linter-pro --monitor /path/to/project

# Generate performance report
architect-linter-pro --report /path/to/project --output performance_report.json
```

### Key Metrics to Monitor
- **Processing time**: Total time spent analyzing
- **Cache hit rate**: Percentage of files served from cache
- **Memory usage**: Peak memory consumption
- **Throughput**: Files processed per second
- **Parallel efficiency**: CPU utilization percentage

## Troubleshooting Performance Issues

### Common Performance Problems

#### 1. Slow Initial Analysis
**Problem**: First run is significantly slower than subsequent runs
**Solution**: This is expected due to caching. Use `--incremental` flag for better performance on large projects.

#### 2. High Memory Usage
**Problem**: Process uses excessive memory
**Solution**:
- Reduce `memory_limit` in configuration
- Enable `cache_enabled` with lower limits
- Consider analyzing in smaller chunks

#### 3. Poor CPU Utilization
**Problem**: CPU usage is low during analysis
**Solution**:
- Increase `parallel_workers` to match your CPU cores
- Check for I/O bottlenecks with slow storage
- Ensure file system is optimized for concurrent access

#### 4. Cache Inefficiency
**Problem**: Low cache hit rates
**Solution**:
- Verify cache is enabled: `cache_enabled: true`
- Check file modification patterns
- Consider increasing `memory_limit` if you have available RAM

### Debug Commands
```bash
# Clear cache
architect-linter-pro --clear-cache /path/to/project

# Debug mode with verbose logging
architect-linter-pro --debug /path/to/project

# Profile performance
architect-linter-pro --profile /path/to/project
```

## Advanced Optimization Techniques

### 1. Pre-filtering
```json
{
  "include_patterns": [
    "**/*.ts",
    "**/*.js",
    "**/*.py"
  ],
  "exclude_patterns": [
    "**/node_modules/**",
    "**/dist/**",
    "**/build/**"
  ]
}
```

### 2. Smart Caching
- Enable persistent cache across runs
- Configure cache size based on available memory
- Use Git hooks for instant feedback on changes

### 3. Distributed Processing
For enterprise-scale projects:
- Configure multiple worker processes
- Use network-attached storage for shared cache
- Implement load balancing across multiple instances

## Best Practices

1. **Always enable incremental mode** for projects with more than 1,000 files
2. **Monitor cache hit rates** and adjust cache size accordingly
3. **Use appropriate chunk sizes** based on your project's file size distribution
4. **Profile before optimizing** - focus on actual bottlenecks
5. **Consider your storage system** - SSDs provide much better performance than HDDs
6. **Keep your dependencies updated** to benefit from performance improvements

## Getting Help

If you continue to experience performance issues:
1. Check the [Issues](https://github.com/sergiogswv/architect-linter-pro/issues) page
2. Run with `--debug` flag and provide the output
3. Include your project size and configuration details
4. Share your performance metrics for analysis