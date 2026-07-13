## MODIFIED

### REQUIREMENT REQ-stats-001

The plugin SHALL obtain project context and derived statistics through the `fledge-v1` JSON-lines protocol.

Acceptance Criteria

- Source inspection confirms response-bearing requests and responses are newline-delimited protocol messages with stable correlation identifiers.
- Snapshot store messages remain valid fire-and-forget protocol messages without claiming a response correlation identifier.
- The release binary builds successfully.

### REQUIREMENT REQ-stats-004

The plugin SHALL load the previous file and LOC totals, report their deltas, and persist the current totals for the next run.

Acceptance Criteria

- Source inspection confirms previous totals are loaded before delta calculation.
- Current totals are stored after statistics and deltas are calculated and before final output is emitted.
