---
change: CHG-0002-correct-stats-lifecycle-guidance-and-snapshot-contract-timing
artifact: design
---

# Design

Narrow response-correlation language to response-bearing protocol operations and document snapshot storage as fire-and-forget. State the implemented ordering precisely: load prior totals, calculate current statistics and deltas, store current totals, then emit final output.

Use the same full-input create-spec guidance for Claude, Cursor, and Gemini, make Gemini create-change shell-escape displayed raw arguments once, and route SDD verification through the committed Fledge lane so formatting, clippy, zero-test disclosure, release build, and manifest validation remain one authority.
