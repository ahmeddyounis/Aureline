# Artifact Save Truth

This document freezes save-truth expectations for generated, exported, draft,
provider-backed, archive-backed, and structured artifact families that do not
cleanly inherit ordinary source-file save semantics.

The canonical implementation lives in
[`crates/aureline-shell/src/artifact_save_truth/mod.rs`](../../crates/aureline-shell/src/artifact_save_truth/mod.rs).
Its checked-in packet is
[`artifacts/state/artifact_save_truth.json`](../../artifacts/state/artifact_save_truth.json),
its reviewer report is
[`artifacts/state/artifact_save_truth.md`](../../artifacts/state/artifact_save_truth.md),
and its scenario fixtures live under
[`fixtures/state/artifact_save_truth/`](../../fixtures/state/artifact_save_truth/).

## What the packet freezes

- Which metadata-sensitive cues must remain visible before save:
  `encoding`, `newline_mode`, `bom_or_final_newline`,
  `execute_bit_or_permissions`, `generated_state_boundary`, and
  `logical_target_ambiguity`.
- Which fallback disclosure family applies when the preferred path is
  unavailable:
  `atomic_with_disclosed_fallback`, `regenerate_or_export_disclosed`,
  `draft_stage_disclosed`, or `compare_only_blocked`.
- Which fallback actions remain visible on the disclosure sheet:
  `continue`, `inspect_safety_note`, `alternate_target`, and
  `compare_before_save`.
- Which mutating lanes are admitted for each artifact family and whether they
  must `rebase_or_abort`, route to `compare_then_manual_recovery`, or remain
  `not_applicable`.
- Which worked fixtures prove lossy decode risk, metadata preservation,
  execute-bit retention, export/regenerate non-exact-save posture, logical
  target ambiguity, and no-silent-stomp behavior.

## Artifact families

The packet covers these families exactly once:

- `notebook_document`
- `notebook_output_artifact`
- `request_workspace_document`
- `request_response_snapshot`
- `database_export_artifact`
- `profiler_trace_artifact`
- `preview_output_artifact`
- `sync_packet_artifact`
- `provider_local_draft`
- `infrastructure_overlay_document`
- `imported_archive_capture`

## Required invariants

- Metadata-sensitive cues stay visible before mutation when they affect durable
  truth.
- Fallback sheets do not collapse degraded saves into success-only copy; they
  keep continue, inspect, alternate-target, and compare actions explicit.
- Format-on-save, organize-imports, refactor apply, AI apply, and
  generated-output writes either rebase/abort on drift or are marked
  not-applicable explicitly.
- Regeneration, export, draft staging, and compare-only flows never impersonate
  an exact local-file save.

## Fixture intent

The fixture corpus deliberately proves:

- metadata-preserving notebook save with rebase-or-abort automation
- generated notebook output refresh
- remote request-workspace execute-bit retention under conditional write
- provider snapshot export-only posture
- regenerate-from-source database exports
- imported profiler trace lossy-decode disclosure
- preview output regeneration under drift
- offline sync packet draft staging
- provider local draft logical-target ambiguity
- compare-only infrastructure overlays
- imported archive capture copy-only posture
