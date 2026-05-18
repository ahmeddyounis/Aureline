# Beta Test Triage Trust Packets

This document is the reviewer-facing landing page for beta test triage trust:
watch-state honesty, flaky verdict lineage, snapshot/baseline mutation
governance, mute/quarantine expiry, and release-visible test debt. The runtime
implementation lives at
[`/crates/aureline-runtime/src/testing_triage/`](../../../crates/aureline-runtime/src/testing_triage/).

Machine-readable boundaries:

- [`/schemas/testing/watch_state.schema.json`](../../../schemas/testing/watch_state.schema.json)
- [`/schemas/testing/flaky_verdict.schema.json`](../../../schemas/testing/flaky_verdict.schema.json)
- [`/schemas/testing/test_quarantine_record.schema.json`](../../../schemas/testing/test_quarantine_record.schema.json)
- [`/schemas/testing/test_trust_packet.schema.json`](../../../schemas/testing/test_trust_packet.schema.json)

## Beta promise

- Watch state uses the closed vocabulary `live`, `reduced`, `polling`,
  `unavailable`, and `imported_only`, with exact downgrade reasons preserved
  from the alpha watch controller.
- Flaky verdict packets point at concrete attempt-ledger inputs: attempt refs,
  results, targets, execution contexts, imported-CI state, source drift, and
  predecessor/origin lineage.
- Snapshot or baseline mutations cannot land unless the review has file/change
  previews, an actor, a grouped rollback checkpoint, and release-bearing policy
  hooks where required.
- Mute and quarantine records require owner, reason, scope, expiry, evidence,
  reopen behavior, and release-debt treatment. Expired records reopen into
  release-visible debt until renewed or resolved.
- The test trust packet summarizes watch degradations, imported-only evidence,
  active mutes, quarantined scope, expired reopened records, snapshot
  mutations, and release-blocking rows for every claimed beta test row.

## Cross-references

- Beta test runner - [`test_runner_beta.md`](test_runner_beta.md)
- Beta test quality truth - [`test_quality_truth_beta.md`](test_quality_truth_beta.md)
- Alpha test-attempt model -
  [`/crates/aureline-runtime/src/tests/`](../../../crates/aureline-runtime/src/tests/)

## Fixture coverage

The fixture manifest lives at
[`/fixtures/testing/m3/watch_flaky_snapshot_quarantine/manifest.json`](../../../fixtures/testing/m3/watch_flaky_snapshot_quarantine/manifest.json).
It names the watch degradation, imported-only, snapshot mutation, and
quarantine expiry scenarios covered by the runtime integration tests.
