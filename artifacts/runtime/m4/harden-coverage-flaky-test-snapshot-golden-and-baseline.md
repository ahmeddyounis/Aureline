# Harden coverage, flaky-test, snapshot/golden, and baseline-truth surfaces — M4 reviewer artifact

This artifact summarizes the checked-in stable coverage / flaky-test /
snapshot-golden / baseline-truth packet for release reviewers. The
canonical packet is
[`harden_coverage_flaky_test_snapshot_golden_and_baseline_truth_packet.json`](./harden_coverage_flaky_test_snapshot_golden_and_baseline_truth_packet.json);
the reviewer-facing contract is at
[`docs/runtime/m4/harden-coverage-flaky-test-snapshot-golden-and-baseline.md`](../../../docs/runtime/m4/harden-coverage-flaky-test-snapshot-golden-and-baseline.md).

## What the packet promises

For each of the four coverage-quality lanes (`coverage_lane`,
`flaky_test_lane`, `snapshot_golden_lane`, `baseline_truth_lane`) the
packet certifies:

- One `coverage_flaky_snapshot_baseline_quality` row at
  `launch_stable` with `release_evidence_review` evidence and
  `auto_block_on_missing_evidence` automation.
- Four `wedge_admission` rows covering every wedge:
  `stability_verdict_separation`, `quarantine_mute_renewal_truth`,
  `ai_candidate_source_attribution`, `coverage_impact_truth`. Each
  row binds `auto_narrow_on_wedge_admission_gap` automation against
  `conformance_suite_evidence`.
- Six `stability_verdict_admission` rows covering every verdict:
  `stable`, `flaky`, `failing`, `quarantined`, `muted`, `unknown`.
  Each row binds `auto_narrow_on_stability_verdict_gap` automation
  against `automated_functional_evidence`.
- Four `quarantine_mute_state_admission` rows covering every state:
  `active`, `expiring_soon`, `expired_pending_renewal`, `removed`.
  Each row binds `auto_narrow_on_quarantine_mute_state_gap`
  automation against `automated_functional_evidence`.
- Four `test_source_admission` rows covering every source class:
  `human_authored`, `candidate_ai_test`, `automated_baseline`,
  `imported_ci_evidence`. Candidate / automated / imported rows
  attest `attests_candidate_lineage_bound` so the candidate is
  pinned to the same session/attempt + review-checkpoint lineage
  that the `candidate_lineage_admission` rows cover. Each row binds
  `auto_narrow_on_test_source_gap` automation against
  `automated_functional_evidence`.
- Four `coverage_impact_admission` rows covering every impact class:
  `measured`, `estimated`, `stale`, `not_comparable`. Each row binds
  `auto_narrow_on_coverage_impact_gap` automation against
  `automated_functional_evidence`.
- Three `candidate_lineage_admission` rows covering every lineage
  class: `session_attempt_bound`, `review_checkpoint_bound`,
  `imported_ci_bound`. Each row binds
  `auto_narrow_on_candidate_lineage_gap` automation against
  `automated_functional_evidence`.
- Five `consumer_surface_binding` rows covering every consumer
  surface: `coverage_surface`, `flaky_triage_surface`,
  `snapshot_golden_surface`, `baseline_surface`,
  `release_packet_surface`. Each row attests the vocabularies it is
  required to preserve (stability-verdict, quarantine-mute-state,
  test-source, coverage-impact, candidate-lineage as applicable per
  the contract) and binds `auto_narrow_on_consumer_surface_gap`
  automation against `conformance_suite_evidence`.
- One `lineage_admission` row carrying an
  `execution_context_id_binding` (e.g.
  `exec:m4:coverage:coverage_quality_lineage`) so coverage,
  flaky-triage, snapshot/golden, baseline, release/support packet,
  evidence-export, and AI tool surfaces all cite one stable lineage
  object.

The packet also carries twelve consumer projections — one each for
the coverage, flaky-triage, snapshot/golden, baseline, release packet,
AI tool, CLI/headless, evidence export, support export, release proof
index, Help/About, and conformance dashboard surfaces — preserving
every closed vocabulary verbatim.

## Why this matters

Without one shared truth packet, every surface invents its own
quarantine/mute state, mixes AI-generated and human-authored tests
in the same coverage narrative, or quietly promotes a single passing
sandbox run into trusted coverage proof. Stability verdicts collapse
into a single pass/fail bit and muted/quarantined tests vanish from
release packets as hidden debt.

The packet refuses to certify any of those failure modes. Reviewers
can read one snapshot and trust that:

- the stability verdict and the quarantine/mute state stay separately
  observable;
- muted and quarantined tests remain visible, countable, filterable,
  and exportable with explicit `active` / `expiring_soon` /
  `expired_pending_renewal` / `removed` semantics;
- AI-generated tests, automated baseline mutations, ordinary
  human-authored tests, and imported CI evidence stay distinguishable
  in release and support packets;
- coverage impact derived from AI / sandbox-run candidate tests stays
  explicitly `measured`, `estimated`, `stale`, or `not_comparable`.

## How to regenerate

Run
[`tools/regenerate_harden_coverage_flaky_test_snapshot_golden_and_baseline_truth_packet.py`](../../../tools/regenerate_harden_coverage_flaky_test_snapshot_golden_and_baseline_truth_packet.py)
to regenerate this artifact and the fixture corpus from a single
seed. The Rust contract is the source of truth for the validator and
support-export surface; the generator mirrors the Rust unit-test
sample input.

## See also

- Boundary schema:
  [`schemas/runtime/harden_coverage_flaky_test_snapshot_golden_and_baseline_truth.schema.json`](../../../schemas/runtime/harden_coverage_flaky_test_snapshot_golden_and_baseline_truth.schema.json)
- Rust contract:
  [`crates/aureline-runtime/src/harden_coverage_flaky_test_snapshot_golden_and_baseline/mod.rs`](../../../crates/aureline-runtime/src/harden_coverage_flaky_test_snapshot_golden_and_baseline/mod.rs)
- Fixture corpus:
  [`fixtures/runtime/m4/harden_coverage_flaky_test_snapshot_golden_and_baseline/`](../../../fixtures/runtime/m4/harden_coverage_flaky_test_snapshot_golden_and_baseline/)
