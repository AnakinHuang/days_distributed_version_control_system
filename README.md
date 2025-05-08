# DAYS DVCS: Distributed Version Control System

![License: BSD](https://img.shields.io/badge/License-BSD%203--Clause-blue.svg)

## 🚀 Overview

**DAYS** is a lightweight, Rust-based distributed version control system (DVCS) developed for the University of Rochester Computer Science Undergraduate Council (CSUG) Fedora servers. It provides core Git-like functionalities including repository creation, commits, branching, merging, and remote synchronization. The system is fully tested and verified for reliability.

---

## 🧩 Features

- **Repository Management:** `init`, `clone`, `commit`, `log`, `checkout`
- **Branching & Merging:** `branch`, `merge`, `heads`
- **File Operations:** `add`, `remove`, `status`, `diff`, `cat`
- **Remote Sync:** `push`, `pull`
- **Metadata Handling:** Robust revision and branch tracking

---

## 🛠️ Architecture

The system is organized into three core modules:

1. **File System Layer:** Manages file and directory operations, metadata storage.
2. **Behavioral Layer:** Handles command parsing, logic, and output.
3. **Repository Layer:** Controls repository state, revisions, branches, and synchronization.

Each module is well-documented and rigorously tested.

---

## 📂 Directory Structure

```
days_dvcs/
├── src/
│   ├── a_1_file_system_hiding/
│   ├── a_2_behavioral_hiding/
│   ├── a_3_repository_hiding/
│   └── main.rs
├── tests/
├── scripts/
│   ├── Unix/
│   └── Windows/
├── Cargo.toml
└── README.md
```

---

## 🚀 Getting Started

### Prerequisites

- **Rust:** Install from [rust-lang.org](https://www.rust-lang.org/tools/install)

### Installation

1️⃣ **Clone the repository:**

```bash
git clone https://github.com/AnakinHuang/days_distributed_version_control_system.git
cd days_distributed_version_control_system
```

2️⃣ **Build the project:**

```bash
cargo build --release
```

---

## ⚙️ Usage

### On Unix/Linux:

```bash
chmod +x ./scripts/Unix/cargo_commands.sh
./scripts/Unix/cargo_commands.sh
```

### On Windows:

```powershell
.\scripts\Windows\cargo_commands.ps1
```

---

## ✅ Testing

Run all tests to ensure functionality:

```bash
cargo test
```

---

## 👥 Contributors

- **Yuesong (Anakin) Huang** – File System, Behavioral Logic, Repository Management
- **Yifan (Alvin) Jiang** – Behavioral Logic
- **Duy Pham** – Repository Management
- **Shervin Tursun-Zade** – File System

---

## 📄 License

This project is licensed under the BSD 3-Clause License – see the [LICENSE](LICENSE) file for details.

---
