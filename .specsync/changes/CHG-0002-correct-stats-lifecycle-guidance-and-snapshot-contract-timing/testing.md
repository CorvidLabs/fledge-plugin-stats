---
change: CHG-0002-correct-stats-lifecycle-guidance-and-snapshot-contract-timing
artifact: testing
---

# Testing

- Run `fledge lanes run verify` for formatting, clippy, Cargo's truthful zero-test result, release build, and manifest validation.
- Run `specsync check --strict --require-coverage 100 --force` and require real Rust file and LOC coverage.
- Map `REQ-stats-001` to direct inspection of response-bearing request IDs and fire-and-forget store messages.
- Map `REQ-stats-004` to direct inspection of load, delta calculation, store, and final output ordering.
- Require hosted Trust and CodeQL success on the exact reviewed head before promotion.
