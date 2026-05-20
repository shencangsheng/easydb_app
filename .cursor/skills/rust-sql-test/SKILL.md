---
name: rust-sql-test
description: >-
  Enforces unit test requirements for SQL query and performance optimization code
  in src-tauri/src/sql/. Use when writing, editing, or reviewing Rust code in
  the SQL module (generator.rs, parse.rs, or any new files under sql/). Ensures
  both stability and performance tests are added for any changes.
---

# Rust SQL Module: Unit Test Requirements

## Rule

**Any change to code under `src-tauri/src/sql/` MUST be accompanied by corresponding unit tests.**

This applies to:
- New functions or structs
- Modified logic in existing functions
- Performance optimizations (require performance tests)
- Bug fixes (require regression tests)

## Test Structure

Tests live in separate `*_test.rs` files within the same module directory:

```
src-tauri/src/sql/
├── generator.rs          # implementation
├── generator_test.rs     # tests for generator
├── parse.rs              # implementation
├── parse_test.rs         # tests for parse (if needed)
└── mod.rs                # registers test modules with #[cfg(test)]
```

Register each test module in `mod.rs`:

```rust
pub mod generator;
pub mod parse;

#[cfg(test)]
mod generator_test;
```

## Two Test Categories Required

### 1. Stability Tests

Every public or `pub(crate)` function must have stability tests covering:

| Aspect | What to test |
|--------|-------------|
| Correctness | Expected output for valid inputs |
| Edge cases | Empty strings, NULL, large values, negative numbers |
| Error handling | Invalid inputs return appropriate errors |
| Type safety | Each `SqlType` variant produces correct SQL output |
| Regression | Previously fixed bugs stay fixed |

Naming convention: `test_<function>_<scenario>()`

Examples:
- `test_format_bool_for_sql_known_values()`
- `test_resolve_export_specs_rejects_duplicate_export_names()`
- `test_format_cell_bool_to_int()`

### 2. Performance Tests

Any code in the hot formatting loop or called per-row must have performance tests:

| Aspect | What to test |
|--------|-------------|
| Time bound | Completes within a reasonable time for N iterations |
| Scaling | Does not degrade unexpectedly with more columns/rows |
| Comparison | Enum dispatch is not slower than string-based approach |

Naming convention: `test_perf_<function>_<scenario>()`

Examples:
- `test_perf_format_cell_for_sql_enum_dispatch()`
- `test_perf_resolve_export_specs_large_columns()`
- `test_perf_parse_sql_type_enum_approach()`

Performance test pattern:

```rust
#[test]
fn test_perf_<name>() {
    use std::time::Instant;
    let iterations = 50_000;
    let start = Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(/* call hot function */);
    }
    let duration = start.elapsed();
    assert!(
        duration.as_millis() < THRESHOLD_MS,
        "function too slow: {:?} for {} iterations",
        duration, iterations
    );
}
```

## Exposing Internals for Testing

Functions and types needed by tests must be marked `pub(crate)`:

```rust
// In generator.rs:
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum SqlType { ... }

pub(crate) fn format_cell_for_sql(...) -> String { ... }
```

## CI Integration

Tests run automatically in GitHub Actions via `.github/workflows/test.yml` on:
- Push to `main` or `ai/*` branches
- Pull requests targeting `main`

## Checklist

When modifying any file under `src-tauri/src/sql/`:

- [ ] Added/updated stability tests in the corresponding `*_test.rs` file
- [ ] Added/updated performance tests for hot-path changes
- [ ] Test module is registered in `mod.rs` with `#[cfg(test)]`
- [ ] Internal items are `pub(crate)` if tests need access
- [ ] `cargo test --lib sql::<module>_test` passes locally
- [ ] No test uses `unwrap()` on fallible operations (use `?` or `assert!`)
