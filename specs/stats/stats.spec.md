---
module: stats
version: 3
status: active
files:
  - src/main.rs

db_tables: []
depends_on: []
---

# Stats

## Purpose

Provide a zero-runtime-dependency `fledge-v1` plugin that summarizes project size, language mix, Git activity, contributors, recent commits, and changes since the previous run.

## Public API

| Surface | Behavior |
|---------|----------|
| Stats command | Exchange JSON-lines messages with the Fledge host and render a project statistics card. |
| Stored snapshot | Persist file and LOC totals so later runs can report deltas. |

## Invariants

1. Standard output contains only valid `fledge-v1` protocol messages until the final host-rendered output message.
2. Response-bearing host requests use stable correlation identifiers and consume their corresponding responses before continuing; snapshot stores are explicitly fire-and-forget.
3. File counts exclude Git metadata, build output, and dependency directories.
4. A missing or malformed optional host value degrades to an empty or zero statistic rather than fabricating data.
5. The current file and LOC totals are persisted after statistics and deltas are calculated and before final output is emitted.
6. The manifest points `fledge stats` at the release binary and declares the `fledge-v1` protocol.

## Behavioral Examples

```
Given a valid Fledge initialization message and host responses
When the Stats plugin analyzes the project
Then it reports language, file, LOC, Git, contributor, recent-commit, and previous-run delta statistics through the protocol
```

## Error Cases

| Error | When | Behavior |
|-------|------|----------|
| Missing init or response | Standard input closes before a required protocol line | Terminate instead of emitting a misleading partial report. |
| Host command failure | A requested command returns no usable output | Use the documented zero or empty fallback for that statistic. |
| Invalid numeric output | LOC or commit output cannot be parsed | Report zero for that field. |
| Output failure | A protocol line cannot be flushed | Terminate rather than continue with unsynchronized messages. |

## Dependencies

- Rust 2021 standard library
- Fledge host implementing `fledge-v1`
- Common project commands supplied by the host execution surface (`find`, `wc`, and `git` on Unix-like systems)

## Change Log

| Version | Date | Changes |
|---------|------|---------|
| 1 | 2026-07-12 | Document existing Stats plugin behavior for SpecSync 5 adoption. |
| 2 | 2026-07-13 | CHG-0001-adopt-specsync-5-0-1-and-trust-1-0-0-governance-for-the-stats-fledge-plugin: Adopt SpecSync 5.0.1 and Trust 1.0.0 governance for the Stats Fledge plugin |
| 3 | 2026-07-13 | CHG-0002-correct-stats-lifecycle-guidance-and-snapshot-contract-timing: Correct Stats lifecycle guidance and snapshot contract timing |
