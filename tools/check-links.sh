#!/bin/bash
set -euo pipefail

# Find all HTML files, excluding specified directories, and run lychee on them.
find . -type d \( -name "node_modules" -o -name "target" \) -prune -o -name "*.html" -print0 | xargs -0 lychee --verbose
