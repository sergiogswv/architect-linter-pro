#!/bin/bash

HOOK_PATH=".git/hooks/pre-commit"
SCRIPT_PATH=".architect-pre-commit.sh"

if [ ! -d ".git" ]; then
    echo "Error: Not in a git repository"
    exit 1
fi

# Check if source script exists
if [ ! -f "$SCRIPT_PATH" ]; then
    echo "Error: $SCRIPT_PATH not found. Make sure you're in the project root directory."
    exit 1
fi

# Copy hook and make executable
cp "$SCRIPT_PATH" "$HOOK_PATH" || {
    echo "Error: Failed to copy $SCRIPT_PATH to $HOOK_PATH"
    echo "Make sure you have write permissions to .git/hooks/"
    exit 1
}

chmod +x "$HOOK_PATH" || {
    echo "Error: Failed to make hook executable"
    exit 1
}

echo "Pre-commit hook installed"
echo "   Hooks will run: architect lint . --severity error"
