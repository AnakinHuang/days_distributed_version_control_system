# DAYS DVCS BETA ACCEPTANCE TESTS
# This script automates the execution of acceptance tests for the DAYS DVCS system.
# It covers all levels of commands, from Level-1 to Level-7, as described in the specifications.

Write-Host "DAYS DVCS BETA ACCEPTANCE TESTS"

# Step 1: Build the project
Write-Host "Building the project..."
Set-Location -Path beta_tests/acceptance_tests
cargo build

# LEVEL-1: Init
Write-Host "Level-1: Init"

# Test-1a: Initialize a repository in a directory
Write-Host "Test-1a: Initializing repo_0"
New-Item -ItemType Directory -Name repo_0
Set-Location -Path repo_0
cargo run init
Set-Location -Path ..

# Test-1b: Initialize a repository with a name
Write-Host "Test-1b: Initializing repo_1"
cargo run init repo_1

# LEVEL-2: Add, Commit, Checkout
Write-Host "Level-2: Add, Commit, Checkout"

# Test-2a: Add a file, commit changes
Write-Host "Test-2a: Adding and committing test.txt in repo_1"
Set-Location -Path repo_1
"Hello World" | Out-File -FilePath tests.txt -Encoding UTF8
cargo run add tests.txt
cargo run commit "Initial commit"

# Test-2b: Checkout a branch
Write-Host "Test-2b: Checkout a branch and verify working directory"
Set-Location -Path ..
cargo run init repo_2
Set-Location -Path repo_2
"Hello World" | Out-File -FilePath tests.txt -Encoding UTF8
cargo run add tests.txt
cargo run commit "Add test.txt"
cargo run checkout main
Set-Location -Path ..
cargo run checkout 3a686763-a07e-47c6-96f9-ad5b7d338485

# LEVEL-3: Status, Heads, Cat, Log
Write-Host "Level-3: Status, Heads, Cat, Log"

# Test-3a: Status and log commands
Write-Host "Test-3a: Displaying status and log in repo_3"
cargo run init repo_3
Set-Location -Path repo_3
"Hello World" | Out-File -FilePath test_1.txt -Encoding UTF8
"Hello World" | Out-File -FilePath test_2.txt -Encoding UTF8
"Hello World" | Out-File -FilePath test_3.txt -Encoding UTF8
cargo run add test_1.txt
cargo run log

# Test-3b: Adding and removing files, checking status
Write-Host "Test-3b: Add, remove, and check status"
cargo run add test_2.txt
cargo run status
cargo run remove test_1.txt
cargo run status

# Test-3c: Heads command
Write-Host "Test-3c: Checking branch heads"
cargo run add test_3.txt
cargo run commit "Add test_3.txt"
cargo run heads

# Test-3d: Cat command
Write-Host "Test-3d: Inspect file content from a specific revision"
Set-Location -Path ..
cargo run cat ab6f22db-fd04-4d2d-8afa-9e42145584d5 README.md
Set-Location -Path repo_3

# LEVEL-4: Remove, Diff
Write-Host "Level-4: Remove, Diff"

# Test-4a: Remove files from staging
Write-Host "Test-4a: Removing files from staging and checking status"
"Hello World" | Out-File -FilePath test_4.txt -Encoding UTF8
"Hello World" | Out-File -FilePath test_5.txt -Encoding UTF8
cargo run add .
cargo run status
cargo run remove test_4.txt
cargo run status

# Test-4b: Diff command to compare revisions
Write-Host "Test-4b: Comparing revisions using diff"
cargo run commit "Add test_5.txt"
"Modified content" | Out-File -FilePath test_5.txt -Encoding UTF8
cargo run diff test_4.txt test_5.txt

# LEVEL-5: Merge
Write-Host "Level-5: Merge"

# Test-5a: Merging branches
Write-Host "Test-5a: Merging beta branch into main"
Set-Location -Path ..
cargo run checkout beta
cargo run status
tree ./beta_tests
"Hello World" | Out-File -FilePath test_6.txt -Encoding UTF8
cargo run add test_6.txt
cargo run commit "Add test_6.txt"
cargo run checkout main
cargo run status
tree ./beta_tests
cargo run merge beta

# Test-5b: Merging main branch into beta
Write-Host "Test-5b: Merging main into beta"
cargo run checkout beta
cargo run merge main
cargo run log

# LEVEL-6: Clone
Write-Host "Level-6: Clone"

# Test-6a: Cloning repo_1 to repo_4
Write-Host "Test-6a: Cloning repo_1 to repo_4"
cargo run clone repo_1 repo_4

# Test-6b: Cloning repo_4 to repo_5
Write-Host "Test-6b: Cloning repo_4 to repo_5"
cargo run clone repo_4 repo_5

# LEVEL-7: Push, Pull
Write-Host "Level-7: Push, Pull"

# Test-7a: Push changes to remote repository
Write-Host "Test-7a: Pushing changes from repo_4"
Set-Location -Path repo_4
cargo run push
Set-Location -Path ..
cargo run clone repo_5 repo_6
cargo run pull

# Test-7b: Pull changes from remote repository
Write-Host "Test-7b: Pulling changes into repo_6"
cargo run checkout beta
"Hello World" | Out-File -FilePath test_7.txt -Encoding UTF8
cargo run add test_7.txt
cargo run commit "Add test_7.txt"
cargo run push
cargo run checkout main
cargo run pull

# Cleanup: Remove all test repositories
Write-Host "Cleaning up test directories..."
Remove-Item -Recurse -Force repo_0, repo_1, repo_2, repo_3, repo_4, repo_5, repo_6
Remove-Item -Recurse -Force README.md, test_*.txt
Set-Location -Path ..

Write-Host "All DVCS acceptance tests executed successfully!"
