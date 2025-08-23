# Agent Guidelines for hash_hunter

## Build/Lint/Test Commands

### Python
- **Lint**: `ruff check .`
- **Format**: `ruff format .`
- **Test**: `pytest`
- **Test single file**: `pytest test_file.py`
- **Test with coverage**: `pytest --cov=./src`

### Rust
- **Build**: `cargo build --verbose`
- **Build (release)**: `cargo build --release`
- **Test**: `cargo test --verbose`
- **Test single**: `cargo test test_name`
- **Lint**: `cargo clippy`
- **Format**: `cargo fmt`
- **Run**: `cargo run`

### General
- **Pre-commit**: Run all hooks with `pre-commit run --all-files`
- **Full CI check**: Run both Python and Rust commands above

## Code Style Guidelines

### Python
- **Imports**: Standard library first, then third-party, then local imports. Use absolute imports.
- **Type hints**: Use type hints for function parameters and return values
- **Naming**: snake_case for variables/functions, PascalCase for classes
- **Constants**: UPPER_SNAKE_CASE for constants
- **Error handling**: Use try/except blocks, log errors with context
- **Logging**: Use loguru logger with appropriate levels (info, success, error)
- **Docstrings**: Use triple quotes for function/class documentation
- **Line length**: Keep lines under 100 characters when possible

### Rust
- **Naming**: snake_case for variables/functions, PascalCase for structs/enums, UPPER_SNAKE_CASE for constants
- **Error handling**: Use Result<T, E> pattern, avoid unwrap() in production code
- **Memory safety**: Prefer references over owned values when possible
- **Concurrency**: Use Arc/RwLock for shared mutable state across threads
- **Documentation**: Use /// for function/struct documentation
- **Testing**: Use #[test] attribute, place tests in same file or tests/ directory

### General
- **Comments**: Use descriptive comments for complex logic, avoid obvious comments
- **Error messages**: Include context and actionable information
- **Security**: Never log sensitive data (private keys, secrets)
- **Performance**: Consider algorithmic complexity, use parallel processing where appropriate
- **File organization**: Group related functionality, separate concerns
- **Configuration**: Use environment variables or config files for runtime settings

### Code Quality Tools
- **Python**: ruff (linting/formatting), pytest (testing), sourcery (code quality)
- **Rust**: clippy (linting), rustfmt (formatting), cargo test (testing)
- **Pre-commit hooks**: Enforced via .pre-commit-config.yaml

### Project Structure
- **Python**: Main code in src/, tests in tests/ or alongside code
- **Rust**: Standard Cargo workspace structure with src/ and tests/
- **Generated files**: Store in gen/ directory
- **Configuration**: Use TOML/YAML for complex config, environment variables for secrets

### Best Practices
- Always run lints and tests before committing
- Use meaningful commit messages following conventional format
- Document public APIs and complex algorithms
- Handle edge cases and validate inputs
- Follow security best practices (no hardcoded secrets, validate user input)
- Use version control appropriately (feature branches, pull requests)