# Rust Tutorial: A Hands-On Introduction

Welcome to this interactive Rust tutorial! This repository is designed to help beginners learn the fundamentals of the Rust programming language through practical examples.

## Why Rust?

Rust is a modern systems programming language focused on safety, speed, and concurrency. It achieves memory safety without a garbage collector, making it suitable for performance-critical applications, embedded systems, web assembly, and more.

Key features of Rust:
- **Memory Safety:** Prevents null pointer dereferences, buffer overflows, and data races at compile time.
- **Concurrency:** Fearless concurrency through its ownership and borrowing system.
- **Performance:** Comparable to C and C++ in terms of speed.
- **Zero-Cost Abstractions:** High-level abstractions that compile down to efficient machine code.
- **Rich Type System:** Supports strong, static typing with type inference.
- **Excellent Tooling:** Cargo (build system and package manager), Rustfmt (code formatter), and Clippy (linter) make development a breeze.

## Prerequisites

Before you begin, you'll need to install Rust on your system.

### Installing Rust

The recommended way to install Rust is by using `rustup`, the Rust toolchain installer.

1.  **Open your terminal and run the following command:**
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```
    This command downloads and runs `rustup-init.exe` (for Windows) or `rustup-init.sh` (for Unix-like systems), which installs the latest stable version of Rust.

2.  **Follow the on-screen instructions.** You might be prompted to choose an installation type. The default option is usually fine.

3.  **Configure your current shell:**
    After installation, `rustup` will attempt to configure your PATH environment variable. If it doesn't, or if you want to make sure, run:
    ```bash
    source $HOME/.cargo/env  # For Linux/macOS
    # For Windows, restart your terminal or follow rustup's instructions
    ```

4.  **Verify the installation:**
    Open a new terminal and type:
    ```bash
    rustc --version
    cargo --version
    ```
    You should see the versions of the Rust compiler (`rustc`) and Cargo.

For more detailed instructions or alternative installation methods, visit the [official Rust installation guide](https://www.rust-lang.org/tools/install).

## Tutorial Structure

This tutorial is organized into several modules, each focusing on a specific Rust concept. We will refactor the existing `prac*.rs` files into modules within a single application.

Each module will contain:
- Explanations of the concept.
- Code examples demonstrating the concept.
- Comments within the code to clarify specific lines or blocks.

**Topics to be covered (based on current files, will be expanded):**

1.  **Getting Started:**
    *   Basic syntax, `main` function.
    *   Variables and mutability (`let`, `mut`).
    *   Basic data types (integers, strings).
    *   Functions and parameters.
2.  **Ownership and Borrowing:**
    *   Understanding ownership (key to Rust's memory safety).
    *   References (`&`) and borrowing.
    *   Mutable references (`&mut`).
    *   Slices.
3.  **Data Types In-Depth:**
    *   Scalar types (integers, floating-point numbers, booleans, characters).
    *   Compound types (tuples, arrays).
4.  **Collections:**
    *   Vectors (`Vec<T>`).
    *   Strings (`String`, `&str`).
    *   Hash Maps (`HashMap<K, V>`) (to be added).
5.  **Control Flow:**
    *   `if/else` expressions.
    *   `match` expressions.
    *   Loops (`loop`, `while`, `for`).
6.  **Structs and Enums:** (to be added/enhanced)
    *   Defining and instantiating structs.
    *   Methods on structs.
    *   Defining enums.
    *   `Option` and `Result` enums for error handling.
7.  **Modules and Project Organization:**
    *   Organizing code into modules.
    *   Using `mod` and `use`.
    *   Understanding Cargo and `Cargo.toml`.
8.  **Error Handling:** (to be added/enhanced)
    *   Panic vs. recoverable errors.
    *   The `Result` enum.
9.  **User Input:**
    *   Reading input from the console.

## How to Use This Tutorial

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/0x-Professor/Rust_Practice.git
    cd Rust_Practice
    ```
2.  **Navigate through the `src` directory.** The code is organized into modules (e.g., `basic_syntax.rs`, `string_ownership.rs`, etc.).
3.  **Read the explanations and code examples within each module.**
4.  **Compile and run the main application:**
    The project is structured as a single binary application that demonstrates concepts from various modules. To compile and run all demonstrations:
    ```bash
    cargo run
    ```
    *Note: The `user_input.rs` module will pause execution and wait for your input in the console.*

5.  **Run tests:**
    This tutorial includes examples of unit tests. To run all tests in the project:
    ```bash
    cargo test
    ```
    You'll learn more about writing tests in the "Testing in Rust" section below.

## Testing in Rust

Rust has built-in support for writing and running tests. Tests are functions annotated with the `#[test]` attribute. Cargo's test runner (`cargo test`) will compile your code in test mode and run these test functions.

### Writing Unit Tests

Unit tests are typically placed in the same file as the code they are testing, inside a module named `tests` annotated with `#[cfg(test)]`. This configuration tells Rust to compile and run the test code only when you run `cargo test`, not when you run `cargo build`.

Here's a general structure:

```rust
// Your module's code (e.g., in src/some_module.rs)
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Tests for this module
#[cfg(test)]
mod tests {
    use super::*; // Import items from the outer module (some_module)

    #[test]
    fn test_add_function() {
        assert_eq!(add(2, 2), 4);
    }

    #[test]
    fn another_test() {
        // You can have multiple tests
        assert_ne!(add(1, 1), 3); // assert_ne! checks for inequality
    }

    #[test]
    #[should_panic] // This test is expected to panic
    fn test_panics() {
        // Code that should cause a panic
        // panic!("This is an expected panic!");
    }
}
```

- `#[cfg(test)]`:  Only compile this module when `cargo test` is run.
- `use super::*;`: Imports all items from the parent module (the module containing the `tests` module).
- `#[test]`: Marks a function as a test.
- `assert_eq!(left, right)`: A macro that checks if two expressions are equal. If not, it panics.
- `assert_ne!(left, right)`: A macro that checks if two expressions are not equal. If not, it panics.
- `assert!(expression)`: A macro that checks if an expression evaluates to `true`. If not, it panics.
- `#[should_panic]`: An attribute that indicates the test function is expected to panic. The test passes if it panics, and fails if it doesn't.

We will add some example tests to the modules in this tutorial. You can run them using `cargo test`.

## Contributing

This tutorial is a work in progress. If you find any errors, have suggestions for improvement, or want to add more examples, please feel free to open an issue or submit a pull request.

Let's start learning Rust!
