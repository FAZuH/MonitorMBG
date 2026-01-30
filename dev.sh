#!/bin/bash

# Development helper script for MonitorMBG
# Usage: ./dev.sh [target] [command]
#   target: backend (default) | frontend | docs
#   command: test | format | lint | build | mermaid | help

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Default values
TARGET="${1:-backend}"
COMMAND="${2:-help}"

# If only one argument provided and it's a command, assume backend target
if [ $# -eq 1 ]; then
    case "$1" in
        test|format|lint|build|mermaid|help)
            TARGET="backend"
            COMMAND="$1"
            ;;
    esac
fi

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

Usage: ./dev.sh [target] [command]

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
  help      - Show this help message

Examples:
  ./dev.sh test              # Run backend tests
  ./dev.sh backend test      # Run backend tests
  ./dev.sh frontend build    # Build frontend
  ./dev.sh docs mermaid      # Compile Mermaid diagrams
  ./dev.sh format            # Format backend code

Backend Testing:
  If database is not running, start it with:
    docker compose up --build -d db

EOF
}

# Backend commands
backend_test() {
    print_info "Running backend tests..."
    cd backend
    
    # Check if database is running
    if ! pg_isready -h localhost -p 5432 -U postgres > /dev/null 2>&1; then
        print_warning "Database not found. Starting PostgreSQL..."
        cd ..
        docker compose up --build -d db
        sleep 5
        cd backend
    fi
    
    cargo test --all-features
    print_success "Backend tests completed"
}

backend_format() {
    print_info "Formatting backend code..."
    cd backend
    cargo +nightly fmt --all
    print_success "Backend formatting completed"
}

backend_lint() {
    print_info "Linting backend code..."
    cd backend
    cargo clippy --all-targets --all-features --fix --allow-dirty
    print_success "Backend linting completed"
}

backend_build() {
    print_info "Building backend..."
    cd backend
    cargo build --release
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
    cd frontend
    npm test
    print_success "Frontend tests completed"
}

frontend_format() {
    print_info "Formatting frontend code..."
    cd frontend
    npm run format
    print_success "Frontend formatting completed"
}

frontend_lint() {
    print_info "Linting frontend code..."
    cd frontend
    npm run lint
    print_success "Frontend linting completed"
}

frontend_build() {
    print_info "Building frontend..."
    cd frontend
    npm run build
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
    
    cd docs
    
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
}

# Main execution
case "$TARGET" in
    backend)
        case "$COMMAND" in
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
                print_error "Unknown command: $COMMAND"
                show_help
                exit 1
                ;;
        esac
        ;;
    
    frontend)
        case "$COMMAND" in
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
                print_error "Unknown command: $COMMAND"
                show_help
                exit 1
                ;;
        esac
        ;;
    
    docs)
        case "$COMMAND" in
            mermaid)
                docs_mermaid
                ;;
            help)
                show_help
                ;;
            *)
                print_error "Unknown command: $COMMAND for docs target"
                show_help
                exit 1
                ;;
        esac
        ;;
    
    help)
        show_help
        ;;
    
    *)
        print_error "Unknown target: $TARGET"
        show_help
        exit 1
        ;;
esac
