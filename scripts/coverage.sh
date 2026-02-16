#!/bin/bash
# Coverage report generation script

set -e

echo "Running coverage report..."
cargo tarpaulin \
    --verbose \
    --all-features \
    --timeout 300 \
    --out xml

echo ""
echo "Coverage by module:"
echo "  - Run: cargo tarpaulin --exclude-files -- --test"
echo "  - Report: cobertura.xml generated"
echo ""
echo "To upload to Codecov (if configured):"
echo "  - curl -Os https://codecov.io/bash -s -R -f cobertura.xml"
