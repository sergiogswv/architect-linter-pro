#!/bin/bash
# Architect Linter GitHub Action Entrypoint
# This script runs architect-linter and outputs results to GitHub Actions

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Get the path argument (default to current directory)
PROJECT_PATH="${1:-.}"

echo -e "${BLUE}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║           ARCHITECT LINTER - GitHub Action                 ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════════╝${NC}"
echo ""

# Change to project directory
cd "$PROJECT_PATH" 2>/dev/null || {
    echo -e "${RED}Error: Directory '$PROJECT_PATH' not found${NC}"
    exit 1
}

echo -e "${BLUE}📁 Analyzing: $(pwd)${NC}"
echo ""

# Build command arguments
ARGS=""

# Check for staged-only mode
if [ "${STAGED_ONLY}" = "true" ]; then
    echo -e "${YELLOW}🔍 Staged-only mode enabled${NC}"
    ARGS="$ARGS --staged"
fi

# Check for report format
if [ -n "${REPORT_FORMAT}" ]; then
    echo -e "${YELLOW}📊 Report format: ${REPORT_FORMAT}${NC}"
    ARGS="$ARGS --report ${REPORT_FORMAT}"

    # Set output path
    if [ -n "${REPORT_OUTPUT}" ]; then
        ARGS="$ARGS --output ${REPORT_OUTPUT}"
    else
        ARGS="$ARGS --output architect-report.${REPORT_FORMAT}"
    fi
fi

# Run architect-linter-pro
echo ""
echo -e "${BLUE}Running architect-linter-pro...${NC}"

# Capture output and exit code
set +e
OUTPUT=$(architect-linter-pro . $ARGS 2>&1)
EXIT_CODE=$?
set -e

# Print output
echo "$OUTPUT"

# Extract score from output (if available)
SCORE=$(echo "$OUTPUT" | grep -oP 'Score: \K\d+' || echo "")
GRADE=$(echo "$OUTPUT" | grep -oP 'Grade: \K[A-F]' || echo "")
VIOLATIONS=$(echo "$OUTPUT" | grep -oP '(\d+) violation' | head -1 | grep -oP '\d+' || echo "0")
CIRCULAR=$(echo "$OUTPUT" | grep -oP '(\d+) circular' | head -1 | grep -oP '\d+' || echo "0")

# Output to GitHub Actions
echo ""
echo -e "${BLUE}Setting GitHub Actions outputs...${NC}"

if [ -n "$SCORE" ]; then
    echo "score=$SCORE" >> $GITHUB_OUTPUT
    echo -e "  Score: ${GREEN}$SCORE${NC}"
fi

if [ -n "$GRADE" ]; then
    echo "grade=$GRADE" >> $GITHUB_OUTPUT
    echo -e "  Grade: ${GREEN}$GRADE${NC}"
fi

echo "violations=$VIOLATIONS" >> $GITHUB_OUTPUT
echo "circular-deps=$CIRCULAR" >> $GITHUB_OUTPUT

if [ -n "${REPORT_OUTPUT}" ] && [ -f "${REPORT_OUTPUT}" ]; then
    echo "report-path=$(pwd)/${REPORT_OUTPUT}" >> $GITHUB_OUTPUT
    echo -e "  Report: ${GREEN}$(pwd)/${REPORT_OUTPUT}${NC}"
elif [ -f "architect-report.json" ]; then
    echo "report-path=$(pwd)/architect-report.json" >> $GITHUB_OUTPUT
    echo -e "  Report: ${GREEN}$(pwd)/architect-report.json${NC}"
elif [ -f "architect-report.md" ]; then
    echo "report-path=$(pwd)/architect-report.md" >> $GITHUB_OUTPUT
    echo -e "  Report: ${GREEN}$(pwd)/architect-report.md${NC}"
fi

# Check minimum score
if [ -n "${MIN_SCORE}" ] && [ -n "$SCORE" ]; then
    if [ "$SCORE" -lt "${MIN_SCORE}" ]; then
        echo ""
        echo -e "${RED}❌ Score $SCORE is below minimum required (${MIN_SCORE})${NC}"
        EXIT_CODE=1
    fi
fi

# Handle failure
if [ $EXIT_CODE -ne 0 ]; then
    if [ "${FAIL_ON_VIOLATIONS}" = "true" ]; then
        echo ""
        echo -e "${RED}❌ Architect Linter found issues${NC}"
        echo ""
        exit 1
    else
        echo ""
        echo -e "${YELLOW}⚠️ Issues found, but fail-on-violations is disabled${NC}"
        exit 0
    fi
fi

echo ""
echo -e "${GREEN}✅ Architect Linter passed${NC}"
exit 0
