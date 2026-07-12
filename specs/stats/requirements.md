---
spec: stats.spec.md
---

## User Stories

- As a developer, I want a concise project-health snapshot without installing plugin runtime dependencies.
- As a returning user, I want file and LOC deltas from the previous run.

## Acceptance Criteria

### REQ-stats-001

The plugin SHALL obtain project context and derived statistics through the `fledge-v1` JSON-lines protocol.

### REQ-stats-002

File and LOC calculations SHALL exclude repository metadata, dependency caches, and build output.

### REQ-stats-003

The report SHALL include language, file, LOC, Git activity, contributor, and recent-commit information when available.

### REQ-stats-004

The plugin SHALL load the previous file and LOC totals, report their deltas, and persist the current totals for the next run.

### REQ-stats-005

Unavailable or unparsable optional statistics SHALL degrade to documented empty or zero values without producing invalid protocol output.

## Constraints

- The plugin intentionally uses the Rust standard library and simple JSON extraction rather than a third-party serializer.
- Filesystem and Git inspection execute through commands authorized by the Fledge host.

## Out of Scope

- Defining a new plugin protocol or acting as a general-purpose code metrics engine.
- Guaranteeing identical external command availability on every host platform.
