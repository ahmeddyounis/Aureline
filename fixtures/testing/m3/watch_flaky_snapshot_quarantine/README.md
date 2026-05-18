# Watch, Flaky, Snapshot, and Quarantine Trust Fixtures

This directory describes the beta test-triage scenarios exercised by
[`/crates/aureline-runtime/tests/testing_triage_beta.rs`](../../../../crates/aureline-runtime/tests/testing_triage_beta.rs).
The runtime test generates the concrete packets from the canonical pytest
fixture workspace so the checked contract stays tied to the Rust types and the
schema boundaries.

Files:

- `manifest.json` - scenario manifest naming the watch degradation,
  flaky-attempt lineage, snapshot mutation review, expired quarantine, and
  imported-only evidence cases.

Schema boundaries:

- [`/schemas/testing/watch_state.schema.json`](../../../../schemas/testing/watch_state.schema.json)
- [`/schemas/testing/flaky_verdict.schema.json`](../../../../schemas/testing/flaky_verdict.schema.json)
- [`/schemas/testing/test_quarantine_record.schema.json`](../../../../schemas/testing/test_quarantine_record.schema.json)
- [`/schemas/testing/test_trust_packet.schema.json`](../../../../schemas/testing/test_trust_packet.schema.json)
