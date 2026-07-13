---
change: CHG-0001-adopt-specsync-5-0-1-and-trust-1-0-0-governance-for-the-stats-fledge-plugin
artifact: testing
---

# Testing

- Run cargo fmt --check, clippy with warnings denied, tests, and release build.
- Validate the manifest protocol and binary path through the verify lane.
- Require SpecSync strict validation at 100% coverage.
- Require all four agent integrations to report installed.
- Run fledge trust doctor and fledge trust verify.
- Confirm hosted CI and the trust job on the pull request.

Requirement evidence maps `REQ-stats-001` to protocol exchange inspection, `REQ-stats-002` to exclusion-command inspection, `REQ-stats-003` to report assembly inspection, `REQ-stats-004` to load/store message inspection, and `REQ-stats-005` to parsing fallback inspection. The native lane compiles and exercises the binary surface without claiming nonexistent unit coverage.
