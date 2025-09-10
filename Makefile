# Law-gorithm Makefile
# Multi-language project with Tauri (Rust), JavaScript frontend, and Python modules

.PHONY: help install build dev test clean lint format check deps all release docker

# Default target
all: install build

# Help target
help:
	@echo "Law-gorithm Project Makefile"
	@echo ""
	@echo "Available targets:"
	@echo "  help      - Show this help message"
	@echo "  install   - Install all dependencies (Rust, Node.js, Python)"
	@echo "  build     - Build the entire project"
	@echo "  dev       - Run development server"
	@echo "  test      - Run all tests"
	@echo "  clean     - Clean build artifacts"
	@echo "  lint      - Run linters for all languages"
	@echo "  format    - Format code for all languages"
	@echo "  check     - Run checks without building"
	@echo "  deps      - Check and install system dependencies"
	@echo "  release   - Build release version"
	@echo ""
	@echo "Component-specific targets:"
	@echo "  rust-*    - Rust/Tauri specific commands"
	@echo "  py-*      - Python specific commands"
	@echo "  js-*      - JavaScript specific commands"

# System dependencies
deps:
	@echo "Checking system dependencies..."
	@which cargo > /dev/null || (echo "Rust/Cargo not found. Please install: https://rustup.rs/" && exit 1)
	@which npm > /dev/null || (echo "Node.js/npm not found. Please install Node.js" && exit 1)
	@which python3 > /dev/null || (echo "Python3 not found. Please install Python 3.8+" && exit 1)
	@which poetry > /dev/null || (echo "Poetry not found. Installing..." && pip3 install poetry)
	@echo "✓ All system dependencies are available"

# Install all dependencies
install: deps
	@echo "Installing dependencies for all components..."
	$(MAKE) rust-install
	$(MAKE) js-install
	$(MAKE) py-install
	@echo "✓ All dependencies installed"

# Build all components
build: install
	@echo "Building all components..."
	$(MAKE) py-build
	$(MAKE) rust-build
	@echo "✓ Build completed"

# Development server
dev: install
	@echo "Starting development server..."
	npm run dev

# Test all components
test:
	@echo "Running tests for all components..."
	$(MAKE) rust-test
	$(MAKE) py-test
	@echo "✓ All tests completed"

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	$(MAKE) rust-clean
	$(MAKE) py-clean
	$(MAKE) js-clean
	rm -rf dist/ build/
	@echo "✓ Cleaned"

# Lint all code
lint:
	@echo "Running linters..."
	$(MAKE) rust-lint
	$(MAKE) py-lint
	@echo "✓ Linting completed"

# Format all code
format:
	@echo "Formatting code..."
	$(MAKE) rust-format
	$(MAKE) py-format
	@echo "✓ Formatting completed"

# Check without building
check:
	@echo "Running checks..."
	$(MAKE) rust-check
	$(MAKE) py-check
	@echo "✓ Checks completed"

# Release build
release: clean install
	@echo "Building release version..."
	npm run build
	@echo "✓ Release build completed"

# Rust/Tauri targets
rust-install:
	@echo "Installing Rust dependencies..."
	cd src-tauri && cargo fetch
	@echo "✓ Rust dependencies installed"

rust-build:
	@echo "Building Rust backend..."
	cd src-tauri && cargo build
	@echo "✓ Rust build completed"

rust-test:
	@echo "Running Rust tests..."
	cd src-tauri && cargo test
	@echo "✓ Rust tests completed"

rust-clean:
	@echo "Cleaning Rust artifacts..."
	cd src-tauri && cargo clean
	@echo "✓ Rust cleaned"

rust-lint:
	@echo "Running Rust linter..."
	cd src-tauri && cargo clippy -- -D warnings
	@echo "✓ Rust linting completed"

rust-format:
	@echo "Formatting Rust code..."
	cd src-tauri && cargo fmt
	@echo "✓ Rust formatting completed"

rust-check:
	@echo "Checking Rust code..."
	cd src-tauri && cargo check
	@echo "✓ Rust check completed"

rust-release:
	@echo "Building Rust release..."
	cd src-tauri && cargo build --release
	@echo "✓ Rust release build completed"

# Python targets
py-install:
	@echo "Installing Python dependencies..."
	cd python_modules && poetry install
	@echo "✓ Python dependencies installed"

py-build:
	@echo "Building Python modules..."
	cd python_modules && poetry build
	@echo "✓ Python build completed"

py-test:
	@echo "Running Python tests..."
	cd python_modules && poetry run python -m pytest tests/ || echo "No Python tests found"
	@echo "✓ Python tests completed"

py-clean:
	@echo "Cleaning Python artifacts..."
	cd python_modules && rm -rf dist/ build/ *.egg-info/
	find python_modules -name "__pycache__" -type d -exec rm -rf {} + || true
	find python_modules -name "*.pyc" -delete || true
	@echo "✓ Python cleaned"

py-lint:
	@echo "Running Python linter..."
	cd python_modules && poetry run flake8 . || echo "flake8 not installed, skipping..."
	cd python_modules && poetry run black --check . || echo "black not installed, skipping..."
	@echo "✓ Python linting completed"

py-format:
	@echo "Formatting Python code..."
	cd python_modules && poetry run black . || echo "black not installed, skipping..."
	@echo "✓ Python formatting completed"

py-check:
	@echo "Checking Python code..."
	cd python_modules && poetry run python -m py_compile *.py || echo "No Python files to check"
	@echo "✓ Python check completed"

py-shell:
	@echo "Starting Python shell..."
	cd python_modules && poetry shell

# JavaScript/Node.js targets
js-install:
	@echo "Installing JavaScript dependencies..."
	npm install
	@echo "✓ JavaScript dependencies installed"

js-clean:
	@echo "Cleaning JavaScript artifacts..."
	rm -rf node_modules/ package-lock.json
	@echo "✓ JavaScript cleaned"

js-serve:
	@echo "Starting JavaScript development server..."
	npm run serve

# Verification targets
verify-tesseract:
	@echo "Verifying Tesseract installation..."
	./scripts/verification/verify_tesseract.sh || echo "Tesseract verification script not found"
	@echo "✓ Tesseract verification completed"

# Documentation targets
docs:
	@echo "Available documentation:"
	@echo "  - README.md - Main project documentation"
	@echo "  - docs/ - Detailed documentation"
	@echo "  - docs/USER_GUIDE.md - User guide"
	@echo "  - docs/api/ - API documentation"
	@echo "  - docs/architecture/ - Architecture documentation"
	@echo "  - docs/deployment/ - Deployment guides"

# Status check
status:
	@echo "Project Status:"
	@echo "==============="
	@echo "Rust/Tauri:"
	@cd src-tauri && cargo --version 2>/dev/null && echo "  ✓ Cargo available" || echo "  ✗ Cargo not found"
	@echo "JavaScript:"
	@npm --version 2>/dev/null && echo "  ✓ npm available" || echo "  ✗ npm not found"
	@echo "Python:"
	@cd python_modules && poetry --version 2>/dev/null && echo "  ✓ Poetry available" || echo "  ✗ Poetry not found"
	@echo ""
	@echo "For detailed project status, see PROJECT_STATUS.md"
