# Contributing to Draco-OS

First of all, thank you for considering contributing to Draco-OS! It's people like you that make Draco-OS such an incredible project.

## How Can I Contribute?

### Reporting Bugs
This section guides you through submitting a bug report for Draco-OS. Following these guidelines helps maintainers and the community understand your report, reproduce the behavior, and find related reports.

- **Check open issues**: Before creating a bug report, please check our GitHub Issues to see if the problem has already been reported.
- **Provide clear steps to reproduce**: Describe the exact steps you took that led to the issue.
- **Include your environment**: Please specify the host OS, Rust version, and system specs you are running Draco-OS on.

### Suggesting Enhancements
This section guides you through submitting an enhancement suggestion for Draco-OS, including completely new features and minor improvements to existing functionality.

- **Explain why this enhancement would be useful** to most Draco-OS users.
- **Provide a potential API/CLI design** if applicable.

### Pull Requests
The process described here has several goals:
- Maintain Draco-OS's quality
- Fix problems that are important to users
- Engage the community in working toward the best possible Draco-OS

Please follow these steps to have your contribution considered by the maintainers:
1. Fork the repository and create your feature branch from `main`.
2. Ensure you have run `cargo fmt` and `cargo clippy` over your changes if they are Rust-related.
3. Update any relevant README.md or configuration documentation.
4. Issue that pull request!

### Setting Up Development
We use the standard Rust toolchain (Cargo) for all microservices in the `draco/` directory.

```bash
# Clone the repository
git clone https://github.com/XplnHUB/Draco-OS.git
cd Draco-OS

# Navigate to the workspace
cd draco

# Build the workspace
cargo build --workspace

# Run tests
cargo test --workspace
```

## Review Process
Once you submit a PR, a maintainer will review your code. We may request changes before it can be merged. When the code is ready, we will merge it into the `main` branch.

Thank you for contributing!
