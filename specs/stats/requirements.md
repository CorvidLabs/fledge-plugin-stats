---
spec: stats.spec.md
---

## User Stories

- As a developer, I want a concise project-health snapshot without installing plugin runtime dependencies.
- As a returning user, I want file and LOC deltas from the previous run.

## Acceptance Criteria

## Constraints

- The plugin intentionally uses the Rust standard library and simple JSON extraction rather than a third-party serializer.
- Filesystem and Git inspection execute through commands authorized by the Fledge host.

## Out of Scope

- Defining a new plugin protocol or acting as a general-purpose code metrics engine.
- Guaranteeing identical external command availability on every host platform.
