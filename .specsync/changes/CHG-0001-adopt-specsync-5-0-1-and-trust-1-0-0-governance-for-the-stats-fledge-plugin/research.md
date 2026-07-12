---
change: CHG-0001-adopt-specsync-5-0-1-and-trust-1-0-0-governance-for-the-stats-fledge-plugin
artifact: research
---

# Research

- The implementation is one Rust 2021 binary with no external crates.
- Existing CI enforces formatting, clippy, release build, and tests on Ubuntu.
- The plugin manifest declares the fledge-v1 protocol and release binary.
- No standalone Atlas or provenance-signing workflow exists.
- SpecSync can measure the single Rust source file at 100% file and LOC coverage.

