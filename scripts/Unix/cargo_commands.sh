#!/bin/bash

# Sequentially execute cargo run commands

# Initialize a new repository
cargo run init test_repo_1

# Clone a remote repository
cargo run clone test_repo_2

# Add a file to the repository
cargo run add program.rs

# Remove a file from the repository
cargo run remove program.rs

# Show the repository status
cargo run status

# Display repository heads
cargo run heads

# Compare differences between two files
cargo run diff file1.rs file2.rs

# Display the contents of a file
cargo run cat test_repo 123 program.rs

# Checkout a specific branch
cargo run checkout main

# Commit changes with a message
cargo run commit 'Initial Commit'

# Show the commit log
cargo run log

# Merge a branch
cargo run merge main

# Pull the latest changes
cargo run pull

# Push changes to the default branch
cargo run push

# Push changes to a specific branch
cargo run push main

echo "All cargo commands executed successfully!"
