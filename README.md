# LogSweep

LogSweep is a simple command-line tool written in Rust for cleaning and searching log files.  
It helps developers quickly extract useful information from logs directly in the terminal.

This project was developed as part of a university course and focuses on Text Editing workflows using a Rust-based CLI application.

---

## 🚀 Features

- Remove empty lines from log files
- Filter and display only error messages
- Filter and display only warning messages
- Search for specific keywords in log files
- Simple and beginner-friendly command-line interface

---

## 🛠 Technologies Used

- Rust
- Cargo (Rust package manager)
- Git & GitHub

---

## 📦 Installation

### Requirements
- Rust installed (rustup recommended)

### Clone the repository
```bash
git clone https://github.com/YOUR_USERNAME/logsweep.git
cd logsweep
Build the project
cargo build
▶️ Usage
General command format:

cargo run <command> <file> [options]
Clean a log file
Removes empty lines and prints the cleaned output:

cargo run clean app.log
Show only error messages
cargo run clean app.log --errors
Example output:

ERROR Connection failed
Show only warning messages
cargo run clean app.log --warnings
Example output:

WARN Low memory
Search for a keyword
cargo run search app.log failed
Example output:

ERROR Connection failed
