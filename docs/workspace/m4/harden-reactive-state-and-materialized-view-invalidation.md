# Reactive-state lineage — contract

This document describes the reactive-state lineage record: the workspace's
governed, export-safe projection that hardens reactive-state and
materialized-view invalidation with truthful stale-view downgrade
behavior.

The record is the single artifact every consuming surface (workspace
reactive-state status, drift inspector, resync review sheet, support
export, Help/About, headless CLI) ingests instead of cloning status text.

## Input

The projection ingests a live
[`ReactiveStateInputs`](../../../crates/aureline-workspace/src/reactive_state_lineage/mod.rs)
envelope verbatim. The envelope carries one
[`MaterializedViewObservation`](../../../crates/aureline-workspace/src/reactive_state_lineage/mod.rs)
per reactive view the workspace tracks, with the authority epoch, the
subscriber epochs per required consumer surface, the support-export
posture, the declared parity state, the declared downgrade label, and
the open-gap rows.

For determinism and replay, the projection accepts the same envelope
shape fixtures and the headless emitter consume.

## What the record proves

The contract claims the stable line is anchored on, specialised to
reactive state:

- **View-class coverage truth.** Each materialized view declares one
  closed view class (`ephemeral_projection`, `durable_local_materialization`,
  `exportable_snapshot`, `managed_replicated_view`), and the corpus
  seeds at least one row per class so the export, support, and
  replication paths are observable. Every view declares one entry per
  required consumer surface (`shell`, `search`, `graph`, `ai`, `review`,
  `support`). No subscriber's observed epoch exceeds the authority
  epoch.
- **Stale-view downgrade truth.** Every non-aligned parity state carries
  a non-`none` downgrade label drawn from the closed vocabulary;
  aligned views carry `none`. Downgraded views record at least one
  open-gap row; aligned views record none.
- **Epoch-parity honesty.** The projection re-derives parity from
  subscriber observations and surfaces both `declared_parity_state` and
  `derived_parity_state`. A view declaring `aligned` while a subscriber
  lags is narrowed below Stable rather than papered over.
- **Support-export honesty.** Each view's support-export projection
  preserves epoch state (`view_class`, `authority_label`,
  `authority_epoch`, `subscriber_epochs`), excludes raw private
  material and ambient authority, and preserves user-authored files.
  `exportable_snapshot` and `managed_replicated_view` views must
  declare a non-`local_only` posture so support bundles can preserve
  the epoch state.
- **No-rerun honesty under stale views.** A controlled set of
  pre-destructive inspection / repair hooks (`inspect_drift`,
  `compare_epochs`, `resync_review`, `rollback_checkpoint`, `export`,
  `repair`) is reachable so any destructive resync is reviewable
  before it fires.

In addition the record carries the producer ref, the schema version,
the capture timestamp, and an integrity hash so import / replay surfaces
can pin the source producer before applying.

## Closed vocabularies

| Field                         | Tokens                                                                                                                                                                                              |
| ----------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `materialized_view_class`     | `ephemeral_projection`, `durable_local_materialization`, `exportable_snapshot`, `managed_replicated_view`                                                                                          |
| `authority_label`             | `workspace_vfs`, `buffer_editor`, `derived_knowledge`, `execution`, `policy_entitlement`, `provider_overlay`                                                                                       |
| `consumer_surface_kind`       | `shell`, `search`, `graph`, `ai`, `review`, `support`                                                                                                                                              |
| `subscriber_freshness`        | `authoritative`, `cached`, `stale`, `imported`, `warming`, `unavailable`                                                                                                                            |
| `invalidation_cause_class`    | `authority_write`, `derived_recompute`, `policy_change`, `provider_overlay_change`, `external_change`, `imported_bundle_swap`, `resync_required`                                                    |
| `epoch_parity_state`          | `aligned`, `drift_detected`, `awaiting_resync`, `terminal_unavailable`                                                                                                                              |
| `reactive_downgrade_label`    | `none`, `red_blocks_stable_row`, `yellow_surface_partial`, `yellow_authority_skew`, `degraded_to_authority_only`, `stale_corpus_blocks_release_candidate`                                          |
| `open_gap_class`              | `none`, `subscriber_pending`, `replication_pending`, `support_export_pending`, `drift_recovery_manual`                                                                                              |
| `support_export_posture`      | `local_only`, `metadata_safe_export`, `held_record`                                                                                                                                                |
| `inspection_hook_class`       | `inspect_drift`, `compare_epochs`, `resync_review`, `rollback_checkpoint`, `export`, `repair`                                                                                                       |

