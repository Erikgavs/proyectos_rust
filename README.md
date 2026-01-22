# 🦀 Rust Projects Collection

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A comprehensive collection of Rust projects that document my journey learning and mastering the Rust programming language. This repository contains projects ranging from basic syntax exercises to advanced GUI applications.

## 📋 Table of Contents

- [About](#about)
- [Project Categories](#project-categories)
  - [🛠️ Normal Projects](#️-normal-projects)
  - [🪓 Freya Projects](#-freya-projects)
- [Prerequisites](#prerequisites)
- [Getting Started](#getting-started)
- [Project Structure](#project-structure)
- [Contributing](#contributing)
- [Author](#-author)
- [License](#license)

## 📖 About

This repository contains all the projects that have helped me improve my skills with the Rust programming language. Each project is designed to explore different aspects of Rust, from fundamental concepts to advanced frameworks and libraries.

The repository is organized into different folders, each containing a dedicated README file with detailed explanations of the projects within.

## 🗂️ Project Categories

### 🛠️ Normal Projects

**Location:** [`normal_projects/`](./normal_projects/)

This folder contains projects that utilize "basic" Rust elements and core language features. These are smaller, focused projects designed to build comfort and proficiency with the language fundamentals.

**What you'll find:**
- Basic syntax and control flow exercises
- Data structures implementations
- File I/O operations
- Error handling patterns
- Concurrency basics
- Common Rust idioms and patterns

👉 [View Normal Projects README](./normal_projects/README.md)

### 🪓 Freya Projects

**Location:** [`freya_projects/`](./freya_projects/)

This folder features projects built with [Freya](https://freyaui.dev/), a native GUI framework for Rust. Projects range from basic GUI applications to more advanced interactive programs.

**What you'll find:**
- Basic GUI components and layouts
- Event handling and state management
- Custom widgets and styling
- Interactive applications
- Progressive complexity from beginner to advanced

👉 [View Freya Projects README](./freya_projects/README.md)

## 🔧 Prerequisites

Before running any project in this repository, ensure you have the following installed:

- **Rust**: Version 1.70 or higher
  ```bash
  # Check your Rust version
  rustc --version

  # Update Rust if needed
  rustup update
  ```

- **Cargo**: Comes bundled with Rust
  ```bash
  # Verify Cargo installation
  cargo --version
  ```

### Installing Rust

If you don't have Rust installed, visit [rustup.rs](https://rustup.rs/) or run:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## 🚀 Getting Started

### Clone the Repository

```bash
git clone https://github.com/Erikgavs/proyectos_rust.git
cd proyectos_rust
```

### Running a Project

Navigate to any project directory and use Cargo commands:

```bash
# Navigate to a specific project
cd normal_projects/<project-name>

# Build the project
cargo build

# Run the project
cargo run

# Run tests (if available)
cargo test
```

### For Freya Projects

Freya projects may require additional dependencies depending on your operating system. Refer to the [Freya documentation](https://book.freyaui.dev/setup.html) for platform-specific setup instructions.

## 📁 Project Structure

```
proyectos_rust/
├── normal_projects/     # Core Rust language projects
│   ├── README.md       # Detailed list of normal projects
│   └── ...             # Individual project folders
├── freya_projects/     # GUI framework projects
│   ├── README.md       # Detailed list of Freya projects
│   └── ...             # Individual project folders
└── README.md           # This file
```

## 🤝 Contributing

This is a personal learning repository, but suggestions and feedback are always welcome! Feel free to:

- Open an issue for bugs or suggestions
- Submit a pull request with improvements
- Share your thoughts on the projects

## 🧙‍♂️ Author

**Erik Garcia**
- GitHub: [@Erikgavs](https://www.github.com/Erikgavs)

## 📜 License

This project is open source and available under the [MIT License](LICENSE).

---

<div align="center">
  <b>Happy Coding with Rust! 🦀</b>
</div>
