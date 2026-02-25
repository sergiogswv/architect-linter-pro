#!/bin/bash

# Pre-commit hook for architect-linter-pro

echo "Running architecture lint..."

if ! architect lint . --severity error; then
    echo "Architecture violations found. Fix them before committing."
    exit 1
fi

echo "Architecture check passed"
exit 0
