# DAYS DVCS: Distributed Version Control System

## Description

**DAYS** is a distributed version control system (DVCS) developed for the University of Rochester Computer Science
Undergraduate Council (CSUG) hosted Fedora machines. It provides robust functionality for local repository creation,
commits, branching, merging, and file inspection. The system has been fully tested and passed all unit and acceptance
tests, ensuring full functionality and reliability.

## Features

The **days** DVCS system implements the following commands:

1. **init**: Create an empty repository.
2. **clone**: Copy an existing repository.
3. **add**: Add specific files that you want to track.
4. **remove**: Remove specific files from the tracking list.
5. **status**: Check the current status of the repository.
6. **heads**: Show the current branch heads.
7. **diff**: Compare changes between revisions.
8. **cat**: Inspect a file of a given revision.
9. **checkout**: Switch to a specific revision.
10. **commit**: Commit changes and create a new revision.
11. **log**: View the change log.
12. **merge**: Merge two revisions.
13. **pull**: Fetch and integrate changes from another repository.
14. **push**: Push changes to another repository.

## Contributors

- **Yuesong (Anakin) Huang** - A.1 File System Hiding - A.2 Behavioral Hiding - A.3 Repository Hiding
- **Yifan (Alvin) Jiang** - A.2 Behavioral Hiding
- **Duy Pham** - A.3 Repository Hiding
- **Shervin Tursun-Zade** - A.1 File System Hiding

## Directory Structure

```plaintext
/days_dvcs
├── cargo.lock
├── cargo.toml
├── README.md
├── src
│   ├── a_1_file_system_hiding
│   │   ├── b_1_1_file_interaction.rs
│   │   ├── b_1_2_directory_interaction.rs
│   │   ├── b_1_3_metadata_management.rs
│   │   └── mod.rs
│   ├── a_2_behavioral_hiding
│   │   ├── b_2_1_command_parser.rs
│   │   ├── b_2_2_command_handler.rs
│   │   ├── b_2_3_output_formatter.rs
│   │   └── mod.rs
│   ├── a_3_repository_hiding
│   │   ├── b_3_1_repository_management.rs
│   │   ├── b_3_2_revision_management.rs
│   │   ├── b_3_3_branch_management.rs
│   │   ├── b_3_4_synchronization_handler.rs
│   │   ├── b_3_5_repository_helper.rs
│   │   └── mod.rs
│   ├── lib.rs
│   └── main.rs
└── tests
    ├── a_1_file_system_hiding_unit_test.rs
    ├── a_2_behavioral_hiding_unit_test.rs
    └── a_3_repository_hiding_unit_test.rs
```

## Repository Structure

```plaintext
./
├── .dvcs
│   ├── .metadata
│   │   └── metadata.json
│   ├── HEAD
│   └── origin
│       ├── main
│       │   ├── .metadata
│       │   │   └── metadata.json
│       │   ├── commits
│       │   └── staging
│       ├── feature
│       │   ├── .metadata
│       │   │   └── metadata.json
│       │   ├── commits
│       │   └── staging
│       └── ...
└── .remote
    └── .dvcs
        ├── .metadata
        │   └── metadata.json
        ├── HEAD
        └── origin
            ├── main
            │   ├── .metadata
            │   │   └── metadata.json
            │   ├── commits
            │   └── staging
            └── ...
```

## Usage

### On Windows

Run the PowerShell script:

```powershell
./scripts/Windows/cargo_commands.ps1
```

### On Linux/Unix-based OS

Run the shell script:

```bash
chmod +x ./scripts/Unix/cargo_commands.sh
./scripts/Unix/cargo_commands.sh
```

## Status

The **DAYS** DVCS system is fully functional and has successfully passed all unit and acceptance tests, including the
implementation of all required commands. It is ready for production use on supported environments.
