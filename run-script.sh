#!/bin/bash

# Script runner for Law-gorithm project
# Usage: ./run-script.sh <category> <script_name>

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/scripts" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print usage
print_usage() {
    echo -e "${BLUE}Law-gorithm Script Runner${NC}"
    echo
    echo "Usage: $0 <category> <script_name>"
    echo
    echo "Available categories:"
    echo "  verification    - Verification and validation scripts"
    echo
    echo "Examples:"
    echo "  $0 verification verify_tesseract.sh"
    echo "  $0 verification verify_tesseract"
    echo
    echo "Or run scripts directly:"
    echo "  ./scripts/verification/verify_tesseract.sh"
}

# Check if arguments are provided
if [ $# -lt 2 ]; then
    print_usage
    exit 1
fi

CATEGORY="$1"
SCRIPT_NAME="$2"

# Remove .sh extension if provided
SCRIPT_NAME="${SCRIPT_NAME%.sh}"

# Construct script path
SCRIPT_PATH="$SCRIPT_DIR/$CATEGORY/${SCRIPT_NAME}.sh"

# Check if script exists
if [ ! -f "$SCRIPT_PATH" ]; then
    echo -e "${RED}Error: Script '$SCRIPT_NAME' not found in category '$CATEGORY'${NC}"
    echo
    echo "Available scripts in '$CATEGORY':"
    if [ -d "$SCRIPT_DIR/$CATEGORY" ]; then
        ls -1 "$SCRIPT_DIR/$CATEGORY"/*.sh 2>/dev/null | xargs -n1 basename | sed 's/\.sh$//' | sed 's/^/  - /'
    else
        echo "  Category '$CATEGORY' does not exist"
    fi
    exit 1
fi

# Check if script is executable
if [ ! -x "$SCRIPT_PATH" ]; then
    echo -e "${YELLOW}Warning: Making script executable${NC}"
    chmod +x "$SCRIPT_PATH"
fi

# Run the script
echo -e "${GREEN}Running script: $CATEGORY/$SCRIPT_NAME${NC}"
echo
"$SCRIPT_PATH" "$@"
