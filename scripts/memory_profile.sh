#!/bin/bash

# Memory profiling script for Architect Linter Pro
# Profiles memory usage of the linter on specified project directory

PROJECT_DIR="${1:-.}"
BINARY="./target/release/architect"

if [ ! -f "$BINARY" ]; then
    echo "Building release binary..."
    cargo build --release
fi

echo "Memory profiling on: $PROJECT_DIR"
echo "================================================"
echo ""

# Run with memory tracking
if command -v /usr/bin/time &> /dev/null; then
    echo "Using /usr/bin/time for memory profiling:"
    echo ""
    /usr/bin/time -v "$BINARY" lint "$PROJECT_DIR" 2>&1 | grep -E "Maximum resident|User time|System time|Elapsed|Command being"
else
    echo "Profiling tool not available. Running analyzer..."
    "$BINARY" lint "$PROJECT_DIR"
fi

echo ""
echo "================================================"
echo "For detailed profiling with valgrind:"
echo ""
echo "  valgrind --tool=massif --massif-out-file=massif.out ./target/release/architect lint $PROJECT_DIR"
echo "  ms_print massif.out | head -100"
echo ""
echo "For CPU profiling with perf:"
echo ""
echo "  perf record -g ./target/release/architect lint $PROJECT_DIR"
echo "  perf report"
echo ""
