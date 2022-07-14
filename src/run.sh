#!/bin/bash

FILENAME="$1"
EXECUTABLE="$(cut -d '.' -f1 <<< "$1")"

rustc -o ./executable/$EXECUTABLE $FILENAME

# echo "rustc -o ./executable/$EXECUTABLE $FILENAME"

./executable/$EXECUTABLE