# Rust Style Guide

This document outlines the coding style and standards for this Rust project. We enforce these standards to maintain code quality, readability, and consistency.

## Formatting

We use `rustfmt` to automatically format our code. All code must be formatted with `rustfmt` before being committed.

### Indentation

We use 4 spaces for indentation. Tabs are not allowed. The `rustfmt.toml` file is configured to enforce this.

### Installation

If you don't have `rustfmt` installed, you can install it with:

```bash
rustup component add rustfmt
```

### Usage

To format the code, run the following command from the `next_prime_finder` directory:

```bash
cargo fmt
```

A pre-commit hook or a CI check can be set up to enforce this.

## Linting

We use `clippy` to catch common mistakes and improve our code. All code must pass `clippy`'s checks.

### Installation

If you don't have `clippy` installed, you can install it with:

```bash
rustup component add clippy
```

### Usage

To run `clippy`, use the following command from the `next_prime_finder` directory:

```bash
cargo clippy -- -D warnings
```

The `-D warnings` flag treats all warnings as errors, ensuring that no warnings are ignored.

## Naming Conventions

*   **Modules**: `snake_case`, e.g., `my_module`.
*   **Types (structs, enums, traits)**: `UpperCamelCase`, e.g., `MyStruct`.
*   **Functions and variables**: `snake_case`, e.g., `my_function`.
*   **Constants**: `UPPER_SNAKE_CASE`, e.g., `MY_CONSTANT`.
*   **Type parameters**: `UpperCamelCase`, concise, e.g., `T`.

## Documentation

All public functions, structs, and enums should have clear and concise documentation comments. Explain the purpose, arguments, and return values.

Example:

```rust
/// Calculates the factorial of a number.
///
/// # Arguments
///
/// * `n` - A non-negative integer.
///
/// # Returns
///
/// * The factorial of `n`.
fn factorial(n: u32) -> u32 {
    // ...
}
```

## Error Handling

Use `Result<T, E>` for functions that can fail. Avoid using `unwrap()` or `expect()` in library code; instead, propagate errors to the caller. `expect()` can be used in binaries or tests to provide a clear error message on panic.

## Testing

All new features should be accompanied by tests. Use `#[cfg(test)]` to create a test module in the same file as the code being tested. Use descriptive test names.

## Code Coverage

We require a high level of code coverage to ensure that our code is well-tested and reliable. The CI is configured to enforce the following coverage rules:

*   **Project Coverage**: The total code coverage of the project must be at least 95%.
*   **New Code Coverage**: Any new code added in a pull request must be 100% covered by tests.

### Generating a Coverage Report

To generate a coverage report locally, you can use the following steps. This requires the `llvm-tools-preview` component and `grcov`.

1.  **Install `llvm-tools-preview` and `grcov`**:
    ```bash
    rustup component add llvm-tools-preview
    cargo install grcov
    ```

2.  **Run tests with coverage enabled**:
    From the `next_prime_finder` directory, run:
    ```bash
    CARGO_INCREMENTAL=0 RUSTFLAGS="-Cinstrument-coverage" RUSTDOCFLAGS="-Cinstrument-coverage" LLVM_PROFILE_FILE="cargo-test-%p-%m.profraw" cargo test
    ```

3.  **Generate the report**:
    From the `next_prime_finder` directory, run:
    ```bash
    grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./coverage/
    ```
    This will generate an HTML report in the `next_prime_finder/coverage/` directory. You can open the `index.html` file in that directory to view the report.
