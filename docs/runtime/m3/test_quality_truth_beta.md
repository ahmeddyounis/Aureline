# Beta Test Quality Truth

This document is the reviewer-facing landing page for the beta promote of the
test-quality truth lane: coverage, flaky verdict, snapshot review, and
per-case baseline. The machine-readable boundary lives at
[`/schemas/testing/test_quality_truth_beta.schema.json`](../../../schemas/testing/test_quality_truth_beta.schema.json).
The runtime implementation lives at
[`/crates/aureline-runtime/src/testing_quality/`](../../../crates/aureline-runtime/src/testing_quality/).

The beta promise:

- every claimed beta test row carries **four typed quality packets** — one
  coverage packet, one flaky packet, one snapshot packet, one baseline
  packet — published as
  [`TestQualityProjection`](../../../crates/aureline-runtime/src/testing_quality/mod.rs);
- each packet quotes **exact provenance** (producing test attempt, producing
  execution attempt, backing artifact identity ref) and a typed
  [`TestQualityFreshness`](../../../crates/aureline-runtime/src/testing_quality/mod.rs)
  label derived from the alpha test-attempt tokens; it does not infer state
  from log text;
- the
  [`TestQualityRowTruth`](../../../crates/aureline-runtime/src/testing_quality/mod.rs)
  roll-up downgrades the row to `limited_imported_or_partial` or
  `retest_pending_no_current_packet` when any packet is partial, imported, or
  missing, so the product never implies stable support for a row that does
  not have it;
- the
  [`TestQualityBetaSupportExport`](../../../crates/aureline-runtime/src/testing_quality/mod.rs)
  packet is referenced from
  [`TestRunnerBetaSupportExport`](../../../crates/aureline-runtime/src/testing/mod.rs)
  via `quality_projection_ref` and `quality_support_export_ref`, so support
  reviewers and review flows can point to the same per-row quality packet
  the in-product flow renders.

## Coverage manifest

| Framework | Claimed quality kinds | Backing artifact kinds |
| --- | --- | --- |
| `pytest` | `coverage`, `flaky`, `snapshot`, `baseline` | `coverage_report`, `snapshot_diff`, `run_report`, `raw_event_envelope` |

Adding a framework, a quality kind, or a backing artifact kind is a
vocabulary change that MUST update this doc, the canonical schema, and the
checked-in coverage manifest fixture together.

## Freshness vocabulary

| Token | Meaning |
| --- | --- |
| `current_local_evidence` | A current local attempt produced the packet. |
| `authoritative_imported_read_only` | Authoritative imported provider evidence; read-only locally. |
| `stale_requires_retest` | Evidence is stale or outside its comparability window; a rerun is required before the packet can claim current support. |
| `no_evidence_retest_pending` | No packet has been produced yet for this row. |
| `unknown_requires_review` | Freshness cannot be classified; mutating actions fail closed. |

The freshness label is derived from the producing attempt's
`coverage_merge_class`, `flaky_verdict_state`, `snapshot_review_state`,
`source_drift_state`, and `imported_ci_projection_class`. It is never
inferred from log text or from the presence of an artifact ref alone.

## Support-class roll-up

| Token | Meaning |
| --- | --- |
| `out_of_scope` | Quality dimension is not requested on this row. |
| `stable_supported` | A current packet exists and binds the canonical identity; the row may claim stable support. |
| `limited_imported_or_partial` | A packet exists but is partial, imported-only, or read-only. |
| `retest_pending_no_current_packet` | No current packet exists; the row downgrades to retest-pending. |
| `unknown_requires_review` | The packet cannot be classified; the row fails closed. |

`TestQualityRowTruth.row_support_class` is the weakest support class across
the row's four packets (ignoring packets the row marks `out_of_scope`). A
beta row may only render the "stable support" affordance when every claimed
packet is `stable_supported`.

## Identity parity with the test runner beta

Every packet's `identity` carries the same `canonical_test_item_ref`,
`selector_ref`, and `test_session_ref` that the test-tree row, the inline
editor row, and the rerun-last command already quote. A support reviewer can
therefore join one canonical identity to:

- the
  [`test_runner_beta_tree_row_record`](../../../schemas/testing/test_runner_beta.schema.json)
  for the case;
- the
  [`test_runner_beta_inline_row_record`](../../../schemas/testing/test_runner_beta.schema.json)
  for the case;
- the
  [`test_runner_beta_rerun_parity_record`](../../../schemas/testing/test_runner_beta.schema.json)
  bound to the case;
- the per-row coverage / flaky / snapshot / baseline packets in this lane.

When the in-product flow renders a support-export for a beta test row, the
test-runner beta support export attaches `quality_projection_ref` and
`quality_support_export_ref`, so the reviewer reads one packet that points
to both projections without re-deriving truth from log text.

## Cross-references

- Beta test runner — [`test_runner_beta.md`](test_runner_beta.md)
- Alpha test-attempt model —
  [`/crates/aureline-runtime/src/tests/`](../../../crates/aureline-runtime/src/tests/)
  and
  [`/schemas/testing/test_attempt.schema.json`](../../../schemas/testing/test_attempt.schema.json)
- Beta task-event model — [`task_event_model_beta.md`](task_event_model_beta.md)

## Out of scope for this beta

- Frameworks beyond pytest (jest / vitest / cargo-test do not yet ship
  packets in the coverage manifest).
- Cross-workspace coverage merge.
- Time-series flaky trends; the flaky packet reports the per-session verdict
  only.
- Snapshot accept/reject workflows; the snapshot packet labels the review
  state and links the diff artifact identity ref, but does not own the
  acceptance ceremony.
- AI-generated baseline reasoning beyond the alpha `ai_test_generation`
  gate tokens.
