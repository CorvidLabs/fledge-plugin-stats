---
id: CHG-0003-cover-remaining-generated-agent-rollout-files
state: accepted
type: migration
base_commit: 49ff88c9b24e97ec0d6c901bfc0dbe19b7a2047f
---

# Cover remaining generated agent rollout files

## Intent

Cover remaining generated agent rollout files

## Affected Canonical Specs

- None

## Acceptance Criteria

- SpecSync recognizes the five existing Claude
- Cursor
- and Gemini rollout files as covered delivery inputs; strict validation retains real 1/1 file and 377/377 LOC coverage; native Fledge verification and the hosted Trust contract gate pass.

## No-spec Rationale

Declare the already-migrated generated Claude, Cursor, and Gemini instruction files as delivery inputs so the pull-request range is fully covered without changing Stats requirements or Rust behavior.
