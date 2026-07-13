---
spec: stats.spec.md
---

## Context

Stats is a reference `fledge-v1` plugin that demonstrates execution, metadata, storage, progress, logging, and output while providing a useful project overview.

## Related Modules

- Fledge owns host execution, rendering, metadata production, and persistent storage.

## Design Decisions

- Keep the binary dependency-free so it remains a minimal protocol reference.
- Ask the host to execute inspection commands instead of giving the plugin direct process-control authority.
- Treat missing optional measurements as zero or empty values so one unavailable statistic does not suppress the report.
