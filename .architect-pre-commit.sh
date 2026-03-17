#!/bin/bash

# Pre-commit hook for architect-linter-pro

echo "Running architecture lint..."

# Execute the linter and capture both output and exit code
OUTPUT=$(architect lint . --severity error 2>&1)
EXIT_CODE=$?

# Extract the project directory name correctly
PROJECT_DIR=$(basename "$PWD")

# Escape logic for bash to safely send JSON payload without formatting issues
escape_json() {
  local str="$1"
  str="${str//\\/\\\\}"
  str="${str//\"/\\\"}"
  str="${str//$'\n'/\\n}"
  str="${str//$'\r'/}"
  str="${str//$'\t'/\\t}"
  echo "$str"
}

SAFE_OUTPUT=$(escape_json "$OUTPUT")

if [ $EXIT_CODE -ne 0 ]; then
    echo "Architecture violations found. Fix them before committing."
    echo "$OUTPUT"
    
    curl -s -X POST -H "Content-Type: application/json" \
         -d "{\"success\": false, \"output\": \"$SAFE_OUTPUT\", \"projectDir\": \"$PROJECT_DIR\"}" \
         http://localhost:3000/api/webhooks/linter-status > /dev/null
         
    exit 1
fi

echo "Architecture check passed"

curl -s -X POST -H "Content-Type: application/json" \
     -d "{\"success\": true, \"output\": \"Architecture check passed\", \"projectDir\": \"$PROJECT_DIR\"}" \
     http://localhost:3000/api/webhooks/linter-status > /dev/null

exit 0
