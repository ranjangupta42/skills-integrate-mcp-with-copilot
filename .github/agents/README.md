# Code Review Agent

A custom GitHub Copilot agent specialized in reviewing Python, Go, and Rust code.

## Overview

This custom agent is designed to provide comprehensive code reviews for Python, Go, and Rust programming languages. It focuses on code quality, security, performance, and best practices specific to each language.

## Features

### Language Support

- **Python**: PEP 8 compliance, Python idioms, error handling, security vulnerabilities
- **Go**: Idiomatic Go code, concurrency issues, error handling patterns, resource management
- **Rust**: Ownership/borrowing, memory safety, unsafe code review, error handling with Result/Option

### Review Capabilities

- Code quality and maintainability assessment
- Security vulnerability detection
- Performance optimization suggestions
- Best practices enforcement
- Documentation review
- Architecture and design pattern feedback

## Usage

### In GitHub Copilot Chat

You can invoke this agent using natural language prompts in GitHub Copilot Chat:

#### Examples

1. **Python Code Review**:
   ```
   @code-review-agent Review this Python function for potential issues
   ```

2. **Go Concurrency Check**:
   ```
   @code-review-agent Check this Go code for race conditions
   ```

3. **Rust Memory Safety**:
   ```
   @code-review-agent Review this Rust code for memory safety
   ```

4. **Comprehensive Review**:
   ```
   @code-review-agent Perform a comprehensive code review on these files
   ```

### Review Focus Areas

The agent will check for:

#### Python
- PEP 8 style compliance
- Security vulnerabilities (SQL injection, XSS, etc.)
- Exception handling
- Type hints usage
- Performance optimizations
- Proper use of Python idioms

#### Go
- Idiomatic Go code
- Error handling patterns
- Race conditions and concurrency issues
- Goroutine and channel usage
- Resource cleanup
- Memory leaks

#### Rust
- Ownership and borrowing
- Lifetime management
- Unsafe code blocks
- Error handling with Result/Option
- Memory and thread safety
- Zero-cost abstractions

## Integration

This agent is configured in `.github/agents/code-review-agent.yml` and is automatically available in:

- GitHub Copilot Chat in VS Code
- GitHub Copilot Workspace
- GitHub Copilot CLI (if supported)

## Configuration

The agent has access to the following tools:
- File viewing and editing capabilities
- Shell command execution
- GitHub MCP server tools for repository operations

## Best Practices

1. Provide specific code snippets or file references when requesting reviews
2. Ask focused questions for better results
3. Review the agent's suggestions carefully and apply them judiciously
4. Use the agent as a learning tool to improve your coding skills

## Contributing

To modify this agent's behavior, edit the `.github/agents/code-review-agent.yml` file and adjust the instructions, tools, or examples as needed.
