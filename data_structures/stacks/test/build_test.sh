#!/bin/sh

# Enable xtrace and errexit modes
set -xe

# Source code file name
SOURCE_FILE="test_stack.c"

# Output executable name
OUTPUT_EXECUTABLE="test_stack"

# Compiler command
CC="clang -std=c99"

# Compilation options (CFLAGS)
CFLAGS="-Wall -Wextra -lm -O2"

# Compile the source code
$CC $SOURCE_FILE $CFLAGS -o $OUTPUT_EXECUTABLE
