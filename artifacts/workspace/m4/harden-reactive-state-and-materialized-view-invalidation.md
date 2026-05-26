# Harden Reactive-State and Materialized-View Invalidation With Truthful Stale-View Downgrade Behavior — proof packet

Reviewer-facing proof packet for the reactive-state lane: every
materialized view that the workspace, editor, search, graph, AI, review,
and support surfaces consume is bound to one closed parity state and one
controlled downgrade label, with subscriber epochs re-derived from
observations rather than trusted prose. This packet is the stable-line
anchor for this lane; dashboards, docs, Help/About surfaces, and support
exports should ingest the typed sources below rather than cloning this
packet's text.

## Canonical machine sources

- Lineage projection and contract types:
  [`/crates/aureline-workspace/src/reactive_state_lineage/`](../../../crates/aureline-workspace/src/reactive_state_lineage/)
- Schema:
  [`/schemas/workspace/reactive_state_lineage.schema.json`](../../../schemas/workspace/reactive_state_lineage.schema.json)
- Headless emitter / CLI:
  [`/crates/aureline-workspace/src/bin/aureline_reactive_state_lineage.rs`](../../../crates/aureline-workspace/src/bin/aureline_reactive_state_lineage.rs)
- Fixtures:
  [`/fixtures/workspace/m4/reactive_state_lineage/`](../../../fixtures/workspace/m4/reactive_state_lineage/)
- Replay gate:
  [`/crates/aureline-workspace/tests/reactive_state_lineage_replay.rs`](../../../crates/aureline-workspace/tests/reactive_state_lineage_replay.rs)
- Companion contract doc:
  [`/docs/workspace/m4/harden-reactive-state-and-materialized-view-invalidation.md`](../../../docs/workspace/m4/harden-reactive-state-and-materialized-view-invalidation.md)
- Typed consumer: `aureline_workspace::project_reactive_state_lineage`

## What this packet proves

1. **View-class coverage truth.** Each record carries a
   [`view_class_coverage`](../../../schemas/workspace/reactive_state_lineage.schema.json)
   row per materialized view declaring one closed
   `materialized_view_class` (`ephemeral_projection`,
   `durable_local_materialization`, `exportable_snapshot`,
   `managed_replicated_view`). A corpus missing any required class
   narrows below Stable with `required_view_class_missing`. A view that
   omits a required consumer surface (`shell`, `search`, `graph`, `ai`,
   `review`, `support`) narrows with `required_consumer_surface_missing`.
   A subscriber whose observed epoch exceeds the authority epoch narrows
   with `subscriber_epoch_exceeds_authority`. Worked examples:
   [`aligned_stable.json`](../../../fixtures/workspace/m4/reactive_state_lineage/aligned_stable.json).

2. **Stale-view downgrade truth.** Every non-aligned parity state carries
   a non-`none` `reactive_downgrade_label` drawn from the closed
   vocabulary (`red_blocks_stable_row`, `yellow_surface_partial`,
   `yellow_authority_skew`, `degraded_to_authority_only`,
   `stale_corpus_blocks_release_candidate`); aligned views carry `none`.
   Downgraded views must record at least one `open_gap_class` other than
   `none`. Aligned views must not record any open-gap row. Worked
   examples:
   [`drift_detected_stable.json`](../../../fixtures/workspace/m4/reactive_state_lineage/drift_detected_stable.json),
   [`awaiting_resync_stable.json`](../../../fixtures/workspace/m4/reactive_state_lineage/awaiting_resync_stable.json),
   [`terminal_unavailable_stable.json`](../../../fixtures/workspace/m4/reactive_state_lineage/terminal_unavailable_stable.json).
   Mismatches narrow with `non_aligned_missing_downgrade`,
   `aligned_carries_downgrade`, `downgraded_without_open_gap`, or
   `aligned_carries_open_gap`.

