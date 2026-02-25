#!/bin/bash

echo "Running code coverage analysis..."
echo "=================================="
echo ""

# Build project first
cargo build --lib 2>&1 | tail -3

# Run coverage with tarpaulin
echo "Generating coverage report..."
cargo tarpaulin --out Html --output-dir coverage_report --timeout 300 2>&1 | tail -10

echo ""
echo "✅ Coverage report generated!"
echo "📊 View report: open coverage_report/index.html"
