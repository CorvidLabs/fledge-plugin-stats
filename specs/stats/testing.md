---
spec: stats.spec.md
---

## Test Plan

### Native Validation

- `cargo fmt --check`
- `cargo clippy -- -D warnings`
- `cargo test`
- `cargo build --release`

### Contract Validation

- Verify the plugin manifest declares `fledge-v1` and the release binary path.
- Run SpecSync strict validation at 100% file and LOC coverage.
- Run Trust doctor and the unified native verification lane.
