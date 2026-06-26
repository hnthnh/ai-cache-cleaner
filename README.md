# 🧹 AI Cache Cleaner - Blazing Fast CLI Clean-up Tool in Rust

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust Version](https://img.shields.io/badge/rust-1.56%2B-orange.svg)](https://www.rust-lang.org/)

**AI Cache Cleaner** is a blazing-fast command-line tool (CLI) written in Rust to scan and clean cache directories, heavy model weights, and logs created by popular AI libraries and editors (Hugging Face, Ollama, PyTorch, Pip, Cursor, etc.).

---

## ⚡ Key Features
* **Smart Detection**: Instantly detects and calculates size of:
  * `hf` (Hugging Face Hub cache): `~/.cache/huggingface`
  * `ollama` (Local LLM weights): `~/.ollama/models`
  * `torch` (PyTorch model cache): `~/.cache/torch`
  * `pip` (Pip package download cache)
  * `cursor` (Cursor prompt logs and workspace history)
* **Custom Scans**: Scan and clean any custom project path.
* **Dry Run Mode**: Preview deletions safely using the `--dry-run` or `-n` flag before removing files.
* **Insanely Fast**: Written in Rust, using `walkdir` to traverse files in parallel without runtime garbage collection pauses.

---

## 🚀 Installation & Local Run

### Prerequisites
Make sure you have Rust installed on your Mac or Linux system. If not, install it using:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Build & Run from Source
```bash
# Clone the repository
git clone https://github.com/yourusername/ai-cache-cleaner.git
cd ai-cache-cleaner

# Build and run the project
cargo run -- scan

# Build production binary
cargo build --release

# The compiled binary will be available at:
# ./target/release/ai-cache-cleaner
```

### Install Globally
```bash
cargo install --path .

# Now run globally
ai-cache-cleaner scan
ai-cache-cleaner clean hf
```

---

## ⚙️ CLI Command Quick Reference

| Command | Action |
| :--- | :--- |
| `ai-cache-cleaner scan` | Scans system cache directories and reports details. |
| `ai-cache-cleaner clean` | Interactive mode (asks confirmation before cleaning each cache). |
| `ai-cache-cleaner clean all` | Purges all detected system cache areas (requires confirmation). |
| `ai-cache-cleaner clean hf` | Cleans Hugging Face Hub cache only. |
| `ai-cache-cleaner clean ollama` | Cleans Ollama models cache only. |
| `ai-cache-cleaner clean pip` | Cleans Python Pip cache only. |
| `ai-cache-cleaner clean torch` | Cleans PyTorch weights cache only. |
| `ai-cache-cleaner clean cursor` | Cleans Cursor AI logs only. |
| `ai-cache-cleaner clean <path>` | Cleans custom folder directory. |

### Safely previewing deletions (Dry Run)
You can append the `--dry-run` or `-n` option to any `clean` command to simulate deletions and see exactly what would be removed:
```bash
ai-cache-cleaner clean -n
ai-cache-cleaner clean all -n
ai-cache-cleaner clean hf --dry-run
```

---

## 📁 Repository Structure
```text
ai-cache-cleaner/
├── Cargo.toml                  # Rust project configuration and dependencies
├── README.md                   # Project documentation
├── LICENSE                     # MIT License
└── src/
    ├── main.rs                 # CLI entry point orchestrator
    ├── cli.rs                  # CLI argument parsing and routing
    ├── cleaner.rs              # Interactive clean workflows and file deletion
    ├── scanner.rs              # Cache directory discovery and size calculations
    ├── locations.rs            # Configured target directories (OS-specific paths)
    ├── display.rs              # Banner rendering and layout helper functions
    └── error.rs                # Custom error enum and conversion traits
```

---

## 📄 LICENSE
Distributed under the **MIT License**. Feel free to edit, share, and commercialize.

