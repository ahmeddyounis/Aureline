# Merge and conflict-resolution contract for text, structured, generated, binary, and notebook artifacts

This document freezes the cross-tool record family every Aureline
surface reads when it presents a merge conflict — plain text, structured
configuration, generated source or lockfile, binary or otherwise
unmergeable payload, notebook or rich document, and immutable evidence
packet — before the merge UX is free to collapse all five postures into
a single line-oriented three-way merge view. Every conflict surface,
resume / abort review sheet, change-stack panel, AI branch-agent panel,
support / export bundle's git section, and audit lane reads exactly one
record from this family and never invents its own "conflict",
"resolution surface", or "fallback" vocabulary.

The machine-readable boundaries are:

- [`/schemas/vcs/conflict_class.schema.json`](../../schemas/vcs/conflict_class.schema.json)
  — the `conflict_class_record` and `conflict_class_audit_event_record`
  shapes plus the closed `conflict_class_value`,
  `resolution_surface_class`, `source_of_truth_authority_class`,
  `fallback_to_text_class`, `unknown_key_preservation_class`,
  `unresolved_count_navigation_class`, `regenerate_first_required_class`,
  and `abandon_reopen_safety_class` vocabularies.
- [`/schemas/vcs/conflict_resolution_session.schema.json`](../../schemas/vcs/conflict_resolution_session.schema.json)
  — the `conflict_resolution_session_record`,
  `conflict_resolution_action_record`, and
  `conflict_resolution_session_audit_event_record` shapes plus the
  closed `session_lifecycle_state`, `action_class`, and
  `action_consequence_class` vocabularies.
- [`/schemas/vcs/merge_validation_hint.schema.json`](../../schemas/vcs/merge_validation_hint.schema.json)
  — the `merge_validation_hint_record` and
  `merge_validation_hint_audit_event_record` shapes plus the closed
  `merge_validation_hint_class`, `merge_validation_hint_severity_class`,
  and `merge_validation_hint_lifecycle_state` vocabularies.

Worked cases (a plain text three-way merge with the unresolved hunk
count and "next unresolved" navigation; a structured-config conflict
that preserves unknown vendor keys through the resolution; a lockfile
conflict routed through regenerate-first with the manifest pinned as
canonical input; a binary choose-source-only conflict with byte merge
mechanically forbidden; a notebook cell-aware conflict that surfaces
metadata-filter and output-handling presets; a structured-config
session that opted into the lossy raw-text fallback only after an
explicit acknowledgement; a session blocked from merge-complete by an
open `blocking_must_resolve_before_merge_complete` validation hint;
and a denial when a downstream surface tried to silently render a
generated-source conflict as a text merge) live under
[`/fixtures/vcs/conflict_cases/`](../../fixtures/vcs/conflict_cases/).

The eventual git-service crate's Rust types are the schema of record.
This document and the JSON Schema exports are the cross-tool boundary
every non-owning surface reads. The branch / worktree / history /
stash contract
([`/docs/vcs/git_state_and_worktree_contract.md`](git_state_and_worktree_contract.md))
and the change-object / patch-stack / sequence-editor contract
([`/docs/vcs/change_stack_contract.md`](change_stack_contract.md))
are upstream: every paused-pending-conflict row from those contracts
resolves through one `conflict_resolution_session_record` and one
`conflict_class_record`, and the
`merge_conflict_class_record_id_ref` slots they reserved become
required non-null on rows whose `conflict_handoff_class` is one of
the `conflict_resolution_packet_*` values.

Companion artifacts:

- [`/schemas/review/review_surface_record.schema.json`](../../schemas/review/review_surface_record.schema.json)
  and
  [`/docs/review/structured_artifact_review_seed.md`](../review/structured_artifact_review_seed.md)
  — the structured-artifact review-surface seed each conflict class
  binds to through `review_artifact_class_refs`. The merge / conflict
  contract carries the *resolution-side* posture (resolution surface,
  fallback class, regenerate-first requirement, abandon / reopen
  safety, unresolved-count navigation); the structured-artifact
  review seed carries the *compare-side* posture (default review
  surface, semantic vs. structure-aware, paired-text declaration,
  notebook presets, generated-source back-link, Git merge-driver
  admission). The two contracts agree on conflict-class identity
  through `conflict_class_value` ↔ `review_artifact_class` pairing
  and never duplicate vocabulary.
- [`/schemas/vcs/change_object.schema.json`](../../schemas/vcs/change_object.schema.json)
  and
  [`/docs/vcs/change_stack_contract.md`](change_stack_contract.md)
  — the change-object family whose `conflict_handoff_class` is the
  upstream signal that a conflict-resolution session is required.
  This contract pins the session shape; that contract pins the
  change-object the session resolves on behalf of.
- [`/schemas/vcs/history_operation_state.schema.json`](../../schemas/vcs/history_operation_state.schema.json)
  and
  [`/docs/vcs/git_state_and_worktree_contract.md`](git_state_and_worktree_contract.md)
  — the in-progress-history-operation row whose
  `paused_pending_conflict_resolution` state pairs with one
  `conflict_resolution_session_record` per conflicting artifact.
  The session row carries the `history_operation_state_id_ref`
  back-pointer.
