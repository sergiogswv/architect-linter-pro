#!/bin/bash
# Pre-push validation script
# Run this before pushing to ensure CI will pass

set -e  # Exit on error

echo "🚀 Running pre-push checks..."
echo ""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Track overall success
FAILED=0

# ============================================================================
# 1. Format Check
# ============================================================================
echo "📝 Checking code formatting..."
if cargo fmt --all -- --check > /dev/null 2>&1; then
    echo -e "${GREEN}✅ Formatting: Passed${NC}"
else
    echo -e "${RED}❌ Formatting: Failed${NC}"
    echo -e "${YELLOW}   Run: cargo fmt --all${NC}"
    FAILED=1
fi
echo ""

# ============================================================================
# 2. Clippy (Linting)
# ============================================================================
echo "🔍 Running clippy..."
if cargo clippy --all-targets --all-features -- -W clippy::all > /dev/null 2>&1; then
    echo -e "${GREEN}✅ Clippy: Passed (warnings allowed)${NC}"
else
    echo -e "${YELLOW}⚠️  Clippy: Warnings found (not failing)${NC}"
    # Don't fail on clippy warnings for now
fi
echo ""

# ============================================================================
# 3. Build
# ============================================================================
echo "🔨 Building project..."
if cargo build > /dev/null 2>&1; then
    echo -e "${GREEN}✅ Build: Passed${NC}"
else
    echo -e "${RED}❌ Build: Failed${NC}"
    echo -e "${YELLOW}   Run: cargo build${NC}"
    FAILED=1
fi
echo ""

# ============================================================================
# 4. Tests
# ============================================================================
echo "🧪 Running tests..."
TEST_OUTPUT=$(cargo test 2>&1)

if echo "$TEST_OUTPUT" | grep -q "test result: ok"; then
    # Count passed tests
    TESTS_PASSED=$(echo "$TEST_OUTPUT" | grep -oP '\d+(?= passed)' | awk '{s+=$1} END {print s}')
    echo -e "${GREEN}✅ Tests: $TESTS_PASSED passed${NC}"
else
    echo -e "${RED}❌ Tests: Failed${NC}"
    echo -e "${YELLOW}   Run: cargo test --verbose${NC}"
    FAILED=1
fi
echo ""

# ============================================================================
# 5. Security Audit (optional, requires cargo-audit)
# ============================================================================
if command -v cargo-audit &> /dev/null; then
    echo "🔒 Running security audit..."
    if cargo audit > /dev/null 2>&1; then
        echo -e "${GREEN}✅ Security Audit: Passed${NC}"
    else
        echo -e "${YELLOW}⚠️  Security Audit: Warnings found${NC}"
        echo -e "${YELLOW}   Run: cargo audit${NC}"
        # Don't fail on audit warnings
    fi
    echo ""
else
    echo -e "${YELLOW}⚠️  cargo-audit not installed (optional)${NC}"
    echo -e "${YELLOW}   Install: cargo install cargo-audit${NC}"
    echo ""
fi

# ============================================================================
# Final Result
# ============================================================================
if [ $FAILED -eq 0 ]; then
    echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${GREEN}✅ All pre-push checks passed!${NC}"
    echo -e "${GREEN}   Ready to push 🚀${NC}"
    echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    exit 0
else
    echo -e "${RED}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${RED}❌ Some checks failed!${NC}"
    echo -e "${RED}   Please fix the issues before pushing.${NC}"
    echo -e "${RED}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    exit 1
fi
