#!/bin/bash
unzip "dense_instances.zip"
unzip "sparse_instances.zip"
mkdir "results"


# Specify the path to your Rust executable
RUST_EXECUTABLE="cargo run --release"

# Directory containing the .gph files
INSTANCES_DIR="dense_instances"

# Check if the instances directory exists
if [ ! -d "$INSTANCES_DIR" ]; then
    echo "Error: Instances directory '$INSTANCES_DIR' not found."
    exit 1
fi

# Run the Rust executable for each .gph file
for file in "$INSTANCES_DIR"/*.gph; do
    if [ -f "$file" ]; then
        filename="${file##*/}"  # Extract filename without path
        filename_without_extension="${filename%.gph}"  # Remove .gph extension
        echo "Running $filename..."
        $RUST_EXECUTABLE "$file" "${filename_without_extension}.txt"
    fi
done

# Directory containing the .gph files
INSTANCES_DIR="sparse_instances"

# Check if the instances directory exists
if [ ! -d "$INSTANCES_DIR" ]; then
    echo "Error: Instances directory '$INSTANCES_DIR' not found."
    exit 1
fi

# Run the Rust executable for each .gph file
for file in "$INSTANCES_DIR"/*.gph; do
    if [ -f "$file" ]; then
        filename="${file##*/}"  # Extract filename without path
        filename_without_extension="${filename%.gph}"  # Remove .gph extension
        echo "Running $filename..."
        $RUST_EXECUTABLE "$file" "${filename_without_extension}.txt"
    fi
done

echo "All .gph files processed."
rm -r dense_instances
rm -r sparse_instances
