# M5 Archived-Object Expiry / Removal State and Metadata Fallback

The archived-object expiry / removal state lane (row **M05-1252**, batch **B149**) keeps a preserved object
**honest after the fact**: once its retention window closes, its content is expired or removed, or its live
target disappears, the archived object transitions into an explicit lifecycle state that still renders its
capture time, provenance, and the exact expiry / removal explanation — never a blank pane, never a generic dead
link, never a live-looking affordance. It is the implement lane over the five non-live-evidence object classes
frozen in the [historical-reference matrix](../../artifacts/program/m5-historical-reference-matrix.md):
retirement snapshot, captured support / export evidence, archived runbook packet, imported / offline route
evidence, and review / incident snapshot.

Where the archive-viewer lane proves how a preserved snapshot is *shown* as non-live, the compare-flow lane
proves how it is *compared* against its live target, and the live-target-handoff lane makes reopening a current
object a *validated pivot*, this lane proves how a preserved object stays *truthful once its bytes or live
target change*.

## Canonical source

- Boundary schema: `schemas/program/m5-archived-object-expiry-removal-state-and-metadata-fallback.schema.json`
- Reused domain schemas (matrix-minted): `schemas/program/m5-historical-snapshot-descriptor.schema.json`,
  `schemas/program/m5-live-target-handoff.schema.json`,
  `schemas/program/m5-imported-offline-evidence-state.schema.json`
- Support export: `artifacts/support/m5-archived-evidence-state/support_export.json`
- Matrix CSV: `artifacts/support/m5-archived-evidence-state/matrix.csv`
- Markdown summary: `artifacts/support/m5-archived-evidence-state/summary.md`
- Narrowed fixtures: `fixtures/recovery/m5-archived-evidence-state/`

Everything is minted from the seed builder in
`crates/aureline-ui/src/m5_archived_object_expiry_removal_state_and_metadata_fallback/` through the example
`dump_m5_archived_object_expiry_removal_state_and_metadata_fallback`; the checked-in artifacts are never
hand-edited.

## The explicit lifecycle states

Each binding carries an `ArchivedEvidenceState` with a stable `state_label`:

- `preserved_available` — content bytes and metadata are both preserved; the live target may still be opened.
- `expired` — the retention / validity window elapsed; bytes are pending cleanup and the object may be safely
  removed, while its metadata, provenance, and expiry reason stay presented.
- `removed` — the content bytes have been removed; only metadata, provenance, and a deletion receipt remain, so
  it never dead-links.
- `retention_window_ended` — the retention window ended; the object is eligible for a reviewed cleanup / remove.
- `missing_live_target` — the current live object the archive referenced no longer exists; the archived metadata
  stays presented and no open-current-live-object action is offered.
- `metadata_only` — only the metadata is retained by design; the object presents metadata rather than a blank
  pane.

## The removal / expiry note

Every non-available state carries a `removal_note`:

- `reason` — `retention_window_elapsed`, `manual_cleanup_requested`, `policy_mandated_deletion`,
  `source_live_target_removed`, `storage_reclaimed`, `legal_hold_released`, or `metadata_only_by_design`; the
  reason must be allowed for the disclosed state.
- `explanation` — a never-omitted account of the exact expiry / removal outcome.
- `preserved_metadata_note` — confirms the snapshot label, capture time, provenance, and mutation-blocked
  posture stay presented.
- `removal_attribution` — joins the outcome to a **retention / deletion receipt**, a **retirement closure
  ledger**, and a **support packet manifest**, so removal outcomes remain attributable.
- `next_action` — `remove_through_reviewed_cleanup` (Expired / RetentionWindowEnded) or `inspect_metadata_only`.

## The closed action set

`ArchiveStateAction` is closed and analysis / cleanup only — there is no apply / sync / restore action:

- `inspect_metadata` and `export_evidence` — the base set on every binding.
- `remove_archived_object` — offered only where a reviewed cleanup is appropriate (Expired, RetentionWindowEnded).
- `open_current_live_object` — offered only when the archive is preserved with a live target.

## Acceptance criteria mapping

- **A seeded archived object can transition into Expired or Removed while still presenting metadata, provenance,
  and the correct cleanup / removal explanation** — the `expired_narrowed` and `removed_narrowed` fixtures
  narrow preserved-available archives into those states while keeping the historical grammar and adding a
  removal note.
- **No claimed archive consumer degrades to a generic dead-link state when the product can still explain
  expiry / removal** — when `content_bytes_present` is false the binding still renders capture time, provenance,
  and the removal / expiry reason, and the `degrades_to_generic_dead_link` guardrail is always false.
- **Export / support packets preserve the same expired / removed vocabulary used in the product UI** — the
  support export and matrix CSV carry the same `state` and `removal_expiry_reason` tokens the UI uses.

## Guardrails (row invariants, one per binding)

- `historical_side_mutation_blocked` — MUST be true.
- `reopens_live_target_without_validating_identity_trust_route_and_authority` — MUST be false.
- `degrades_to_generic_dead_link` — MUST be false.
- `removes_content_without_attribution` — MUST be false.
- `presents_expired_or_removed_as_live_or_current` — MUST be false.
- `drops_removal_or_expiry_vocabulary_in_export` — MUST be false.

## Regenerating

```text
cargo run -p aureline-ui --example dump_m5_archived_object_expiry_removal_state_and_metadata_fallback -- support-export
cargo run -p aureline-ui --example dump_m5_archived_object_expiry_removal_state_and_metadata_fallback -- csv
cargo run -p aureline-ui --example dump_m5_archived_object_expiry_removal_state_and_metadata_fallback -- report
cargo run -p aureline-ui --example dump_m5_archived_object_expiry_removal_state_and_metadata_fallback -- fixture-expired-narrowed
cargo run -p aureline-ui --example dump_m5_archived_object_expiry_removal_state_and_metadata_fallback -- fixture-removed-narrowed
cargo run -p aureline-ui --example dump_m5_archived_object_expiry_removal_state_and_metadata_fallback -- validate
```
