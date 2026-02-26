#!/bin/bash
#
# Architect Linter Pro - Automatic Installer/Updater
# Usage: ./install.sh [--check-only] [--force]
#

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BINARY_NAME="architect-linter-pro"
INSTALL_DIR="${CARGO_HOME:-$HOME/.cargo}/bin"
BACKUP_DIR="${INSTALL_DIR}/.architect-backups"
CURRENT_BINARY="${INSTALL_DIR}/${BINARY_NAME}"

# Flags
CHECK_ONLY=false
FORCE_UPDATE=false
VERBOSE=false

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --check-only)
            CHECK_ONLY=true
            shift
            ;;
        --force)
            FORCE_UPDATE=true
            shift
            ;;
        -v|--verbose)
            VERBOSE=true
            shift
            ;;
        *)
            echo "Unknown option: $1"
            echo "Usage: $0 [--check-only] [--force] [-v|--verbose]"
            exit 1
            ;;
    esac
done

# Functions
log_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

log_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

log_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

log_error() {
    echo -e "${RED}❌ $1${NC}"
}

get_version() {
    local binary=$1
    if [[ -f "$binary" ]]; then
        "$binary" --version 2>/dev/null || echo "unknown"
    else
        echo "not-installed"
    fi
}

get_project_version() {
    grep '^version = ' "$PROJECT_ROOT/Cargo.toml" | sed 's/.*"\([^"]*\)".*/\1/'
}

# Main logic
echo ""
echo "╔════════════════════════════════════════════════════════════╗"
echo "║   Architect Linter Pro - Installer/Updater                 ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""

log_info "Checking environment..."

# Check if Rust/Cargo is installed
if ! command -v cargo &> /dev/null; then
    log_error "Cargo not found. Please install Rust from https://rustup.rs/"
    exit 1
fi

log_success "Cargo found"

# Get versions
PROJECT_VERSION=$(get_project_version)
INSTALLED_VERSION=$(get_version "$CURRENT_BINARY")

echo ""
log_info "Version Check:"
echo "   📦 Project version:   v${PROJECT_VERSION}"
echo "   💾 Installed version: v${INSTALLED_VERSION}"
echo ""

# Check if update is needed
if [[ "$INSTALLED_VERSION" == "v${PROJECT_VERSION}" ]] && [[ "$FORCE_UPDATE" != "true" ]]; then
    log_success "Already up to date!"
    if [[ "$CHECK_ONLY" == "true" ]]; then
        exit 0
    fi
    read -p "Continue with installation anyway? (y/n) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 0
    fi
fi

if [[ "$CHECK_ONLY" == "true" ]]; then
    log_info "Check-only mode enabled"
    if [[ "$INSTALLED_VERSION" != "v${PROJECT_VERSION}" ]]; then
        log_warning "Update available: v${INSTALLED_VERSION} → v${PROJECT_VERSION}"
        exit 1
    else
        log_success "Already up to date"
        exit 0
    fi
fi

# Build
echo ""
log_info "Building Architect Linter Pro v${PROJECT_VERSION}..."
echo ""

cd "$PROJECT_ROOT"

if [[ "$VERBOSE" == "true" ]]; then
    cargo build --release
else
    cargo build --release 2>&1 | grep -E "Compiling architect|Finished|error|warning: unused" || true
fi

BUILT_BINARY="${PROJECT_ROOT}/target/release/${BINARY_NAME}"

if [[ ! -f "$BUILT_BINARY" ]]; then
    log_error "Build failed: Binary not found at $BUILT_BINARY"
    exit 1
fi

log_success "Build completed"

# Prepare installation
echo ""
log_info "Installing to ${INSTALL_DIR}..."

mkdir -p "$INSTALL_DIR"
mkdir -p "$BACKUP_DIR"

# Backup current version if it exists
if [[ -f "$CURRENT_BINARY" ]]; then
    TIMESTAMP=$(date +%Y%m%d_%H%M%S)
    BACKUP_PATH="${BACKUP_DIR}/${BINARY_NAME}.${INSTALLED_VERSION}.${TIMESTAMP}"

    log_info "Backing up current version to ${BACKUP_PATH}"
    mv "$CURRENT_BINARY" "$BACKUP_PATH"

    # Keep only last 3 backups
    ls -t "${BACKUP_DIR}/${BINARY_NAME}."* 2>/dev/null | tail -n +4 | xargs -r rm -f
fi

# Install new version
cp "$BUILT_BINARY" "$CURRENT_BINARY"
chmod +x "$CURRENT_BINARY"

# Verify installation
VERIFY_VERSION=$(get_version "$CURRENT_BINARY")
if [[ "$VERIFY_VERSION" == "v${PROJECT_VERSION}" ]]; then
    log_success "Installation verified: v${PROJECT_VERSION}"
else
    log_warning "Version verification mismatch: expected v${PROJECT_VERSION}, got ${VERIFY_VERSION}"
fi

echo ""
log_success "Installation complete!"
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  Updates in this version:"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Show changelog
if [[ -f "${PROJECT_ROOT}/CHANGELOG.md" ]]; then
    grep -A 10 "## v${PROJECT_VERSION}" "${PROJECT_ROOT}/CHANGELOG.md" 2>/dev/null || true
fi

echo ""
echo "Testing binary..."
"$CURRENT_BINARY" --version

echo ""
log_success "Ready to use! Run: architect-linter-pro --help"
echo ""
