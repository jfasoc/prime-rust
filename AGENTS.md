# Agent Instructions

This document provides instructions for AI agents working with this repository.

## Development Workflow

1.  **Install Dependencies**: The project uses Rust and Cargo. Ensure you have a recent version of the Rust toolchain installed.
2.  **Run Tests**: Before making any changes, and after making changes, run the test suite to ensure everything is working correctly.
    ```bash
    cd next_prime_finder
    cargo test
    ```
3.  **Check Formatting**: Ensure your code is formatted with `rustfmt`.
    ```bash
    cd next_prime_finder
    cargo fmt -- --check
    ```
4.  **Run Clippy**: Run `clippy` to check for lints.
    ```bash
    cd next_prime_finder
    cargo clippy -- -D warnings
    ```
5.  **Submit Changes**: Once all checks pass, you can submit your changes.

## Style Guide

Refer to the `STYLE_GUIDE.md` for detailed information on the coding style.
