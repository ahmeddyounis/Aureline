# Nightly-governance report row and waiver-expiry queue fixtures

Worked fixtures for the nightly-governance report row, waiver-expiry
queue item, and recurring-evidence compare contract frozen in
[`/docs/governance/nightly_report_and_waiver_queue_contract.md`](../../../docs/governance/nightly_report_and_waiver_queue_contract.md)
and its boundary schemas
[`/schemas/governance/nightly_report_row.schema.json`](../../../schemas/governance/nightly_report_row.schema.json)
and
[`/schemas/governance/waiver_expiry_item.schema.json`](../../../schemas/governance/waiver_expiry_item.schema.json).

Each fixture renders one concrete row or queue item: the live state,
the compare snapshot, the corpus / profile envelope, the evidence-
freshness envelope, the linked rule or claim refs, the linked waiver-
queue items (or linked nightly-row refs), the typed mitigation note,
and the cluster indicators. The corpus exists so consuming surfaces
(governance dashboard, milestone scorecard, weekly governance review,
release-evidence shiproom packet, support bundle, public claim
manifest, governance packet) can be validated against one shared set
of rows rather than inventing local fixtures.

## Intended usage

- **Run-state grammar conformance.** Every nightly-row fixture
  renders one of the nine run-state tokens; every queue-item fixture
  renders one of the six expiry-proximity tokens. A surface that
  renders a different token, or a generic "needs review" chip, is
  non-conforming.
- **Compare-action conformance.** Every nightly-row fixture pins the
  comparison envelope to the same corpus / profile envelope it was
  captured against. A consuming surface MUST NOT widen "drift inside
  warning band on macOS" into a generic "drift detected."
- **Visual / export parity.** Every fixture carries the parity-floor
  fields the contract requires; consuming surfaces project those
  fields on every channel.
- **Recurring-waiver clustering and repeated-protected-path regression.**
  The cluster fixture and the regression fixture exercise the schema's
  `allOf` rules that prevent chronic drift from rendering as an
  isolated incident.

## Fixtures

- [`fresh_recurring_pass_input_to_paint.yaml`](./fresh_recurring_pass_input_to_paint.yaml)
  — `pass_full_corpus_full_profile` row on `ff.input_to_paint`
  compared cleanly against the prior baseline on the macOS reference-
  laptop envelope. The `no_mitigation_required_passing` mitigation
  class and an empty waiver-queue-item-refs array.
- [`partial_run_subset_compatibility_window.yaml`](./partial_run_subset_compatibility_window.yaml)
  — `partial_run_subset_only` row on the recurring compatibility-
  window report whose command-action-schema slice did not capture this
  cycle. The `partial_profile_result_pending_full_capture` mitigation
  class and a `subset_of_corpus_rows_covered` partial-profile class.
  `render_on_claim_manifest` is false so partial-corpus capture does
  not silently widen claim manifest rows.
- [`waiver_nearing_expiry_vfs_save.yaml`](./waiver_nearing_expiry_vfs_save.yaml)
  — `nearing_expiry_within_seven_days` queue item on the performance-
  council waiver covering `ff.vfs_save_conflict_handling` on the air-
  gapped lab profile. Surfaces render the queue item before the
  release review depends on a renewal or retire decision.
- [`expired_waiver_first_paint.yaml`](./expired_waiver_first_paint.yaml)
  — `expired_past_due` queue item on the performance-council waiver
  covering `ff.first_paint` whose renewal lapsed. The
  `mitigation_blocked_pending_decision` status forces an
  `escalate_to_decision_forum` escalation; the queue item links the
  nightly-row that captured `waiver_expired_failure`.
- [`recurring_waiver_cluster_save_pipeline.yaml`](./recurring_waiver_cluster_save_pipeline.yaml)
  — `recurring_across_quarters` cluster on `ff.vfs_save_conflict_-
  handling` after a third performance-council waiver across distinct
  quarters. The `open_cluster_inspector` action and a non-empty
  `cluster_member_refs` make repeated waivers on the same protected
  path visible as a pattern, not only as isolated queue items.
- [`repeated_protected_path_regression_buffer_operations.yaml`](./repeated_protected_path_regression_buffer_operations.yaml)
  — `blocked_by_failure` row on `ff.buffer_operations` whose
  `repeated_protected_path_within_milestone` regression class names
  a second blocker on the same protected path inside the foundations
  milestone. Chronic drift is visible without an active waiver.