## Narrow reasons

When a claim cannot be proven on the captured posture the record auto-
narrows below Stable with a named reason.

| Narrow reason                              | Fires when                                                                                                          |
| ------------------------------------------ | ------------------------------------------------------------------------------------------------------------------- |
| `corpus_empty`                             | The envelope carried no materialized-view observations                                                              |
| `required_view_class_missing`              | The corpus omitted at least one of the four required view classes                                                   |
| `required_consumer_surface_missing`        | A view omitted an entry for at least one required consumer surface                                                  |
| `subscriber_epoch_exceeds_authority`       | A subscriber's observed epoch exceeded the authority epoch                                                          |
| `aligned_parity_not_proven`                | A view declared `aligned` while a subscriber lagged or was non-authoritative                                        |
| `drift_without_epoch_lag`                  | A view declared `drift_detected` while no subscriber lagged the authority epoch                                     |
| `awaiting_resync_without_signal`           | A view declared `awaiting_resync` while no subscriber carried a resync-required / stale / warming signal            |
| `terminal_unavailable_without_signal`      | A view declared `terminal_unavailable` while no subscriber was unavailable                                          |
| `non_aligned_missing_downgrade`            | A non-aligned view declared the `none` downgrade label                                                              |
| `aligned_carries_downgrade`                | An aligned view declared a non-`none` downgrade label                                                               |
| `downgraded_without_open_gap`              | A downgraded view recorded no open-gap row                                                                          |
| `aligned_carries_open_gap`                 | An aligned view recorded a non-`none` open-gap row                                                                  |
| `support_export_epoch_fields_dropped`      | A view's support-export projection dropped one of the required epoch fields                                         |
| `support_export_posture_unsafe`            | An exportable / managed-replicated view declared a `local_only` support-export posture                              |
| `support_export_redaction_unsafe`          | A view declared `raw_private_material_excluded = false`, `ambient_authority_excluded = false`, or did not preserve user-authored files |
| `inspection_hook_unavailable`              | A required pre-destructive inspection / repair hook was unavailable                                                 |
| `producer_attribution_incomplete`          | Producer attribution fields were empty (producer ref / captured-at)                                                 |
| `lineage_export_unsafe`                    | Workspace ref or corpus ref was empty (would break support export)                                                  |

## Inspection hooks

A destructive resync, export, or repair never fires without an
inspection hook the user can reach first.

| Hook class            | Default action id                       | Purpose                                                                                          |
| --------------------- | --------------------------------------- | ------------------------------------------------------------------------------------------------ |
| `inspect_drift`       | `reactive_state.inspect_drift`         | Opens the drift inspector with each subscriber's observed epoch, freshness, and last cause       |
| `compare_epochs`      | `reactive_state.compare_epochs`        | Produces a reviewable diff between the authority epoch and each subscriber's observed epoch      |
| `resync_review`       | `reactive_state.resync_review`         | Opens the resync review sheet so any destructive resync can be inspected before it fires        |
| `rollback_checkpoint` | `reactive_state.rollback_checkpoint`   | Captures a one-step rollback checkpoint before the resync                                        |
| `export`              | `reactive_state.export`                | Exports the lineage record (support-safe, no raw payload bytes)                                  |
| `repair`              | `reactive_state.repair`                | Opens the repair sheet for a stuck or unavailable view; surfaces the manual remediation steps    |

## Replay gate

Every fixture under
[`/fixtures/workspace/m4/reactive_state_lineage/`](../../../fixtures/workspace/m4/reactive_state_lineage/)
carries the posture inputs and the expected projected record. The replay
gate at
[`/crates/aureline-workspace/tests/reactive_state_lineage_replay.rs`](../../../crates/aureline-workspace/tests/reactive_state_lineage_replay.rs)
re-projects each input and asserts the result equals the checked-in
`expected`, so the projection cannot drift from the canonical record
without failing CI. The gate also asserts each fixture is support-export
safe and that the corpus covers every controlled parity state plus a
narrowed-below-Stable posture.
