# AGENTS.md - Repository Guidelines for Agentic Coding

## Project Overview
This is a Rust experimental project containing graphics simulations and games using the kiss3d graphics library. The project uses Rust 2021 edition.

## Build, Lint, and Test Commands

### Essential Commands
```bash
# Build the project
cargo build

# Build with optimizations (release)
cargo build --release

# Run the conway binary
cargo run --bin conway

# Check for compilation errors without building
cargo check

# Run tests
cargo test

# Run a single test
cargo test test_name

# Run tests with output
cargo test -- --nocapture

# Format code
cargo fmt

# Check formatting without making changes
cargo fmt --all -- --check

# Run linter
cargo clippy

# Run clippy with strict warnings (CI standard)
cargo clippy -- -D warnings

# Check documentation
cargo doc --no-deps --open
```

### CI Commands
The project uses GitHub Actions with these commands:
- `cargo check` - Compilation verification
- `cargo test` - Test suite execution
- `cargo fmt --all -- --check` - Formatting verification
- `cargo clippy -- -D warnings` - Linting with warnings as errors

## Code Style Guidelines

### Imports and Dependencies
- Use `extern crate` statements for external crates (as seen in main.rs:1)
- Group imports by purpose: std library, external crates, local modules
- Use qualified imports for specific types and traits
- Import from `kiss3d` and related graphics libraries should be grouped together

### Naming Conventions
- **Types**: PascalCase (e.g., `Board`, `Tile`, `Simulation`)
- **Functions**: snake_case (e.g., `is_alive`, `set_dead`, `get_neighbors`)
- **Variables**: snake_case (e.g., `grid_state`, `num_of_alive_neighbors`)
- **Constants**: SCREAMING_SNAKE_CASE
- **Private fields**:snake_case with descriptive names

### Code Organization
- Keep related structs and implementations together
- Use `impl` blocks for each type, grouping related functionality
- Separate public API from internal implementation details
- Use `pub` modifier explicitly for public methods and types

### Error Handling
- Use `Result<T, E>` for operations that can fail
- Handle errors explicitly with `match`, `if let`, or `?` operator
- Avoid using `.unwrap()` except in examples or tests where panics are acceptable
- Provide meaningful error messages

### Documentation
- Use `///` for public API documentation
- Use `//` for implementation comments
- Document struct fields and function parameters where purpose isn't obvious
- Include usage examples for complex functionality

### Graphics and Performance Guidelines
- Use `Vector3<f32>` and similar types for graphics coordinates
- Prefer `f32` for graphics calculations to match kiss3d expectations
- Consider performance for grid operations - avoid unnecessary allocations in loops
- Use `Vec::with_capacity()` when size is known to improve performance

### Testing
- Unit tests should be placed in the same module using `#[cfg(test)]`
- Integration tests go in `tests/` directory
- Test names should describe what they're testing
- Use descriptive assertions with clear failure messages

### Memory Management
- Use references (`&`) and borrowed data where possible to avoid unnecessary clones
- Use `Box<T>` for large heap allocations (as seen with `world: Box<Board>`)
- Be mindful of cloning large data structures in loops
- Use iterators instead of index-based loops when appropriate

### Specific Patterns in This Codebase
- Grid-based simulations use `Vec<Vec<T>>` for 2D arrays
- Coordinates use `Coords2D` struct with `row` and `col` fields
- Graphics state and logic state are separated (grid_state vs grid_graphics)
- Color values use normalized 0.0-1.0 range for kiss3d compatibility

## Project Structure
```
src/
├── conway/
│   └── bin/
│       └── main.rs    # Conway's Game of Life implementation
examples/               # Example code
Cargo.toml             # Project configuration
```

## Development Workflow
1. Run `cargo check` frequently during development
2. Use `cargo fmt` before committing
3. Run `cargo clippy` and address warnings
4. Ensure `cargo test` passes before PR
5. Use `cargo run --bin conway` to test the main application

## Dependencies Notes
- `kiss3d`: 3D graphics library for visualizations
- `rand`: Random number generation for initial configurations
- Both dependencies are actively maintained and suitable for experimental projects