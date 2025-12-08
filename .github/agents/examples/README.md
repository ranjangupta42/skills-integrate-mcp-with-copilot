# Code Review Agent Examples

This directory contains example code files in Python, Go, and Rust that demonstrate the types of issues the Code Review Agent can detect.

## Example Files

### Python (`example_python.py`)
Contains examples of:
- SQL injection vulnerability
- Missing error handling
- Resource management issues
- PEP 8 style violations
- Performance anti-patterns

### Go (`example_go.go`)
Contains examples of:
- Missing error handling
- Race conditions
- Resource leaks
- Non-idiomatic Go code
- Improper use of panic

### Rust (`example_rust.rs`)
Contains examples of:
- Unsafe code blocks
- Unwrap() calls that can panic
- Memory leaks
- Non-idiomatic Rust patterns
- Missing Result types

## Using These Examples

You can use these example files to test the Code Review Agent:

1. Open one of the example files in your editor
2. Invoke the Code Review Agent in GitHub Copilot Chat:
   ```
   @code-review-agent Review this file for issues
   ```
3. The agent will identify the intentional problems and suggest fixes

## Expected Issues

Each file contains intentional code quality issues that represent common mistakes developers make. The agent should detect and provide feedback on:

- **Security vulnerabilities**
- **Performance problems**
- **Code style violations**
- **Error handling gaps**
- **Resource management issues**
- **Concurrency problems**
- **Memory safety concerns**

These examples serve as both test cases and educational resources for understanding what the agent looks for during code reviews.
