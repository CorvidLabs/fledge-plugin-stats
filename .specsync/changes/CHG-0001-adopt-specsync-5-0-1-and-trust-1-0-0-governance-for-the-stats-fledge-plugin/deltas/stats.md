## ADDED

### REQUIREMENT REQ-stats-001

The plugin SHALL obtain project context and derived statistics through the `fledge-v1` JSON-lines protocol.

Acceptance Criteria
- Source inspection confirms every request and response is a newline-delimited protocol message with a stable correlation identifier; the release binary builds successfully.

### REQUIREMENT REQ-stats-002

File and LOC calculations SHALL exclude repository metadata, dependency caches, and build output.

Acceptance Criteria
- Source inspection confirms the committed exclusion expressions cover Git metadata, target output, and dependency directories.

### REQUIREMENT REQ-stats-003

The report SHALL include language, file, LOC, Git activity, contributor, and recent-commit information when available.

Acceptance Criteria
- Source inspection confirms each available host result is mapped into the final report while unavailable optional values use the documented fallback.

### REQUIREMENT REQ-stats-004

The plugin SHALL load the previous file and LOC totals, report their deltas, and persist the current totals for the next run.

Acceptance Criteria
- Source inspection confirms the previous snapshot is loaded before delta calculation and the current totals are stored after report assembly.

### REQUIREMENT REQ-stats-005

Unavailable or unparsable optional statistics SHALL degrade to documented empty or zero values without producing invalid protocol output.

Acceptance Criteria
- Source inspection confirms optional parsing paths use explicit empty or zero fallbacks and all output is emitted through the protocol encoder.
