#!/bin/bash

# Verification script for bundled Tesseract setup
echo "=== Law-gorithm Bundled Tesseract Verification ==="
echo

# Get the project root directory (parent of scripts directory)
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

# Check if vendor directory exists
if [ -d "$PROJECT_ROOT/vendor" ]; then
    echo "✅ Vendor directory found"
else
    echo "❌ Vendor directory not found at: $PROJECT_ROOT/vendor"
    exit 1
fi

# Check if tesseract-linux directory exists
if [ -d "$PROJECT_ROOT/vendor/tesseract-linux" ]; then
    echo "✅ Tesseract Linux binaries found"
else
    echo "❌ Tesseract Linux binaries not found at: $PROJECT_ROOT/vendor/tesseract-linux"
    exit 1
fi

# Check if tesseract executable exists
if [ -f "$PROJECT_ROOT/vendor/tesseract-linux/tesseract" ]; then
    echo "✅ Tesseract executable found"
else
    echo "❌ Tesseract executable not found at: $PROJECT_ROOT/vendor/tesseract-linux/tesseract"
    exit 1
fi

# Check if tessdata directory exists
if [ -d "$PROJECT_ROOT/vendor/tesseract-linux/tessdata" ]; then
    echo "✅ Tessdata directory found"
else
    echo "❌ Tessdata directory not found at: $PROJECT_ROOT/vendor/tesseract-linux/tessdata"
    exit 1
fi

# Check if English language data exists
if [ -f "$PROJECT_ROOT/vendor/tesseract-linux/tessdata/eng.traineddata" ]; then
    echo "✅ English language data found"
else
    echo "❌ English language data not found at: $PROJECT_ROOT/vendor/tesseract-linux/tessdata/eng.traineddata"
    exit 1
fi

# Check if required libraries exist
REQUIRED_LIBS=("libtesseract.so.5" "libleptonica.so.6" "libpng16.so.16" "libjpeg.so.62")
for lib in "${REQUIRED_LIBS[@]}"; do
    if [ -f "$PROJECT_ROOT/vendor/tesseract-linux/$lib" ]; then
        echo "✅ $lib found"
    else
        echo "❌ $lib not found at: $PROJECT_ROOT/vendor/tesseract-linux/$lib"
    fi
done

echo
echo "=== Configuration Check ==="
echo "OCR Config will automatically detect:"
echo "  - Tesseract binary: $PROJECT_ROOT/vendor/tesseract-linux/tesseract"
echo "  - Tessdata path: $PROJECT_ROOT/vendor/tesseract-linux/tessdata/"
echo "  - Language: eng"
echo
echo "✅ Bundled Tesseract setup is ready!"
echo "Users will not need to install Tesseract system-wide."
