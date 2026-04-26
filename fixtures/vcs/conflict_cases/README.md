# Merge / conflict-resolution worked fixtures

These YAML fixtures exercise the conflict-class, conflict-resolution-
session, resolution-action, and validation-hint contract frozen in
[`/docs/vcs/merge_and_conflict_contract.md`](../../../docs/vcs/merge_and_conflict_contract.md)
and the boundary schemas at
[`/schemas/vcs/conflict_class.schema.json`](../../../schemas/vcs/conflict_class.schema.json),
[`/schemas/vcs/conflict_resolution_session.schema.json`](../../../schemas/vcs/conflict_resolution_session.schema.json),
and
[`/schemas/vcs/merge_validation_hint.schema.json`](../../../schemas/vcs/merge_validation_hint.schema.json).

Every fixture is one record validated against `oneOf` in the
appropriate schema. Each carries only opaque workspace / revision /
artifact / conflict-class / session / action / hint / recovery-object /
approval-ticket / command / actor / policy-epoch handles plus monotonic
placeholder timestamps and redaction-aware labels — no raw absolute
paths, no raw branch / commit / remote URLs, no raw author identity
strings, no raw commit message bodies, no raw patch bodies, no raw
notebook output bytes, no raw generator argv, no raw secrets, and no
raw lockfile / SBOM bodies.

## Conflict-class fixtures

| Fixture | Class / acceptance bullet |
|---|---|
| `class_plain_text_line_oriented_merge.yaml` | `plain_text_line_oriented_merge` row pinning `inline_three_way_merge_view`, `local_branch_authoritative_for_text`, `count_required_with_next_unresolved_navigation`, and `regenerate_first_not_required`. Acceptance bullet 1 — reviewers can tell conflict class, source-of-truth, unresolved count, and safe escape path. |
| `class_lockfile_regenerate_first.yaml` | `lockfile_or_dependency_manifest_regenerate_first` row pinning `regenerate_first_with_canonical_input_pinned`, `manifest_canonical_lockfile_regenerated`, `count_not_applicable_regenerate_first`, and `raw_text_fallback_forbidden_for_class`. Acceptance bullet 2 — structured / generated conflicts cannot silently degrade into lossy text merges. |
| `class_notebook_cell_aware_merge.yaml` | `notebook_cell_aware_merge` row pinning `cell_aware_merge_view`, `notebook_self_canonical_cell_identity_required`, `required_round_trip_preservation`, and `count_required_with_next_cell_or_section_navigation`. Acceptance bullet 1 — reviewers can tell conflict class, source-of-truth, unresolved count, and safe escape path. |

## Conflict-resolution-session fixtures

| Fixture | Class / lifecycle / acceptance bullet |
|---|---|
| `session_plain_text_three_way_merge.yaml` | `plain_text_line_oriented_merge` / `session_active_resolution_in_progress` paused mid-rebase on three unresolved hunks; cites the in-progress history operation, the recovery object, and one open blocking validation hint. Acceptance bullet 3 — plain text merge fixture coverage. |
| `session_structured_config_unknown_keys_preserved.yaml` | `structured_config_key_aware_merge` / `session_active_resolution_in_progress` resolving two top-level keys through `structure_aware_merge_view_with_unknown_key_preservation`; unknown vendor metadata round-trips through every accept action. Acceptance bullet 3 — structured config conflict preserving unknown keys fixture coverage. |
| `session_lockfile_regenerate_first.yaml` | `lockfile_or_dependency_manifest_regenerate_first` / `session_active_resolution_in_progress` routing through `regenerate_first_with_canonical_input_pinned` with the manifest pinned as canonical input (`canonical_input_pin_fresh`). Acceptance bullet 3 — lockfile regenerate-first conflict fixture coverage. |
| `session_binary_choose_source_only.yaml` | `binary_or_unmergeable_choose_source_only` / `session_active_resolution_in_progress` resolving an image-snapshot conflict through `choose_source_only_no_byte_merge`; abandon requires a lossy-state export before reopen. Acceptance bullet 3 — binary choose-source case fixture coverage. |
| `session_notebook_cell_aware_metadata_outputs.yaml` | `notebook_cell_aware_merge` / `session_active_resolution_in_progress` resolving four cells (two metadata, two output) through `cell_aware_merge_view`; cites two validation hints (notebook metadata drop, cell-id collision). Acceptance bullet 3 — notebook metadata/output conflict fixture coverage. |
| `session_structured_config_text_fallback_explicit_lossy.yaml` | `structured_config_key_aware_merge` / `session_paused_pending_validation_hint_review` where the user opted into `raw_text_fallback_explicit_user_acknowledged_lossy` and the audit stream carries the paired `text_fallback_lossy_paired_with_structured_artifact` warning hint; the lossy chip is mechanically explicit. Acceptance bullet 2 — structured / generated conflicts cannot silently degrade into lossy text merges without explicit fallback labelling. |

## Validation-hint fixtures

| Fixture | Class / severity / acceptance bullet |
|---|---|
| `hint_blocking_unresolved_marker_remaining.yaml` | `unresolved_marker_remaining` / `blocking_must_resolve_before_merge_complete` / `hint_open_unacknowledged` against the plain-text three-way merge session. Acceptance bullet 1 — reviewers can tell unresolved count and safe escape path before taking action. |

## Audit-denial fixtures

| Fixture | Denial reason / acceptance bullet |
|---|---|
| `audit_silent_text_merge_on_generated_source_denied.yaml` | `conflict_resolution_session_audit_event_record` carrying `structured_or_generated_silent_text_merge_forbidden` denial when a downstream surface tried to admit a generated-source session under `inline_three_way_merge_view`. Acceptance bullet 2 — structured / generated conflicts cannot silently degrade into lossy text merges. |

## Cross-walk to the spec

- The conflict-class fixture set covers plain text, lockfile
  (regenerate-first), and notebook (cell-aware) — the three classes
  whose defaults differ most sharply from a "one inline three-way
  merge view" world. The remaining four classes
  (`structured_config_key_aware_merge`,
  `generated_or_derived_source_regenerate_first`,
  `binary_or_unmergeable_choose_source_only`,
  `evidence_immutable_no_merge`) are covered through their session
  fixtures (and through the schema's allOf gates which enforce the
  per-class defaults whenever a row is admitted).
- The session fixture set covers the five conflict cases the plan
  required (plain text, structured config preserving unknown keys,
  lockfile regenerate-first, binary choose-source, notebook
  metadata/output). The structured-config-text-fallback fixture
  exercises the only path through which a structured row may render
  raw text — and demonstrates that the path requires a paired warning
  hint and an explicit lossy acknowledgement on the audit stream.
- The validation-hint fixture covers the blocking-severity gate that
  prevents merge-complete admission until the hint is resolved
  through a waiver-review event.
- The audit-denial fixture covers the rule that structured / generated
  conflicts cannot silently degrade into lossy text merges.
- Forward dependency slots (`merge_conflict_class_record_id_ref`
  and `history_edit_recovery_record_id_ref`) are set to `null` on
  every fixture; they will become non-null when paired downstream
  contracts (the dedicated history-edit recovery contract, and any
  cross-citing merge / conflict-class consumer) land.
