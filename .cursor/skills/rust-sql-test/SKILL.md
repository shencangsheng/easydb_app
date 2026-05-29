---
name: rust-sql-test
description: >-
  Enforces unit test requirements for the Rust data-processing modules in
  src-tauri/src/sql/ (generator.rs, parse.rs) and src-tauri/src/reader/
  (excel.rs and other readers). Use when writing, editing, or reviewing Rust
  code in these modules. Ensures stability tests (and performance tests for
  hot-path code) are added for any change, and that tests follow the shared
  `<module>_test.rs` convention.
---

# Rust Data Module: Unit Test Requirements

## Rule

**Any change to code under `src-tauri/src/sql/` or `src-tauri/src/reader/` MUST be accompanied by corresponding unit tests.**

This applies to:
- New functions or structs
- Modified logic in existing functions
- Performance optimizations (require performance tests)
- Bug fixes (require regression tests)

## Test Structure

Tests live in separate `<module>_test.rs` files within the same module directory
(one test file per implementation file). Do NOT place `#[cfg(test)] mod tests`
blocks inline inside the implementation file.

```
src-tauri/src/sql/
├── generator.rs          # implementation
├── generator_test.rs     # tests for generator
├── parse.rs              # implementation
├── parse_test.rs         # tests for parse (if needed)
└── mod.rs                # registers test modules with #[cfg(test)]

src-tauri/src/reader/
├── excel.rs              # implementation
├── excel_test.rs         # tests for excel
└── mod.rs                # registers test modules with #[cfg(test)]
```

Register each test module in `mod.rs`:

```rust
pub mod excel;

#[cfg(test)]
mod excel_test;
```

Each test file imports the items under test from its sibling module:

```rust
use super::excel::{excel_cell_to_timestamp_nanos, infer_cell_data_type};
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
- `test_converts_excel_datetime_cell()`
- `test_non_datetime_cells_return_none()`

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

// In excel.rs:
pub(crate) fn excel_cell_to_timestamp_nanos(cell: &Data) -> Option<i64> { ... }
pub(crate) fn infer_cell_data_type(cell: &Data) -> DataType { ... }
```

## CI Integration

Tests run automatically in GitHub Actions via `.github/workflows/test.yml` on:
- Push to `main` or `ai/*` branches
- Pull requests targeting `main`

## Checklist

When modifying any file under `src-tauri/src/sql/` or `src-tauri/src/reader/`:

- [ ] Added/updated stability tests in the corresponding `<module>_test.rs` file (not inline)
- [ ] Added/updated performance tests for hot-path changes
- [ ] Test module is registered in `mod.rs` with `#[cfg(test)]`
- [ ] Internal items are `pub(crate)` if tests need access
- [ ] `cargo test --lib <module>::<module>_test` passes locally (e.g. `sql::generator_test`, `reader::excel_test`)
- [ ] No test uses `unwrap()` on fallible operations (use `?` or `assert!`)
