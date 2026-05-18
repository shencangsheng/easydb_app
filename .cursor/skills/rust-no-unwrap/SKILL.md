---
name: rust-no-unwrap
description: >-
  Enforces strict avoidance of unwrap() in Rust code. Use when writing, editing,
  or reviewing Rust (.rs) code in this project. Prevents unsafe panic-prone
  patterns and promotes proper error handling alternatives.
---

# Rust: Avoid unwrap()

## Rule

**Do not use `.unwrap()` on `Option` or `Result` types unless absolutely certain the value is always present.**

When writing or editing Rust code in this project:

1. **Never use `unwrap()`** on values that can reasonably fail — file I/O, network calls, user input, parsing, indexing, HashMap lookups, etc.
2. **`unwrap()` is only acceptable** when the code can mathematically prove the value is always present, e.g.:
   - `Some(x)` that was just constructed in the same expression
   - A `Result` from an operation that by construction cannot fail (e.g., `1_usize.checked_add(0).unwrap()`)
   - Test code where panicking on unexpected states is intentional

## Alternatives

Replace `unwrap()` with one of the following, depending on context:

| Situation | Replacement | Example |
|-----------|-------------|---------|
| Need the value, want to propagate errors | `?` operator | `let val = result?;` |
| Need the value, have a sensible default | `.unwrap_or()` / `.unwrap_or_default()` | `map.get(k).unwrap_or(0)` |
| Need the value, want to provide context on failure | `.expect("reason")` | `file.expect("config must be readable")` |
| Need the value, custom error type | `.map_err()` then `?` | `result.map_err(|e| MyError::Foo(e))?` |
| Want to handle both branches | `match` | `match opt { Some(v) => ..., None => ... }` |
| Want to handle both branches with fallback | `.or_else()` / `.or()` | `opt.or_else(|| compute_fallback())` |
| Option on a collection lookup | `.get()` + fallback | `map.get(key).copied().unwrap_or_default()` |

## Decision Flow

```
Can the value ever be None/Err in practice?
  → YES: Use ? or match or unwrap_or/expect with clear reason
  → UNCERTAIN: Assume YES — use ? or match or unwrap_or
  → NO (provably always Ok/Some): unwrap() is acceptable
```

## Review Checklist

When reviewing or editing Rust files, check:

- [ ] No `.unwrap()` calls on I/O results, parse results, collection lookups, or any fallible operation
- [ ] Every `.expect()` has a meaningful, specific reason string (not just "should work")
- [ ] Error types implement proper conversion so `?` chains cleanly
- [ ] Test code that intentionally uses `unwrap()` is clearly in a `#[test]` or test helper context

## Examples

**Bad — panics on missing key:**
```rust
let config = settings.get("host").unwrap();
```

**Good — provides fallback:**
```rust
let config = settings.get("host").unwrap_or("localhost");
```

**Bad — panics on file read:**
```rust
let data = fs::read_to_string(path).unwrap();
```

**Good — propagates error:**
```rust
let data = fs::read_to_string(path)?;
```

**Bad — vague expect:**
```rust
let port = str::parse::<u16>(s).expect("parse failed");
```

**Good — specific expect:**
```rust
let port = str::parse::<u16>(s).expect("port number must be valid u16");
```

**Acceptable — provably safe:**
```rust
// Just created this Option, it is always Some
Some(42).unwrap()
```