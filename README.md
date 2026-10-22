# Privacy Hands-on CLI (`handson`)

[![CI](https://github.com/PrivacyCodeOrange/privacy-handson-cli/actions/workflows/rust.yml/badge.svg)](https://github.com/PrivacyCodeOrange/privacy-handson-cli/actions/workflows/rust.yml)

A companion command-line interface tool built for **Privacy Code Orange** hands-on workshops and technical labs (such as the Payjoin lab).

`privacy_hands_on` streamlines the onboarding and lab workflow for participants by:
- Authenticating directly via GitHub's OAuth Device Flow (no manual Personal Access Tokens required).
- Selecting and tracking active workshop labs.
- Automatically generating private student repositories from official workshop template repositories on GitHub.
- Securely storing local credentials and session metadata.

---

## Table of Contents

- [Features](#features)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Quick Start](#quick-start)
- [Usage & Commands](#usage--commands)
- [Configuration and Local Storage](#configuration-and-local-storage)
- [Development](#development)
---

## Features

- **GitHub Device Authorization Flow**: Simple web-based sign-in using short one-time verification codes.
- **Automated Repository Generation**: Instantiates private repositories from GitHub templates directly into the student's GitHub account.
- **GitHub App Integration**: Verifies and guides installation of the [Privacy Hands-on GitHub App](https://github.com/apps/privacy-hands-on) with appropriate repository permissions.
- **Secure Local Storage**: Respects the XDG base directory specification (`~/.config/handson`) and restricts file permissions (`0600`) for tokens and credentials.

---

## Prerequisites

- **Rust & Cargo**: Rust 1.85 or newer (Rust 2024 edition support required). You can install or update Rust using [rustup](https://rustup.rs/):
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  rustup update
  ```
- **Git**: Ensure `git` is installed and available in your `PATH`.
- **A GitHub Account**: Required to authenticate and create private student repositories.

---

## Installation

### Install from repository

1. Clone the repository:
   ```bash
   cargo install --git https://github.com/PrivacyCodeOrange/privacy-handson-cli.git
   ```

2. Install the binary locally into `~/.cargo/bin`:
   ```bash
   cargo install --path .
   ```

   Verify the installation:
   ```bash
   handson --help
   ```

## Quick Start

Here is a standard end-to-end workflow for a workshop participant:

```bash
# 1. Authenticate with GitHub
privacy_hands_on login

# 2. Verify your authenticated identity
privacy_hands_on whoami

# 3. Activate the relevant lab
privacy_hands_on activate <lab name>

# 4. Generate your private student lab repository from the template
privacy_hands_on createrepo <template repository>
```

---

## Usage & Commands

```
Usage: privacy_hands_on <COMMAND>

Commands:
  login       Authenticate with GitHub via device flow
  whoami      Display current authenticated GitHub username
  createrepo  Create a private student repo from a template repository
  activate    Select and activate a workshop lab
  help        Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
```

---

## Configuration and Local Storage

Configuration and credentials are saved according to the XDG Base Directory specification:

- **Directory Path**: `$XDG_CONFIG_HOME/handson/` (defaults to `~/.config/handson/`).
- **Files**:
  - `token`: Contains the GitHub OAuth token (mode `0600`, `rw-------`).
  - `user_data`: JSON file containing the active lab and list of repositories created for each lab.

To reset your local credentials and state:
```bash
rm -rf ~/.config/handson
```

---

## Development

### Code Formatting and Linting

Format the code:
```bash
cargo fmt --all --check
```

Run clippy linter:
```bash
cargo clippy --all-targets --all-features -- -D warnings
```

Run tests:
```bash
cargo test --verbose
```
