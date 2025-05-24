When writing code, please ensure that it adheres to the following quality standards:

## Core

- Embrace Rust's ownership model and borrow checker.
- Use $[derive] attributes when appropriate.
- Make sure the code is idiomatic and follows Rust's conventions.
- Make sure the code is maintainable and easy to read.
- Use strong typing and newtype patterns to prevent logic errors.

## Naming Conventions

Follow Rust's naming conventions:

- Use `snake_case` for functions and variables.
- Use `PascalCase` for types and traits.
- Use `SCREAMING_SNAKE_CASE` for constants and statics.
- Use `kebab-case` for crate names.

## Safety

- Minimize use of unsafe blocks; document why they're necessary when used
- Use appropriate lifetime annotations to ensure memory safety
- Prefer immutable variables (let) over mutable ones (let mut)
- Implement proper error handling using Result<T, E> and the ? operator

## Handling and Results

- Use `Result` and `Option` types for error handling instead of null values or exceptions.
- Implement proper error handling using `Result<T, E>` and the `?` operator.
- Ensure that error handling is consistent and provides meaningful feedback to the user.
- Avoid using `unwrap` or `expect` unless absolutely necessary, and document why it's safe to do so.
- Use `?` operator for error propagation to keep code clean and readable.

## Control Flow

- Use iterators and iterator combinators for efficient data processing.
- Use pattern matching effectively with `match` and `if let`.
- Use `for` loops and iterators instead of traditional `while` loops when possible.
- Use `match` statements for exhaustive pattern matching instead of `if` statements when applicable.
- Use `if let` and `while let` for conditional destructuring of enums and options.

## Documentation

- Write clear doc comments (///) for public APIs
- Include examples in documentation when appropriate
- Document error conditions and panics
- Use inline comments sparingly, only for complex logic
- Make sure to provide examples for public functions and modules in the documentation.
- Use `///` for public documentation and `//!` for module-level documentation.

## Testing

- Write unit tests for all code where reasonably possible to write tests.
- Write tests that are clear, concise, and focused on a single behavior.
- Follow the Arrange-Act-Assert (AAA) pattern in tests.
- Use descriptive test names that clearly indicate the purpose of the test.
- Ensure tests are deterministic and do not rely on external state or side effects.
- Write minimum documentation for tests, but ensure to document the purpose of the test and any important details.
- Place unit tests in a `#[cfg(test)]` module at the bottom of the file.
- Place integration tests in a separate `tests` directory.

```rust
// Example test structure
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_descriptive_name() {
        // Arrange
        let input = setup_test_data();

        // Act
        let result = function_under_test(input);

        // Assert
        assert_eq!(result, expected_output);
    }
}
```

- Aim for comprehensive coverage of both success and error paths.
- Test edge cases and boundary conditions.
- Include negative tests cases.
- Test public interfaces thoroughly.
- Use `assert!`, `assert_eq!`, and `assert_ne!` appropriately.

Example of error testing:

```rust
// Example error test pattern
#[test]
fn test_error_condition() {
    let result = function_that_can_fail();
    assert!(matches!(result, Err(MyError::SpecificError)));
}
```

Remember: Tests should be reliable, readable, and maintainable. They serve as documentation and regression protection for your code.

## Quality

- Use `cargo fmt` to format code according to Rust's style guidelines.
- Use `cargo clippy` to lint code and ensure it adheres to Rust's best practices.
- Use `cargo test` to run tests and ensure all tests pass before committing code.
- Use `cargo audit` to check for vulnerabilities in dependencies.
- Use `cargo deny` to check for license compliance and security issues in dependencies.
- Use `cargo outdated` to check for outdated dependencies and update them as necessary.
- Use `cargo update` to update dependencies to the latest compatible versions.
- Use `cargo tree` to visualize the dependency graph and identify potential issues.
- Use `cargo doc` to generate documentation and ensure it is up to date.
- Use `cargo bench` to benchmark code and identify performance bottlenecks.

## Namespacing

Some modules hoist the names of their submodules to the root of the module. This is done to make the API cleaner and easier to use. For example, if you have a module `foo` with a submodule `bar`, you can access `bar` directly from `foo` like this:

```rust
mod foo {
    pub mod bar {
        pub fn baz() {
            // ...
        }
    }
}

use foo::bar;
bar::baz();
```

**NOTE:** Whenever you are writing or reviewing code, please review the `lib.rs` file to ensure that the namespacing is done correctly. This helps maintain a clean and organized API surface.

## Continuous Improvement

- Continuously review and refactor your code following the rules above.
- Your first version of the code is not your final version. Always strive to improve it.
- Seek feedback from peers and be open to suggestions for improvement.
- Build on existing code, understand it, and make it better.
- Use version control effectively to track changes and collaborate with others.