3. **Epoch-parity honesty.** The projection re-derives parity from
   subscriber observations rather than trusting the declared field. The
   record exposes both `declared_parity_state` and `derived_parity_state`
   per view plus a `parity_state_matches` flag. A view declaring
   `aligned` while a subscriber lags narrows with
   `aligned_parity_not_proven`; `drift_detected` without epoch lag narrows
   with `drift_without_epoch_lag`; `awaiting_resync` without a
   resync-required / stale / warming signal narrows with
   `awaiting_resync_without_signal`; `terminal_unavailable` without an
   unavailable subscriber narrows with
   `terminal_unavailable_without_signal`.

4. **Support-export honesty.** Each view's support-export projection must
   preserve epoch state (`view_class`, `authority_label`,
   `authority_epoch`, `subscriber_epochs`), redact raw private material,
   exclude ambient authority, and preserve user-authored files. Dropping
   an epoch field narrows with `support_export_epoch_fields_dropped`;
   raising raw material narrows with `support_export_redaction_unsafe`.
   `exportable_snapshot` and `managed_replicated_view` views must declare
   a non-`local_only` posture so support bundles can preserve the epoch
   state; otherwise the record narrows with
   `support_export_posture_unsafe`.

5. **Inspection precedes destructive resync.** A controlled inspection /
   repair hook table is required to be available before any destructive
   resync. The required classes are `inspect_drift`, `compare_epochs`,
   `resync_review`, `rollback_checkpoint`, `export`, and `repair`. A
   missing hook narrows with `inspection_hook_unavailable`. Worked
   example:
   [`missing_compare_hook_narrowed.json`](../../../fixtures/workspace/m4/reactive_state_lineage/missing_compare_hook_narrowed.json).

6. **Producer attribution is pinnable for replay.** Each record carries
   the producer ref, the schema version, the capture timestamp, and an
   integrity hash derived from the input view identities so replay and
   support pipelines can pin the source before applying. Incomplete
   attribution narrows with `producer_attribution_incomplete`.

7. **Lineage and export stay honest.** Every record sets
   `raw_payload_excluded = true` and carries only opaque refs to the
   source workspace, corpus, and producer. An empty workspace or corpus
   ref narrows with `lineage_export_unsafe`.

8. **The record is replay-gated.** The replay gate re-projects each
   fixture and asserts it equals the checked-in `expected`, so the
   projection cannot drift without failing CI.

## Fixture corpus

| Fixture                            | Posture                                              | Parity state(s) observed                                  | Qualification           | Proves                                                |
| ---------------------------------- | ---------------------------------------------------- | --------------------------------------------------------- | ----------------------- | ----------------------------------------------------- |
| `aligned_stable`                   | All four view classes aligned at the same epoch       | `aligned` (×4)                                            | `stable`                | Aligned corpus, all pillars proven                    |
| `drift_detected_stable`            | Durable-local view lagging on the shell subscriber    | `aligned`, `drift_detected`                               | `stable`                | Drift surfaces a yellow surface-partial downgrade     |
| `awaiting_resync_stable`           | Exportable snapshot awaiting resync on support        | `aligned`, `awaiting_resync`                              | `stable`                | Resync waiting surfaces a yellow authority-skew label |
| `terminal_unavailable_stable`      | Managed replicated view unavailable on review surface | `aligned`, `terminal_unavailable`                         | `stable`                | Terminal-unavailable degrades to authority-only       |
| `missing_compare_hook_narrowed`    | `compare_epochs` hook unavailable                     | `aligned` (×4)                                            | `narrowed_below_stable` | Destructive resync with no compare hook narrows       |

## How to verify

```sh
# Unit + replay gate for the reactive-state lineage projection.
cargo test -p aureline-workspace --lib reactive_state_lineage
cargo test -p aureline-workspace --test reactive_state_lineage_replay

# Headless emitter (JSON or --lines projection).
cargo run -p aureline-workspace --bin aureline_reactive_state_lineage -- --lines \
  fixtures/workspace/m4/reactive_state_lineage/drift_detected_stable.json
```

## Stable-line registration

This lane's truth is the checked-in record, schema, fixtures, and replay
gate above. The lineage record self-describes its stable qualification:
surfaces that cannot prove the contract carry
`stable_qualification.level = narrowed_below_stable` with a named reason,
so they never inherit an adjacent green row. No public scope is widened
from this row.
