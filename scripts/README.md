# Scripts Directory

This directory contains utility scripts for the Law-gorithm project.

## Directory Structure

```
scripts/
├── verification/          # Verification and validation scripts
│   └── verify_tesseract.sh
├── run-script.sh         # Script runner (in parent directory)
└── README.md             # This file
```

## Script Runner

A convenient script runner is available at the project root:

```bash
./run-script.sh <category> <script_name>
```

**Examples:**
```bash
# Run Tesseract verification
./run-script.sh verification verify_tesseract

# You can omit the .sh extension
./run-script.sh verification verify_tesseract
```

## Scripts Overview

### Verification Scripts (`verification/`)

#### `verify_tesseract.sh`
- **Purpose**: Verifies that the bundled Tesseract OCR setup is complete and functional
- **Usage**:
  ```bash
  ./run-script.sh verification verify_tesseract
  # or directly:
  ./scripts/verification/verify_tesseract.sh
  ```
- **Checks**:
  - Vendor directory exists
  - Tesseract binaries are present
  - Required libraries are available
  - Language data files are installed
  - Configuration paths are correct

## Usage

### Running Scripts

```bash
# From project root
./scripts/verification/verify_tesseract.sh

# Or from scripts directory
cd scripts/verification
./verify_tesseract.sh
```

### Adding New Scripts

When adding new scripts:

1. **Choose appropriate category**: Place scripts in relevant subdirectories
2. **Make executable**: `chmod +x script_name.sh`
3. **Document in README**: Update this file with script description
4. **Add usage examples**: Include command-line usage in documentation

### Script Categories

- `verification/` - Scripts for validating setup and configuration
- `build/` - Build and deployment scripts (future)
- `development/` - Development workflow scripts (future)
- `maintenance/` - System maintenance and cleanup scripts (future)

## Contributing

When contributing scripts:

- Use descriptive names with `.sh` extension
- Include shebang (`#!/bin/bash`) at the top
- Add comments explaining complex operations
- Handle errors gracefully with appropriate exit codes
- Test scripts on target platforms before committing
