# Code Review Agent - Usage Guide

## Quick Start

The Code Review Agent is now available in your GitHub Copilot environment. This custom agent specializes in reviewing Python, Go, and Rust code.

### Activating the Agent

In GitHub Copilot Chat, you can reference the agent using:
```
@code-review-agent [your request]
```

## Common Use Cases

### 1. Review a Specific File
```
@code-review-agent Please review the file src/app.py for any issues
```

### 2. Check for Security Vulnerabilities
```
@code-review-agent Check this Python code for security vulnerabilities
```

### 3. Review for Best Practices
```
@code-review-agent Review this Go code for idiomatic patterns and best practices
```

### 4. Performance Optimization
```
@code-review-agent Analyze this Rust code for performance optimizations
```

### 5. Comprehensive Code Review
```
@code-review-agent Perform a comprehensive code review on all Python files in this directory
```

## What the Agent Reviews

### Python Code
- ✅ PEP 8 style compliance
- ✅ Security vulnerabilities (SQL injection, XSS, etc.)
- ✅ Error handling and exception management
- ✅ Type hints and documentation
- ✅ Performance anti-patterns
- ✅ Proper use of Python idioms

### Go Code
- ✅ Idiomatic Go patterns
- ✅ Error handling (proper error returns)
- ✅ Race conditions and concurrency issues
- ✅ Goroutine and channel management
- ✅ Resource cleanup (defer usage)
- ✅ Memory leaks

### Rust Code
- ✅ Ownership and borrowing patterns
- ✅ Lifetime management
- ✅ Unsafe code validation
- ✅ Error handling (Result/Option types)
- ✅ Memory and thread safety
- ✅ Zero-cost abstractions

## Example Workflows

### Workflow 1: Pre-commit Review
Before committing your changes:
```
@code-review-agent Review all my changes in this pull request
```

### Workflow 2: Learning from Examples
To understand common issues:
```
@code-review-agent Review the example files in .github/agents/examples/ and explain the issues
```

### Workflow 3: Security Audit
For security-focused review:
```
@code-review-agent Perform a security audit on the authentication module
```

### Workflow 4: Refactoring Assistance
When refactoring:
```
@code-review-agent I'm refactoring this function. Review my changes and suggest improvements
```

## Tips for Best Results

1. **Be Specific**: Provide file paths or code snippets for focused reviews
2. **Ask Follow-up Questions**: Engage with the agent to understand suggestions better
3. **Iterate**: Apply suggestions and ask for re-review
4. **Learn**: Use the agent as an educational tool to improve your coding skills

## Testing the Agent

You can test the agent using the example files in `.github/agents/examples/`:

1. **Python Example**: `.github/agents/examples/example_python.py`
2. **Go Example**: `.github/agents/examples/example_go.go`
3. **Rust Example**: `.github/agents/examples/example_rust.rs`

Each example contains intentional issues that the agent should detect.

## Troubleshooting

### Agent Not Responding
- Ensure you're using the `@code-review-agent` mention
- Check that the `.github/agents/code-review-agent.yml` file exists
- Restart your IDE if needed

### Incorrect Reviews
- Provide more context in your request
- Reference specific files or code sections
- Ask clarifying questions

## Integration with Workflows

This agent can be integrated into:
- Pre-commit hooks
- Pull request reviews
- CI/CD pipelines (where supported)
- Code review workflows

## Further Reading

- [Code Review Agent Configuration](.github/agents/code-review-agent.yml)
- [Example Code Files](.github/agents/examples/)
- [GitHub Copilot Documentation](https://docs.github.com/en/copilot)
