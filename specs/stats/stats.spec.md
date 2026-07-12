---
module: stats
version: 1
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
2. Host requests use stable correlation identifiers and consume their corresponding responses before continuing.
3. File counts exclude Git metadata, build output, and dependency directories.
4. A missing or malformed optional host value degrades to an empty or zero statistic rather than fabricating data.
5. The current file and LOC totals are persisted after the report is produced for the next-run delta.
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
