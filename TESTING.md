# Testing Guide

This document describes the testing strategy for the Guitar Practice Dashboard project.

## Overview

We use Rust's built-in testing framework for regression testing. Tests help ensure that:
- Music theory calculations remain correct
- Component logic doesn't regress
- Edge cases are handled properly
- Refactoring doesn't break existing functionality

## Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run a specific test
cargo test test_key_from_int

# Run tests in a specific module
cargo test music_theory::
```

## Test Structure

### Unit Tests

Unit tests are located in the same file as the code they test, using `#[cfg(test)]` modules:

- `src/music_theory.rs` - Tests for music theory calculations
  - Key/Scale conversions
  - Note calculations
  - Frequency calculations
  - Scale membership checks

### Future Test Modules

As we build out the project, we'll add:

1. **Integration Tests** (`tests/` directory)
   - Component initialization tests
   - UI interaction tests (if Slint supports headless testing)
   - Layout save/load tests

2. **Regression Tests**
   - Tests for bugs we've fixed
   - Stack overflow prevention tests
   - Memory leak tests

3. **Performance Tests**
   - Component creation benchmarks
   - Rendering performance tests

## Writing New Tests

When adding new features or fixing bugs:

1. **Write the test first** (TDD approach when possible)
2. **Test edge cases** (boundary conditions, null values, etc.)
3. **Test the fix** (for bug fixes, add a test that fails before the fix and passes after)
4. **Keep tests focused** (one assertion per test when possible)

### Example Test

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_feature() {
        // Arrange
        let input = /* setup */;
        
        // Act
        let result = function_to_test(input);
        
        // Assert
        assert_eq!(result, expected_value);
    }
}
```

## Test Coverage Goals

- **Critical paths**: 100% coverage (music theory, component logic)
- **UI logic**: High coverage (event handlers, callbacks)
- **Error handling**: Test all error paths
- **Edge cases**: Test boundary conditions

## CI/CD Integration

In the future, we'll integrate tests into CI/CD:
- Run tests on every commit
- Block merges if tests fail
- Generate coverage reports

## Known Issues

- Slint UI components are harder to test (may need mock UI)
- Audio testing may require mock audio backends
- Some tests may need platform-specific handling

## Best Practices

1. **Run tests frequently** - Before and after making changes
2. **Fix failing tests immediately** - Don't let them accumulate
3. **Add tests for bugs** - Prevent regressions
4. **Keep tests fast** - Unit tests should complete in <1 second
5. **Test behavior, not implementation** - Tests should survive refactoring

