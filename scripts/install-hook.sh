#!/bin/bash

HOOK_PATH=".git/hooks/pre-commit"
SCRIPT_PATH=".architect-pre-commit.sh"

if [ ! -d ".git" ]; then
    echo "Error: Not in a git repository"
    exit 1
fi

# Copy hook and make executable
cp "$SCRIPT_PATH" "$HOOK_PATH"
chmod +x "$HOOK_PATH"

echo "Pre-commit hook installed"
echo "   Hooks will run: architect lint . --severity error"
