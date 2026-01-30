#!/bin/bash

# Development helper script for MonitorMBG
# Usage: ./dev.sh [target] [command1] [command2] ...
#   target: backend (default) | frontend | docs
#   commands: test | format | lint | build | mermaid | help
#   Multiple commands can be specified and will execute left to right

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

# Help function
show_help() {
    cat << EOF
MonitorMBG Development Helper

Usage: ./dev.sh [target] [command1] [command2] ...

Targets:
  backend   - Backend Rust application (default)
  frontend  - Frontend React application
  docs      - Documentation

Commands:
  test      - Run tests
  format    - Format code
  lint      - Run linter
  build     - Build the application
  mermaid   - Compile Mermaid diagrams (docs target only)
  precommit - Run all pre-commit checks (backend only)
  help      - Show this help message

Multiple commands can be specified and will execute sequentially from left to right.

Examples:
  ./dev.sh test                    # Run backend tests
  ./dev.sh backend test            # Run backend tests
  ./dev.sh frontend build          # Build frontend
  ./dev.sh docs mermaid            # Compile Mermaid diagrams
  ./dev.sh format                  # Format backend code
  ./dev.sh backend format lint     # Format then lint backend
  ./dev.sh backend precommit       # Run all pre-commit checks

Backend Testing:
  If database is not running, start it with:
    docker compose up --build -d db

EOF
}

# Backend commands
backend_test() {
    print_info "Running backend tests..."
    pushd backend > /dev/null
    
    # Check if database is running
    if ! pg_isready -h localhost -p 5432 -U postgres > /dev/null 2>&1; then
        print_warning "Database not found. Starting PostgreSQL..."
        popd > /dev/null
        docker compose up --build -d db
        sleep 5
        pushd backend > /dev/null
    fi
    
    cargo test --all-features
    popd > /dev/null
    print_success "Backend tests completed"
}

backend_format() {
    print_info "Formatting backend code..."
    pushd backend > /dev/null
    cargo +nightly fmt --all
    popd > /dev/null
    print_success "Backend formatting completed"
}

backend_lint() {
    print_info "Linting backend code..."
    pushd backend > /dev/null
    cargo clippy --all-targets --all-features --fix --allow-dirty
    popd > /dev/null
    print_success "Backend linting completed"
}

backend_build() {
    print_info "Building backend..."
    pushd backend > /dev/null
    cargo build --release
    popd > /dev/null
    print_success "Backend build completed"
}

backend_precommit() {
    print_info "Running pre-commit checks..."
    backend_format
    backend_lint
    backend_test
    print_success "All pre-commit checks passed"
}

# Frontend commands
frontend_test() {
    print_info "Running frontend tests..."
    pushd frontend > /dev/null
    npm test
    popd > /dev/null
    print_success "Frontend tests completed"
}

frontend_format() {
    print_info "Formatting frontend code..."
    pushd frontend > /dev/null
    npm run format
    popd > /dev/null
    print_success "Frontend formatting completed"
}

frontend_lint() {
    print_info "Linting frontend code..."
    pushd frontend > /dev/null
    npm run lint
    popd > /dev/null
    print_success "Frontend linting completed"
}

frontend_build() {
    print_info "Building frontend..."
    pushd frontend > /dev/null
    npm run build
    popd > /dev/null
    print_success "Frontend build completed"
}

# Docs commands
docs_mermaid() {
    print_info "Compiling Mermaid diagrams..."
    
    # Check if mmdc (Mermaid CLI) is installed
    if ! command -v mmdc &> /dev/null; then
        print_warning "Mermaid CLI not found. Installing..."
        npm install -g @mermaid-js/mermaid-cli
    fi
    
    pushd docs > /dev/null
    
    # Create output directory
    mkdir -p diagrams
    
    # Compile each .mmd file to PNG
    print_info "Processing .mmd diagram files..."
    
    for file in diagrams/*.mmd; do
        if [ -f "$file" ]; then
            filename=$(basename "$file" .mmd)
            print_info "Compiling $filename.mmd..."
            mmdc -i "$file" -o "diagrams/${filename}.png" -b transparent
        fi
    done
    
    print_success "Mermaid diagrams compiled to docs/diagrams/"
    popd > /dev/null
}

# Execute a single command for a target
execute_command() {
    local target="$1"
    local command="$2"
    
    case "$target" in
        backend)
            case "$command" in
                test)
                    backend_test
                    ;;
                format)
                    backend_format
                    ;;
                lint)
                    backend_lint
                    ;;
                build)
                    backend_build
                    ;;
                precommit)
                    backend_precommit
                    ;;
                help)
                    show_help
                    ;;
                *)
                    print_error "Unknown command: $command"
                    show_help
                    exit 1
                    ;;
            esac
            ;;
        
        frontend)
            case "$command" in
                test)
                    frontend_test
                    ;;
                format)
                    frontend_format
                    ;;
                lint)
                    frontend_lint
                    ;;
                build)
                    frontend_build
                    ;;
                help)
                    show_help
                    ;;
                *)
                    print_error "Unknown command: $command"
                    show_help
                    exit 1
                    ;;
            esac
            ;;
        
        docs)
            case "$command" in
                mermaid)
                    docs_mermaid
                    ;;
                help)
                    show_help
                    ;;
                *)
                    print_error "Unknown command: $command for docs target"
                    show_help
                    exit 1
                    ;;
            esac
            ;;
        
        help)
            show_help
            ;;
        
        *)
            print_error "Unknown target: $target"
            show_help
            exit 1
            ;;
    esac
}

# Main execution
# Parse arguments to determine target and commands
if [ $# -eq 0 ]; then
    show_help
    exit 0
fi

# Check if first argument is a target or a command
first_arg="$1"
is_target=false

case "$first_arg" in
    backend|frontend|docs)
        is_target=true
        ;;
esac

if [ "$is_target" = true ]; then
    # First arg is a target, rest are commands
    TARGET="$first_arg"
    shift
    
    if [ $# -eq 0 ]; then
        # No commands provided, show help
        show_help
        exit 0
    fi
    
    # Execute each command sequentially
    for command in "$@"; do
        execute_command "$TARGET" "$command"
    done
else
    # First arg is a command (default to backend target)
    TARGET="backend"
    
    # Execute each command sequentially
    for command in "$@"; do
        execute_command "$TARGET" "$command"
    done
fi
