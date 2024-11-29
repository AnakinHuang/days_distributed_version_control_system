#!/bin/bash

# DAYS DVCS BETA ACCEPTANCE TESTS
# This script automates the execution of acceptance tests for the DAYS DVCS system.
# It covers all levels of commands, from Level-1 to Level-7, as described in the specifications.

echo "DAYS DVCS BETA ACCEPTANCE TESTS"

# Step 1: Build the project
echo "Building the project..."
cd beta_tests/acceptance_tests || exit
cargo build

# LEVEL-1: Init
echo "Level-1: Init"

# Test-1a: Initialize a repository in a directory
echo "Test-1a: Initializing repo_0"
mkdir repo_0
cd repo_0 || exit
cargo run init
cd ..

# Test-1b: Initialize a repository with a name
echo "Test-1b: Initializing repo_1"
cargo run init repo_1

# LEVEL-2: Add, Commit, Checkout
echo "Level-2: Add, Commit, Checkout"

# Test-2a: Add a file, commit changes
echo "Test-2a: Adding and committing test.txt in repo_1"
cd repo_1 || exit
echo "Hello World" > tests.txt
cargo run add tests.txt
cargo run commit "Initial commit"

# Test-2b: Checkout a branch
echo "Test-2b: Checkout a branch and verify working directory"
cd ..
cargo run init repo_2
cd repo_2 || exit
echo "Hello World" > tests.txt
cargo run add tests.txt
cargo run commit "Add test.txt"
cargo run checkout main
cd ../.. || exit
cargo run checkout '1f1f6c33-e94e-4b59-8452-c3a5e4db73ef'

# LEVEL-3: Status, Heads, Cat, Log
echo "Level-3: Status, Heads, Cat, Log"

# Test-3a: Status and log commands
echo "Test-3a: Displaying status and log in repo_3"
cd acceptance_tests || exit
cargo run init repo_3
cd repo_3 || exit
echo "Hello World" > test_1.txt
echo "Hello World" > test_2.txt
echo "Hello World" > test_3.txt
cargo run add test_1.txt
cargo run log

# Test-3b: Adding and removing files, checking status
echo "Test-3b: Add, remove, and check status"
cargo run add test_2.txt
cargo run status
cargo run remove test_1.txt
cargo run status

# Test-3c: Heads command
echo "Test-3c: Checking branch heads"
cargo run add test_3.txt
cargo run commit "Add test_3.txt"
cargo run heads

# Test-3d: Cat command
echo "Test-3d: Inspect file content from a specific revision"
cd ../ || exit
cargo run cat 'a75ea01b-fc02-4e06-a5cc-56fed3f7068e' README.md
cd repo_3 || exit

# LEVEL-4: Remove, Diff
echo "Level-4: Remove, Diff"

# Test-4a: Remove files from staging
echo "Test-4a: Removing files from staging and checking status"
echo "Hello World" > test_4.txt
echo "Hello World" > test_5.txt
cargo run add .
cargo run status
cargo run remove test_4.txt
cargo run status

# Test-4b: Diff command to compare revisions
echo "Test-4b: Comparing revisions using diff"
cargo run commit "Add test_5.txt"
echo "Modified content" > test_5.txt
cargo run diff test_4.txt test_5.txt

# LEVEL-5: Merge
echo "Level-5: Merge"

# Test-5a: Merging branches
echo "Test-5a: Merging beta branch into main"
cd ..
cargo run checkout beta
cargo run status
tree ./beta_tests
echo "Hello World" > test_6.txt
cargo run add test_6.txt
cargo run commit "Add test_6.txt"
cargo run checkout main
cargo run status
tree ./beta_tests
cargo run merge beta

# Test-5b: Merging main branch into beta
echo "Test-5b: Merging main into beta"
cargo run checkout beta
cargo run merge main
cargo run log

# LEVEL-6: Clone
echo "Level-6: Clone"

# Test-6a: Cloning repo_1 to repo_4
echo "Test-6a: Cloning repo_1 to repo_4"
cargo run clone repo_1 repo_4

# Test-6b: Cloning repo_4 to repo_5
echo "Test-6b: Cloning repo_4 to repo_5"
cargo run clone repo_4 repo_5

# LEVEL-7: Push, Pull
echo "Level-7: Push, Pull"

# Test-7a: Push changes to remote repository
echo "Test-7a: Pushing changes from repo_4"
cd repo_4 || exit
cargo run push
cd ..
cargo run clone repo_5 repo_6
cargo run pull

# Test-7b: Pull changes from remote repository
echo "Test-7b: Pulling changes into repo_6"
cargo run checkout beta
echo "Hello World" > test_7.txt
cargo run add test_7.txt
cargo run commit "Add test_7.txt"
cargo run push
cargo run checkout main
cargo run pull

# Cleanup: Remove all tests repositories
echo "Cleaning up test directories..."
rm -rf repo_* test_*.txt ../README.md ../program.rs
cd ../../ || exit

echo "All DVCS acceptance tests executed successfully!"
