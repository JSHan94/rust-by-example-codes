#!/bin/bash

FILENAME="$1"
EXECUTABLE="$(cut -d '.' -f1 <<< "$1")"

mkdir -p executable

rustc -o ./executable/$EXECUTABLE $FILENAME

# echo "rustc -o ./executable/$EXECUTABLE $FILENAME"

./executable/$EXECUTABLE