- [`/schemas/integration/approval_ticket.schema.json`](../../schemas/integration/approval_ticket.schema.json)
  — the approval-ticket model every mutation-class action
  (merge-complete, abandon, regenerate, external-tool handoff)
  cites. A mutation never appears available without resolving to
  an approval ticket plus a command id.
- [`/schemas/governance/capability_lifecycle.schema.json`](../../schemas/governance/capability_lifecycle.schema.json)
  — re-exported `freshness_class`, `client_scope`, and
  `redaction_class` vocabularies (ADR-0011). This contract never
  redefines them.
- [`/schemas/identity/policy_bundle.schema.json`](../../schemas/identity/policy_bundle.schema.json)
  — re-exported `workspace_trust_state_class` and policy / trust
  envelope (ADR-0001 / ADR-0018). A merge-complete or abandon
  action never appears available under an unset trust decision.

Normative source anchors:

- `.t2/docs/Aureline_Technical_Architecture_Document.md` — git
  service architecture, structured-artifact diff / merge / review
  architecture (§16.7.1), notebook architecture (§20), and
  Appendix AC (structured-artifact review corpus and conformance
  rules).
- `.t2/docs/Aureline_PRD.md` — explicit-recovery, regenerate-first,
  no-silent-text-merge, and abandon / reopen safety MUST / SHOULD
  language.

If this contract disagrees with those sources, those sources win and
this document plus the schemas and fixtures update in the same change.

## Why freeze this now

Until this contract lands, every surface that touches a merge conflict
would be free to invent its own resolution vocabulary:

- A "merge conflict" surface could render a structured config, a
  notebook, and an opaque binary as one inline three-way text view.
  The user would resolve a notebook by editing JSON bytes,
  silently dropping cell identity and unknown vendor metadata; the
  binary would be saved with a corrupt midpoint between two image
  payloads; the lockfile would be hand-edited into a resolver state
  that does not match any real solver run.
- A "regenerate" affordance could appear without naming the canonical
  input it regenerates from. The user would press it on a lockfile
  conflict and discover only after the regenerate that the manifest
  pin had drifted between session admission and regenerate, leaving
  a lockfile that does not match the manifest the session opened
  against.
- A "merge complete" button could land on top of unresolved hunks,
  unknown-key drops, or unverified external-tool round-trips. The
  reviewer would have no mechanical way to tell that the merge was
  in fact incomplete.
- An "abandon" button could roll back to an unspecified state, or to
  the working directory's current state rather than the recovery
  object the session pinned at admission. Reopen would reattach to a
  drifted base.
- A provider review overlay or AI branch-agent could "resolve" a
  conflict on a binary or evidence row through silent byte
  manipulation; support and audit would have no record of what the
  resolution actually changed.

Freezing one record family
(`conflict_class_record`, `conflict_resolution_session_record`,
`conflict_resolution_action_record`, `merge_validation_hint_record`)
and the closed conflict-class, resolution-surface, source-of-truth
authority, fallback-to-text, regenerate-first, abandon / reopen,
session-lifecycle, action-class, action-consequence, validation-hint,
severity, and lifecycle vocabularies they read solves all five
problems in one shape.

## Scope

Frozen at this revision:

1. The `conflict_class_record` shape every conflict surface reads to
   resolve which class of conflict it is rendering, including:
   - one `conflict_class_value` from the closed seven-value vocabulary
     (`plain_text_line_oriented_merge`,
     `structured_config_key_aware_merge`,
     `lockfile_or_dependency_manifest_regenerate_first`,
     `generated_or_derived_source_regenerate_first`,
     `binary_or_unmergeable_choose_source_only`,
     `notebook_cell_aware_merge`,
     `evidence_immutable_no_merge`);
   - the `default_resolution_surface_class` from the closed seven-value
     vocabulary
     (`inline_three_way_merge_view`,
     `structure_aware_merge_view_with_unknown_key_preservation`,
     `regenerate_first_with_canonical_input_pinned`,
     `choose_source_only_no_byte_merge`,
     `cell_aware_merge_view`,
     `external_tool_handoff_with_round_trip_required`,
     `evidence_reader_no_resolution_admissible`);
   - the `default_source_of_truth_authority_class` from the closed
     seven-value vocabulary
     (`local_branch_authoritative_for_text`,
     `structured_artifact_self_canonical`,
     `manifest_canonical_lockfile_regenerated`,
     `generator_input_canonical_artifact_regenerated`,
     `binary_payload_one_of_two_authoritative_choose_source`,
     `notebook_self_canonical_cell_identity_required`,
     `evidence_packet_immutable_authoritative_no_merge`);
   - the `default_fallback_to_text_class` from the closed three-value
     vocabulary
     (`no_fallback_admitted_native_surface_only`,
     `raw_text_fallback_explicit_user_acknowledged_lossy`,
     `raw_text_fallback_forbidden_for_class`);
   - the `default_unknown_key_preservation_class` from the closed
     three-value vocabulary
     (`required_round_trip_preservation`,
     `recommended_with_warning_on_drop`,
     `not_applicable_no_user_authored_metadata`);
   - the `default_unresolved_count_navigation_class` from the closed
     five-value vocabulary
     (`count_required_with_next_unresolved_navigation`,
     `count_required_with_next_cell_or_section_navigation`,
     `count_not_applicable_choose_source_only`,
     `count_not_applicable_regenerate_first`,
     `count_not_applicable_evidence_immutable`);
   - the `default_regenerate_first_required_class` from the closed
     three-value vocabulary
     (`regenerate_first_not_required`,
     `regenerate_first_required_canonical_input_pinned`,
     `regenerate_first_required_external_tool_handoff`);
   - the `default_abandon_reopen_safety_class` from the closed
     three-value vocabulary
     (`abandon_rolls_back_to_recovery_object_reopen_admissible`,
     `abandon_blocked_pending_user_review`,
     `abandon_with_lossy_state_export_required_before_reopen`);
   - the `review_artifact_class_refs` (closed list of structured-
     artifact classes from the review-surface seed this conflict class
     binds to);
   - the `policy_context`, `client_scopes`, `redaction_class`,
     `freshness_class`, `summary_label`, attribution, and timestamps;
     and
   - the audit-event row shape on the `conflict_class` audit stream.

   The schema's allOf gates pin every conflict_class_value to its
   matching surface, source-of-truth, navigation, regenerate-first,
   and (for non-text classes) fallback / unknown-key / abandon-safety
   constraints. A row that disagrees denies with
   `resolution_surface_must_match_conflict_class`.

