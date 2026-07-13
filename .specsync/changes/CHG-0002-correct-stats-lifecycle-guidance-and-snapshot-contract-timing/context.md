---
change: CHG-0002-correct-stats-lifecycle-guidance-and-snapshot-contract-timing
artifact: context
---

# Context

Review of the canonical Stats contract against `src/main.rs` found two timing and correlation overstatements: snapshot stores are fire-and-forget messages without correlation identifiers, and current totals are stored before final output rather than after the report. The installed create-spec prompts also split natural-language input too early, Gemini create-change referenced an unavailable variable, and SDD verification duplicated native commands while omitting manifest validation.

These corrections align specifications and governance with the existing dependency-free Rust implementation. No Rust code changes are included.
