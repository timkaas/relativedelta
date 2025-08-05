## Migration from 0.2.x to 0.3.x

The `RelativeDelta` type has been significantly refactored in version 0.3.x. The main changes are:

### API Changes

1. **Builder Pattern**: The method to finalize a `RelativeDelta` construction has changed from `.new()` to `.build()`:

```rust
// 0.2.x
let delta = RelativeDelta::with_years(1).new();

// 0.3.x
let delta = RelativeDelta::with_years(1).build();
```

2. **Codebase Reorganization**: The codebase has been reorganized with separate modules:
   - `weekday.rs`: Contains the `Weekday` enum and related functionality
   - `chrono_impl.rs`: Implementations for the chrono crate integration
   - `time_impl.rs`: Implementations for the time crate integration
   - `from_error.rs`: Error handling for conversions

3. **Error Handling**: Improved error handling with a dedicated `FromError` enum using the thiserror crate.

4. **Serde Feature**: The serde feature has been renamed from "serde1" to "serde".

5. **Rust Edition**: Updated to Rust edition 2024 with MSRV 1.85.0.

### Migration Steps

1. Replace all instances of `.new()` with `.build()` when constructing `RelativeDelta` instances.
2. If you were using the "serde1" feature, update your Cargo.toml to use the "serde" feature instead.
3. Ensure your project is compatible with Rust 1.85.0 or later.
4. If you were directly accessing internal modules or implementation details, update your code to use the new module structure.
5. If you were handling errors from `RelativeDelta` conversions, update your code to handle the new error types from `from_error.rs`.