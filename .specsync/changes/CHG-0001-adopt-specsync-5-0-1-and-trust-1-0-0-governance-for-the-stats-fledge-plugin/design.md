---
change: CHG-0001-adopt-specsync-5-0-1-and-trust-1-0-0-governance-for-the-stats-fledge-plugin
artifact: design
---

# Design

Keep the existing CI workflow and add an independently required trust job. The committed policy uses the standard profile, blocking risk, soft provenance, 100% contract coverage, and disabled Trust-managed Atlas. A native Fledge lane composes formatting, clippy, tests, release build, and manifest checks without recursively invoking SpecSync.

