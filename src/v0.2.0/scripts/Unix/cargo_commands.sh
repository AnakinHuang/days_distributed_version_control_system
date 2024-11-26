#!/bin/bash

# Sequentially execute cargo run commands

# Initialize a new repository
cargo run init

# Clone a remote repository
cargo run clone https://remote_url

# Add a file to the repository
cargo run add file.rs

# Remove a file from the repository
cargo run remove file.rs

# Show the repository status
cargo run status

# Display repository heads
cargo run heads

# Compare differences between two files
cargo run diff file1.rs file2.rs

# Display the contents of a file
cargo run cat file.rs

# Checkout a specific branch
cargo run checkout branch_name

# Commit changes with a message
cargo run commit 'Initial Commit'

# Show the commit log
cargo run log

# Merge a branch
cargo run merge branch_name

# Pull the latest changes
cargo run pull

# Push changes to the default branch
cargo run push

# Push changes to a specific branch
cargo run push branch_name

echo "All cargo commands executed successfully!"
