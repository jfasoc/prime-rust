# Next Prime Finder

[![Rust](https://github.com/USERNAME/REPONAME/actions/workflows/rust.yml/badge.svg)](https://github.com/USERNAME/REPONAME/actions/workflows/rust.yml)
[![codecov](https://codecov.io/gh/USERNAME/REPONAME/branch/main/graph/badge.svg)](https://codecov.io/gh/USERNAME/REPONAME)

A simple command-line tool written in Rust to find the next prime number.

## Description

This tool takes a non-negative integer as input and checks if it is a prime number. If the number is not prime, it finds the next prime number.

## Installation

1.  Clone the repository:
    ```sh
    git clone https://github.com/USERNAME/REPONAME.git
    cd REPONAME/next_prime_finder
    ```
2.  Build the project:
    ```sh
    cargo build --release
    ```
    The executable will be located in `target/release/next_prime_finder`.

## Usage

Run the tool from the `next_prime_finder` directory:

```sh
cargo run -- <number>
```

Or run the compiled binary directly:

```sh
./target/release/next_prime_finder <number>
```

### Example

```sh
$ cargo run -- 10
10 is 2 digits long
10 is not prime, next prime is 11
Prime is 1 more
```

```sh
$ cargo run -- 7
7 is 1 digit long
7 is prime
```

**Note:** Please replace `USERNAME` and `REPONAME` in the badges and installation instructions with your actual GitHub username and repository name.