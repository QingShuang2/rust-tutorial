#!/bin/bash

# Compile the Rust code
rustc lesson4.rs

# Check if compilation was successful
if [ $? -eq 0 ]; then
    echo "Compilation successful. Running the program..."
    # Run the compiled binary
    ./lesson4
else
    echo "Compilation failed."
fi