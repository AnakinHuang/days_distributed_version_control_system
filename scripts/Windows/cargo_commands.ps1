# DAYS DVCS BETA ACCEPTANCE TESTS
# This script automates the execution of acceptance tests for the DAYS DVCS system.
# It covers all levels of commands, from Level-1 to Level-7, as described in the specifications.

Write-Output "DAYS DVCS BETA ACCEPTANCE TESTS"

# Step 1: Build the project
Write-Output "Building the project..."
Set-Location -Path "beta_tests/acceptance_tests"
cargo build

# LEVEL-1: Init
Write-Output "Level-1: Init"

# Test-1a: Initialize a repository in a directory
Write-Output "Test-1a: Initializing repo_0"
New-Item -ItemType Directory -Path "repo_0"
Set-Location -Path "repo_0"
cargo run init
Set-Location -Path ..

# Test-1b: Initialize a repository with a name
Write-Output "Test-1b: Initializing repo_1"
cargo run init repo_1

# LEVEL-2: Add, Commit, Checkout
Write-Output "Level-2: Add, Commit, Checkout"

# Test-2a: Add a file, commit changes
Write-Output "Test-2a: Adding and committing test.txt in repo_1"
Set-Location -Path "repo_1"
"Hello World" | Out-File -FilePath "tests.txt"
cargo run add tests.txt
cargo run commit "Initial commit"

# Test-2b: Checkout a branch
Write-Output "Test-2b: Checkout a branch and verify working directory"
Set-Location -Path ..
cargo run init repo_2
Set-Location -Path "repo_2"
"Hello World" | Out-File -FilePath "tests.txt"
cargo run add tests.txt
cargo run commit "Add test.txt"
cargo run checkout main
Set-Location -Path ../..
cargo run checkout 'ade0dee6-3515-436b-941a-c6b147409d8d'

# LEVEL-3: Status, Heads, Cat, Log
Write-Output "Level-3: Status, Heads, Cat, Log"

# Test-3a: Status and log commands
Write-Output "Test-3a: Displaying status and log in repo_3"
Set-Location -Path "acceptance_tests"
cargo run init repo_3
Set-Location -Path "repo_3"
"Hello World" | Out-File -FilePath "test_1.txt"
"Hello World" | Out-File -FilePath "test_2.txt"
"Hello World" | Out-File -FilePath "test_3.txt"
cargo run add test_1.txt
cargo run log

# Test-3b: Adding and removing files, checking status
Write-Output "Test-3b: Add, remove, and check status"
cargo run add test_2.txt
cargo run status
cargo run remove test_1.txt
cargo run status

# Test-3c: Heads command
Write-Output "Test-3c: Checking branch heads"
cargo run add test_3.txt
cargo run commit "Add test_3.txt"
cargo run heads

# Test-3d: Cat command
Write-Output "Test-3d: Inspect file content from a specific revision"
Set-Location -Path ../..
cargo run cat 'e3e9b566-9369-4257-8697-068907f92dae' README.md
cargo run cat 'dacaba2a-b89e-4413-9ec3-81e636de7a88' program.rs
cargo run add README.md
cargo run commit "Add README.md"
Set-Location -Path "acceptance_tests/repo_3"

# LEVEL-4: Remove, Diff
Write-Output "Level-4: Remove, Diff"

# Test-4a: Remove files from staging
Write-Output "Test-4a: Removing files from staging and checking status"
"Hello World" | Out-File -FilePath "test_4.txt"
"Hello World" | Out-File -FilePath "test_5.txt"
cargo run add .
cargo run status
cargo run remove test_4.txt
cargo run status

# Test-4b: Diff command to compare revisions
Write-Output "Test-4b: Comparing revisions using diff"
cargo run commit "Add test_5.txt"
"Modified content" | Out-File -FilePath "test_5.txt"
Set-Location -Path ..
Add-Content -Path "../README.md" -Value "This is a new line of text"
cargo run add ../README.md
cargo run commit "Update README.md"
cargo run diff
cargo run diff '98b0436a-f409-4288-a5a4-c817133639ba' 'main'

# LEVEL-5: Merge
Write-Output "Level-5: Merge"

# Test-5a: Merging branches
Write-Output "Test-5a: Merging beta branch into main"
cargo run checkout beta
cargo run status
"Hello World" | Out-File -FilePath "test_6.txt"
cargo run add test_6.txt
cargo run commit "Add test_6.txt"
cargo run checkout main
cargo run status
cargo run merge beta

# Test-5b: Merging main branch into beta
Write-Output "Test-5b: Merging main into beta"
cargo run push
cargo run checkout beta
cargo run merge main
cargo run log

# LEVEL-6: Clone
Write-Output "Level-6: Clone"

# Test-6a: Cloning repo_1 to repo_4
Write-Output "Test-6a: Cloning repo_1 to repo_4"
cargo run clone repo_1 repo_4

# Test-6b: Cloning repo_4 to repo_5
Write-Output "Test-6b: Cloning repo_4 to repo_5"
cargo run clone repo_4 repo_5

# LEVEL-7: Push, Pull
Write-Output "Level-7: Push, Pull"

# Test-7a: Push changes to remote repository
Write-Output "Test-7a: Pushing changes from repo_4"
cargo run push
Set-Location -Path "repo_4"
Set-Location -Path ..
cargo run clone repo_5 repo_6
cargo run pull

# Test-7b: Pull changes from remote repository
Write-Output "Test-7b: Pulling changes into repo_6"
cargo run checkout beta
"Hello World" | Out-File -FilePath "test_7.txt"
cargo run add test_7.txt
cargo run commit "Add test_7.txt"
cargo run push
cargo run checkout main
cargo run push
cargo run pull

# Cleanup: Remove all tests repositories
Write-Output "Cleaning up test directories..."
Remove-Item -Recurse -Force .remote repo_* test_*.txt
Set-Location -Path "../../"

Write-Output "All DVCS acceptance tests executed successfully!"
