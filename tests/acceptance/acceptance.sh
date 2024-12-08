#!/bin/bash

# Level 1: Init
cargo run init -d test_repo
echo "Failure expected here, since repository already exists:"
cargo run init -d test_repo

# Level 2: Add, Commit, Checkout
cargo run add new_file.txt
cargo run add changed_file.txt
cargo run add new_file.txt
echo "Failure expected here:"
cargo run commit "Empty commit"

# Level 3: Status, Heads, Cat, Log
cargo run status
cargo run heads
echo "Failure expected here: need to acquire revision id. Funcitonality is fine"
cargo run cat existing_file.txt
echo "Failure expected here: a directory shouldn't be read"
cargo run cat directory_name
cargo run log -d repo_name

# Level 4: Remove, Diff
cargo run add file1.txt
cargo run remove file1.txt
cargo run diff staged_file.txt
cargo run diff untracked_file.txt

# Level 5: Merge
cargo run merge branch1
cargo run merge branch2

# Level 6: Clone
cargo run clone existing_repo existing_clone
cargo run clone nonexistent_repo clone_dir

# Level 7: Pull, Push
cargo run pull origin main
cargo run push main
cargo run push main