2. The `conflict_resolution_session_record` shape every live conflict
   reads, including:
   - the `conflict_class_record_id_ref` (the row from §1) and the
     `conflict_class_value` pinned on the session for cheap reads;
   - the optional `history_operation_state_id_ref` (the in-progress
     history operation that paused on this conflict; null when the
     session was minted outside an in-progress operation, e.g. a
     stash-apply conflict on a clean worktree);
   - the base / current / incoming revision id refs and the
     `result_revision_id_ref` (required non-null on
     `session_completed_merge_admitted` rows);
   - the `artifact_identity_ref` (the path-equivalent locator family
     token; raw absolute paths never appear);
   - the `resolution_surface_class` and `fallback_to_text_class`
     actually rendered (paired through allOf with the conflict-class
     row's required values; a mismatch denies);
   - the `unresolved_count` plus
     `unresolved_count_navigation_class` so reviewers can see how
     many hunks / cells / sections remain;
   - the `regenerate_canonical_input_envelope` (required when the
     surface is `regenerate_first_with_canonical_input_pinned`); and
     the `external_tool_handoff_envelope` (required when the surface
     is `external_tool_handoff_with_round_trip_required`);
   - the `abandon_reopen_safety_class` (read from the conflict-class
     row);
   - the `session_lifecycle_state` from the closed seven-value
     vocabulary
     (`session_drafted_no_resolution_in_progress`,
     `session_active_resolution_in_progress`,
     `session_paused_pending_external_tool_handoff`,
     `session_paused_pending_validation_hint_review`,
     `session_completed_merge_admitted`,
     `session_abandoned_rolled_back_to_recovery_object`,
     `session_archived_tombstone`);
   - the `recovery_object_ref` (always non-null past
     `session_drafted_*`; abandon rolls back to this object and
     reopen reattaches to it);
   - the `ordered_action_id_refs` chain (resolution actions in order)
     and the `merge_validation_hint_id_refs` chain (validation hints
     emitted against the session);
   - the `actor_ref`, `command_id_ref`, `approval_ticket_ref`
     attribution required on every mutation; and
   - the audit-event row shape on the `conflict_resolution_session`
     audit stream.

3. The `conflict_resolution_action_record` shape every resolution
   action reads, including:
   - the parent `conflict_resolution_session_id_ref` and the
     zero-based `ordinal`;
   - one `action_class` from the closed ten-value vocabulary
     (`choose_source_current`, `choose_source_incoming`,
     `accept_hunk_from_current`, `accept_hunk_from_incoming`,
     `accept_combined_hunk_user_edited`,
     `regenerate_from_canonical_input`,
     `external_tool_handoff_round_trip_required`,
     `merge_complete_admit_result`,
     `abandon_rollback_to_recovery_object`,
     `reopen_after_validation_hint_review`);
   - one `action_consequence_class` from the closed six-value
     vocabulary
     (`local_only_reversible_via_recovery_object`,
     `mutates_canonical_artifact_reversible_via_recovery_object`,
     `regenerates_derived_artifact_input_unchanged`,
     `routes_through_external_tool_user_must_round_trip`,
     `irreversible_provider_side_effect_blocked_until_admission`,
     `evidence_immutable_no_consequence_admissible`);
   - the `target_hunk_or_cell_or_section_ref` (required on
     `accept_hunk_*` actions) and the `output_revision_id_ref`
     (required on `merge_complete_admit_result`);
   - the `actor_ref`, `command_id_ref`, and `approval_ticket_ref`
     attribution.

4. The `merge_validation_hint_record` shape every validation hint
   reads, including:
   - the `conflict_resolution_session_id_ref` (always non-null);
   - one `merge_validation_hint_class` from the closed ten-value
     vocabulary
     (`unresolved_marker_remaining`,
     `unknown_key_dropped_or_renamed`,
     `regenerate_input_drifted_from_pin`,
     `cell_identity_collision_unstable`,
     `output_payload_modified_outside_cell_aware_view`,
     `binary_payload_silent_byte_diff_detected`,
     `evidence_packet_modification_attempted`,
     `text_fallback_lossy_paired_with_structured_artifact`,
     `external_tool_round_trip_unverified`,
     `validation_hint_recovery_object_drifted`);
   - one `merge_validation_hint_severity_class` from the closed
     three-value vocabulary
     (`info_advisory_no_block`, `warning_user_must_review`,
     `blocking_must_resolve_before_merge_complete`);
   - one `merge_validation_hint_lifecycle_state` from the closed
     four-value vocabulary
     (`hint_open_unacknowledged`,
     `hint_acknowledged_advisory_only`,
     `hint_resolved`,
     `hint_archived_tombstone`);
   - the `waiver_review_event_record_id_ref` (required when a
     `blocking_must_resolve_before_merge_complete` hint is
     resolved); and
   - the audit-event row shape on the `merge_validation_hint`
     audit stream.

5. The acceptance invariants this contract enforces:

   - Reviewers can tell conflict class, source-of-truth, unresolved
     count, and safe escape path before taking action. Every session
     row resolves to one `conflict_class_record_id_ref`; the
     resolution surface, the source-of-truth authority, the
     unresolved count, and the abandon / reopen safety class are all
     read mechanically from the conflict-class row paired with the
     session row.
   - Structured and generated conflicts cannot silently degrade into
     lossy text merges without explicit fallback labelling. The
     conflict-class row's `default_fallback_to_text_class` is
     `raw_text_fallback_forbidden_for_class` for lockfile, generated,
     binary, and evidence rows; a session that tries to render
     `inline_three_way_merge_view` for any of those classes denies
     with `structured_or_generated_silent_text_merge_forbidden` /
     `binary_or_unmergeable_class_forbids_byte_merge` /
     `evidence_immutable_class_forbids_resolution_action`. The only
     lossy text-fallback admissible today is on
     `structured_config_key_aware_merge` rows whose audit stream
     carries a `text_fallback_lossy_paired_with_structured_artifact`
     warning hint and whose `fallback_to_text_class` is
     `raw_text_fallback_explicit_user_acknowledged_lossy`.
   - Authority and side-effect rules are explicit per action class.
     The `action_consequence_class` vocabulary names what each action
     does to local state and to provider state. Merge-complete on a
     row whose consequence is
     `irreversible_provider_side_effect_blocked_until_admission` is
     refused until the approval ticket admits; binary merge-complete
     resolves through `choose_source_current` /
     `choose_source_incoming` only and `accept_hunk_*` denies with
     `choose_source_only_class_forbids_accept_hunk`; evidence rows
     forbid every action class except read-only navigation.
   - Regenerate-first conflicts pin a canonical input. Lockfile and
     generated-source sessions MUST cite a non-null
     `regenerate_canonical_input_envelope` whose
     `canonical_input_pin_class` is one of `canonical_input_pin_fresh`,
     `canonical_input_pin_drifted_warning`, or
     `canonical_input_pin_drifted_blocking`. A drifted-blocking pin
     pairs with a `blocking_must_resolve_before_merge_complete`
     validation hint and prevents merge-complete admission until the
     pin is refreshed.
   - External-tool handoff requires a verified round-trip. Sessions
     whose `resolution_surface_class` is
     `external_tool_handoff_with_round_trip_required` MUST cite an
     `external_tool_handoff_envelope` whose
     `round_trip_verification_state` is
     `round_trip_verified_against_corpus` before
     `merge_complete_admit_result` admits; an unverified state
     pairs with the `external_tool_round_trip_unverified` validation
     hint and denies merge-complete with
     `external_tool_round_trip_unverified_for_merge_complete`.
   - Merge-complete is blocked while blocking validation hints are
     open. A session whose `merge_validation_hint_id_refs` resolve
     to any hint with
     `severity = blocking_must_resolve_before_merge_complete` and
     `lifecycle_state != hint_resolved` denies merge-complete with
     `merge_complete_blocked_pending_blocking_validation_hint`.
     Resolution of a blocking hint at severity
     `blocking_must_resolve_before_merge_complete` requires a
     `waiver_review_event_record_id_ref` recorded on the
     `merge_validation_hint` audit stream.
   - Abandon and reopen are reachable through the recovery object.
     Every session past `session_drafted_*` cites a non-null
     `recovery_object_ref`; abandon rolls back to that object;
     reopen reattaches to that object. The
     `abandon_reopen_safety_class` names the per-class rule:
     evidence rows block abandon pending review; binary rows that
     admitted partial choose-source actions require a lossy-state
     export before reopen.
   - Notebook conflicts pair with cell identity and metadata /
     output presets. Notebook sessions resolve through
     `cell_aware_merge_view`; an action that bypasses the cell-aware
     view emits an
     `output_payload_modified_outside_cell_aware_view` validation
     hint and a row that lacks cell identity denies with
     `notebook_cell_identity_required_for_cell_aware_merge`. The
     notebook presets themselves live on the structured-artifact
     review-surface row and are inherited through the conflict-class
     row's `review_artifact_class_refs`.
   - Binary and evidence rows forbid byte-level merge and
     write-back. `binary_or_unmergeable_choose_source_only` denies
     `accept_hunk_*` and any byte-level merge attempt with
     `binary_or_unmergeable_class_forbids_byte_merge`;
     `evidence_immutable_no_merge` denies every resolution action
     with `evidence_immutable_class_forbids_resolution_action` and
     forces the surface to `evidence_reader_no_resolution_admissible`.

Out of scope until a superseding decision row opens:

- Implementing a full merge engine, a real three-way merge driver,
  a notebook diff / merge UI, an image diff / merge UI, or a
  provider-specific merge queue. The contract reserves the row
  shape; the engine is a later lane.
- Cross-repo or distributed conflict federation; provider-side
  conflict APIs.
- Authoring the dedicated history-edit recovery contract — that
  remains the forward dependency the change-stack and history-
  operation-state contracts already reserve.

## 1. The conflict-class record

Every merge / conflict surface in Aureline (the inline three-way
view, the structure-aware merge view, the cell-aware notebook merge,
the regenerate-first surface, the choose-source-only surface, the
external-tool handoff surface, the evidence reader, the resume /
abort review sheet, the change-stack panel, the AI branch-agent
panel, the support / export bundle, the audit lane) MUST resolve
the conflict it is operating on to exactly one
`conflict_class_record`. The record is the answer to seven
questions a reviewer must be able to answer without opening any
other object:

1. *What class of conflict is this?* — `conflict_class_value`.
2. *Which resolution surface is the user looking at?* —
   `default_resolution_surface_class`.
3. *Which side is the source of truth?* —
   `default_source_of_truth_authority_class`.
4. *Is unknown / vendor metadata required to round-trip?* —
   `default_unknown_key_preservation_class`.
5. *How many hunks / cells / sections are unresolved, and what does
   "next unresolved" mean?* —
   `default_unresolved_count_navigation_class`.
6. *Does the resolution flow through regenerate-first, an external
   tool, or in-place merge?* —
   `default_regenerate_first_required_class`.
7. *What is the safe escape path (abandon / reopen)?* —
   `default_abandon_reopen_safety_class`.

The schema's allOf gates pin every `conflict_class_value` to its
matching surface, source-of-truth, navigation, regenerate-first
posture, and (for non-text classes) fallback / unknown-key /
abandon-safety constraints. A row that mismatches denies with
`resolution_surface_must_match_conflict_class`.

### 1.1 Class summary

| `conflict_class_value` | Surface | Source of truth | Fallback to text |
|---|---|---|---|
| `plain_text_line_oriented_merge` | `inline_three_way_merge_view` | `local_branch_authoritative_for_text` | `no_fallback_admitted_native_surface_only` |
| `structured_config_key_aware_merge` | `structure_aware_merge_view_with_unknown_key_preservation` | `structured_artifact_self_canonical` | `no_fallback_admitted_native_surface_only` *or* `raw_text_fallback_explicit_user_acknowledged_lossy` |
| `lockfile_or_dependency_manifest_regenerate_first` | `regenerate_first_with_canonical_input_pinned` | `manifest_canonical_lockfile_regenerated` | `raw_text_fallback_forbidden_for_class` |
| `generated_or_derived_source_regenerate_first` | `regenerate_first_with_canonical_input_pinned` | `generator_input_canonical_artifact_regenerated` | `raw_text_fallback_forbidden_for_class` |
| `binary_or_unmergeable_choose_source_only` | `choose_source_only_no_byte_merge` | `binary_payload_one_of_two_authoritative_choose_source` | `raw_text_fallback_forbidden_for_class` |
| `notebook_cell_aware_merge` | `cell_aware_merge_view` | `notebook_self_canonical_cell_identity_required` | `no_fallback_admitted_native_surface_only` |
| `evidence_immutable_no_merge` | `evidence_reader_no_resolution_admissible` | `evidence_packet_immutable_authoritative_no_merge` | `raw_text_fallback_forbidden_for_class` |

The `review_artifact_class_refs` array binds each conflict class
to one or more rows in
[`/schemas/review/review_surface_record.schema.json`](../../schemas/review/review_surface_record.schema.json).
Plain text binds to `line_oriented_source_artifact`; structured
configs bind to one of `structured_config_json` /
`structured_config_yaml` / `structured_config_toml`; lockfiles bind
to `lockfile_or_dependency_manifest`; generated sources bind to
`generated_source_artifact` (and may also bind to
`source_map_or_debug_sidecar` or `sbom_or_generated_metadata`);
binary and design-snapshot rows bind to `image_or_design_snapshot`;
notebooks bind to `jupyter_notebook`; evidence packets bind to
`evidence_packet`.

### 1.2 Fallback-to-text discipline

`fallback_to_text_class` is closed:

- `no_fallback_admitted_native_surface_only` — the conflict class
  has no admissible text-fallback surface; the native resolution
  surface is mandatory. Plain text, structured configs (default),
  notebook, and evidence rows take this value.
- `raw_text_fallback_explicit_user_acknowledged_lossy` — admissible
  *only* on `structured_config_key_aware_merge` sessions whose
  audit stream carries a paired
  `text_fallback_lossy_paired_with_structured_artifact` warning
  hint. The session row pins this value only after the
  acknowledgement event lands; the surface renders the raw-text
  view with the lossy chip explicit.
- `raw_text_fallback_forbidden_for_class` — the conflict class
  forbids raw text fallback unconditionally. Lockfile, generated,
  binary, and evidence rows take this value. A surface that tries
  to render `inline_three_way_merge_view` on any of these classes
  denies with `structured_or_generated_silent_text_merge_forbidden`
  (or its binary / evidence analogues).

### 1.3 Regenerate-first discipline

`regenerate_first_required_class` is closed:

- `regenerate_first_not_required` — plain text, structured config,
  notebook, evidence; no canonical input outside the artifact
  itself.
- `regenerate_first_required_canonical_input_pinned` — lockfile and
  generated-source rows; the resolution session MUST cite a non-null
  `regenerate_canonical_input_envelope` before admission.
- `regenerate_first_required_external_tool_handoff` — reserved for
  classes whose canonical input is generated by an external tool
  whose handoff returns a round-trip artifact (today, none of the
  seven default rows take this value, but the slot is reserved so
  the schema does not need to break when an external-only
  generator class lands).

## 2. The conflict-resolution session record

Every live conflict surface MUST resolve to exactly one
`conflict_resolution_session_record`. The record is the answer to
seven questions a reviewer must be able to answer at any point in
the session's lifecycle:

1. *Which conflict class does this session resolve against?* —
   `conflict_class_record_id_ref` plus `conflict_class_value`.
2. *Which in-progress history operation paused on it?* —
   `history_operation_state_id_ref` (null when the session was minted
   outside an in-progress operation).
3. *What are the base / current / incoming / result revisions?* —
   `base_revision_id_ref`, `current_revision_id_ref`,
   `incoming_revision_id_ref`, `result_revision_id_ref`.
4. *How many hunks / cells / sections are unresolved?* —
   `unresolved_count` paired with
   `unresolved_count_navigation_class`.
5. *Which canonical input or external tool is pinned?* —
   `regenerate_canonical_input_envelope` and / or
   `external_tool_handoff_envelope`.
6. *What is the recovery object and the abandon / reopen rule?* —
   `recovery_object_ref` and `abandon_reopen_safety_class`.
7. *Where is the session in its lifecycle?* —
   `session_lifecycle_state` plus `ordered_action_id_refs` and
   `merge_validation_hint_id_refs`.

### 2.1 Base / current / incoming / result semantics

The session row carries four revision id refs that pin the merge
quartet:

- `base_revision_id_ref` — the merge base (the fork point or the
  pre-rebase parent). Always non-null.
- `current_revision_id_ref` — the local-branch head at conflict
  mint. Always non-null.
- `incoming_revision_id_ref` — the merge / rebase / cherry-pick /
  revert / mailbox-apply / stash-apply incoming revision. Always
  non-null.
- `result_revision_id_ref` — the resolved revision the session
  admitted. Required (non-null) on
  `session_completed_merge_admitted`; null on every other
  lifecycle state. A session that admits merge-complete without
  citing a result revision denies with
  `merge_complete_result_revision_required`.

Raw revision shas, raw paths, and raw author identity strings never
appear; the four refs are opaque ids into the workspace family.

### 2.2 Session lifecycle

`session_lifecycle_state` is closed:

- `session_drafted_no_resolution_in_progress` — the session is
  drafted but no resolution action has been recorded; the recovery
  object pin is optional at this state.
- `session_active_resolution_in_progress` — at least one resolution
  action has been recorded; the recovery object MUST be non-null.
- `session_paused_pending_external_tool_handoff` — the session
  paused awaiting a round-trip artifact from an external tool;
  reopen reattaches when the handoff returns and the round-trip
  is verified.
- `session_paused_pending_validation_hint_review` — the session
  paused awaiting an explicit user review of a warning or blocking
  validation hint; reopen reattaches when the hint flips to
  `hint_acknowledged_advisory_only` (advisory) or `hint_resolved`
  (blocking, with a waiver-review event).
- `session_completed_merge_admitted` — the session admitted a
  result; pairs with a non-null `result_revision_id_ref` and
  `completed_at`.
- `session_abandoned_rolled_back_to_recovery_object` — the session
  was abandoned and the worktree was rolled back to the recovery
  object pinned at admission; pairs with a non-null `abandoned_at`
  and a null `result_revision_id_ref`.
- `session_archived_tombstone` — the row is kept as a tombstone for
  audit / restore.

## 3. Resolution actions

Every action recorded against a session resolves to one
`conflict_resolution_action_record`. The `action_class` vocabulary
is closed and read mechanically by the action surface so the
"choose current", "choose incoming", "accept hunk", "regenerate",
"hand off", "merge complete", "abandon", and "reopen" buttons all
agree on consequence and recoverability:

| `action_class` | Default `action_consequence_class` | Admissible on |
|---|---|---|
| `choose_source_current` / `choose_source_incoming` | `local_only_reversible_via_recovery_object` | every class except `evidence_immutable_no_merge` |
| `accept_hunk_from_current` / `accept_hunk_from_incoming` | `local_only_reversible_via_recovery_object` | every class except `binary_or_unmergeable_choose_source_only` and `evidence_immutable_no_merge` (binary denies with `choose_source_only_class_forbids_accept_hunk`) |
| `accept_combined_hunk_user_edited` | `local_only_reversible_via_recovery_object` | plain text, structured config, notebook |
| `regenerate_from_canonical_input` | `regenerates_derived_artifact_input_unchanged` | lockfile, generated (the conflict-class row's `regenerate_first_required_class` MUST be `regenerate_first_required_*`) |
| `external_tool_handoff_round_trip_required` | `routes_through_external_tool_user_must_round_trip` | rows whose surface is `external_tool_handoff_with_round_trip_required` |
| `merge_complete_admit_result` | `mutates_canonical_artifact_reversible_via_recovery_object` (or `irreversible_provider_side_effect_blocked_until_admission` when the merge publishes provider-side) | every class except `evidence_immutable_no_merge`; blocked when blocking validation hints are open |
| `abandon_rollback_to_recovery_object` | `local_only_reversible_via_recovery_object` | every class except `evidence_immutable_no_merge` (which uses `abandon_blocked_pending_user_review`) |
| `reopen_after_validation_hint_review` | `local_only_reversible_via_recovery_object` | rows whose lifecycle is `session_paused_pending_validation_hint_review` |

Every action whose consequence is more than `local_only_*` MUST
cite a non-null `approval_ticket_ref`.

## 4. Validation hints

Every validation hint emitted against a session resolves to one
`merge_validation_hint_record`. The hint is the cross-tool record
the session reads to decide whether merge-complete is admissible.
Severities are closed:

- `info_advisory_no_block` — the hint is advisory; merge-complete
  is unaffected. Lifecycle moves freely between
  `hint_open_unacknowledged`, `hint_acknowledged_advisory_only`,
  and `hint_resolved`.
- `warning_user_must_review` — merge-complete is admissible after
  the user moves the lifecycle to `hint_acknowledged_advisory_only`
  or `hint_resolved`.
- `blocking_must_resolve_before_merge_complete` — merge-complete
  is denied until lifecycle reaches `hint_resolved`. Resolution at
  this severity requires a non-null
  `waiver_review_event_record_id_ref` recorded on the
  `merge_validation_hint` audit stream.

The ten hint classes
(`unresolved_marker_remaining`, `unknown_key_dropped_or_renamed`,
`regenerate_input_drifted_from_pin`,
`cell_identity_collision_unstable`,
`output_payload_modified_outside_cell_aware_view`,
`binary_payload_silent_byte_diff_detected`,
`evidence_packet_modification_attempted`,
`text_fallback_lossy_paired_with_structured_artifact`,
`external_tool_round_trip_unverified`,
`validation_hint_recovery_object_drifted`) cover the structural
problems the merge surface is allowed to surface mechanically.
Adding a new hint class is additive-minor and bumps
`merge_validation_hint_schema_version`.

## 5. Admission and denial rules

The schemas enforce, through allOf gates and closed denial-reason
vocabularies, the following rules:

| Rule | Mechanism |
|---|---|
| Resolution surface MUST match conflict class. | `resolution_surface_must_match_conflict_class` denial on `conflict_class_record` and `conflict_resolution_session_record`. |
| Structured / generated conflicts cannot silently degrade into text merges. | `structured_or_generated_silent_text_merge_forbidden`, `binary_or_unmergeable_class_forbids_byte_merge`, `evidence_immutable_class_forbids_resolution_action` denials. |
| Fallback to text MUST be explicit on non-text classes. | `fallback_to_text_must_be_explicit_for_non_text_class` denial paired with `text_fallback_lossy_paired_with_structured_artifact` warning hint. |
| Regenerate-first conflicts pin a canonical input. | `regenerate_first_canonical_input_pinned_required` denial. |
| External-tool handoff requires round-trip verification before merge-complete. | `external_tool_handoff_envelope_required` plus `external_tool_round_trip_unverified_for_merge_complete` denials. |
| Choose-source-only classes forbid `accept_hunk_*`. | `choose_source_only_class_forbids_accept_hunk` denial. |
| Evidence-immutable classes forbid every resolution action. | `evidence_immutable_class_forbids_resolution_action` denial. |
| Notebook merge requires cell identity. | `notebook_cell_identity_required_for_cell_aware_merge` denial. |
| Merge-complete is blocked while blocking validation hints are open. | `merge_complete_blocked_pending_blocking_validation_hint` denial. |
| Abandon and reopen require the recovery object. | `abandon_reopen_recovery_object_required` denial. |
| Sessions past draft pin a recovery object at admission. | `session_recovery_object_required_at_admission` denial. |

## 6. Audit streams

Three audit streams are reserved by this contract:

- `conflict_class_audit_event` — closed event-id vocabulary
  including `conflict_class_record_minted`,
  `conflict_class_record_updated`,
  `conflict_class_record_archived`,
  `conflict_class_audit_denial_emitted`. Denial events MUST cite
  one reason from the `conflict_class_denial_reason` vocabulary.
- `conflict_resolution_session_audit_event` — closed event-id
  vocabulary including
  `conflict_resolution_session_admitted`,
  `conflict_resolution_session_action_recorded`,
  `conflict_resolution_session_paused_pending_external_tool`,
  `conflict_resolution_session_paused_pending_validation_hint_review`,
  `conflict_resolution_session_resumed`,
  `conflict_resolution_session_completed`,
  `conflict_resolution_session_abandoned`,
  `conflict_resolution_session_archived`,
  `conflict_resolution_session_audit_denial_emitted`. Denial events
  MUST cite one reason from the
  `conflict_resolution_session_denial_reason` vocabulary.
- `merge_validation_hint_audit_event` — closed event-id vocabulary
  including `merge_validation_hint_minted`,
  `merge_validation_hint_acknowledged`,
  `merge_validation_hint_resolved`,
  `merge_validation_hint_waiver_review_recorded`,
  `merge_validation_hint_archived`,
  `merge_validation_hint_audit_denial_emitted`. Denial events MUST
  cite one reason from the `merge_validation_hint_denial_reason`
  vocabulary.

Adding a new denial reason or a new audit-event id is additive-minor
and bumps the per-record schema-version const; repurposing an
existing value is breaking and requires a new decision row.

## 7. Redaction posture

Every record published against this contract carries one
`redaction_class` from the re-exported capability-lifecycle
vocabulary. Defaults:

- Conflict-class, session, and hint rows default to
  `metadata_safe_default`.
- Rows whose actor or approval ticket touches a credentialed flow
  (e.g. an external-tool handoff under a managed-admin surface)
  MUST raise to `operator_only_restricted`.
- Support exports of any row MUST raise to
  `internal_support_restricted`.
- Raw paths, raw URLs, raw branch / commit / remote names, raw
  author identity strings, raw notebook output bytes, raw generator
  argv, raw secrets, and raw lockfile / SBOM bodies never appear on
  any record published against this contract.

## 8. Acceptance cross-walk

| Acceptance bullet from the plan | Where it lands |
|---|---|
| Reviewers can tell conflict class, source-of-truth, unresolved count, and safe escape path before taking action. | §1.1 conflict-class record + §2 session record + §3 action record + the `resolution_surface_must_match_conflict_class` and `merge_complete_result_revision_required` denials. Fixtures `session_plain_text_three_way_merge.yaml`, `session_structured_config_unknown_keys_preserved.yaml`, `session_lockfile_regenerate_first.yaml`, `session_binary_choose_source_only.yaml`, and `session_notebook_cell_aware_metadata_outputs.yaml`. |
| Structured and generated conflicts cannot silently degrade into lossy text merges without explicit fallback labelling. | §1.2 fallback-to-text discipline + the `structured_or_generated_silent_text_merge_forbidden`, `binary_or_unmergeable_class_forbids_byte_merge`, and `evidence_immutable_class_forbids_resolution_action` denials. Fixtures `session_structured_config_text_fallback_explicit_lossy.yaml` and `audit_silent_text_merge_on_generated_source_denied.yaml`. |
| Fixtures cover at least: plain text merge, structured config conflict preserving unknown keys, lockfile regenerate-first conflict, binary choose-source case, and notebook metadata/output conflict. | Fixtures `session_plain_text_three_way_merge.yaml`, `session_structured_config_unknown_keys_preserved.yaml`, `session_lockfile_regenerate_first.yaml`, `session_binary_choose_source_only.yaml`, and `session_notebook_cell_aware_metadata_outputs.yaml`. |

## 9. Versioning

Each schema in this family carries a document-level
`*_schema_version` const. Adding a new enum value, a new optional
property, or a new additive sub-record is additive-minor and bumps
the relevant `*_schema_version` const. Repurposing an existing value
is breaking and requires a new decision row. The schemas join the
`vcs` family row in
[`artifacts/governance/schema_families.yaml`](../../artifacts/governance/schema_families.yaml)
and each artifact joins
[`artifacts/governance/control_artifact_index.yaml`](../../artifacts/governance/control_artifact_index.yaml)
in the same change.
