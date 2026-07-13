---
change: CHG-0003-cover-remaining-generated-agent-rollout-files
artifact: testing
---

# Testing

- Run `fledge lanes run verify` and truthfully retain Cargo's zero-test result.
- Run `specsync check --strict --require-coverage 100 --force` and require 1/1 file plus 377/377 LOC coverage.
- Confirm hosted Trust no longer reports the five generated integration paths as uncovered.
