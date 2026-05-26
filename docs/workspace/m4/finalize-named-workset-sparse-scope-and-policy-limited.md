# Named-workset, sparse-scope, and policy-limited-view UX lineage — contract

This document describes the workset / scope UX lineage record: the
workspace's governed, export-safe projection that proves how named
worksets, sparse slices, and policy-limited views surface across
workspace UX so the user can always distinguish `outside current
slice`, `omitted path`, and `policy hidden` content from `no
result`, and so that widening from a sparse or partial view to a
fuller checkout/workset preserves root identity, query / session
continuity, and restore provenance instead of creating a second
ambiguous workspace truth.

The record is the single artifact every consuming surface
(workset switcher, active-scope chip, search results, explorer
tree, dependency graph, review surface, support export header,
refactor / AI-context / export / deep-link surfaces, headless CLI,
Help/About) ingests instead of cloning status text.

## Input

The projection ingests a live
[`WorksetScopeUxInputs`](../../../crates/aureline-workspace/src/workset_scope_ux_lineage/mod.rs)
envelope verbatim. The envelope carries:

- one
  [`ScopeObservation`](../../../crates/aureline-workspace/src/workset_scope_ux_lineage/mod.rs)
  per governed scope (named workset, sparse slice, policy-limited
  view, full workspace, plus the optional current repo), recording
  the stable scope identity, workset ref / name, scope class,
  included roots, excluded root classes, optional policy-limitation
  ref, optional narrowing cause, hidden member list visibility,
  readiness state, hidden-result count posture, the widen / narrow
  actions offered, and the support-export projection.
- one
  [`SurfaceObservation`](../../../crates/aureline-workspace/src/workset_scope_ux_lineage/mod.rs)
  per UX surface labeling a scope (`workset_switcher`,
  `scope_chip`, `search`, `tree`, `graph`, `review`,
  `support_export`, plus the optional refactor / AI-context /
  export / deep-link surfaces), recording the labeled scope id, the
  result-bearing distinctions (`outside_current_slice`,
  `omitted_path`, `policy_hidden`), the hidden-result count
  disclosure, the slice-ref propagation into deep-link and export
  flows, and the support-export projection.
- one
  [`WidenPreviewObservation`](../../../crates/aureline-workspace/src/workset_scope_ux_lineage/mod.rs)
  per widen-scope preview path, recording the base and candidate
  scope identities, the four required preview disclosures
  (hidden-result count, omitted-root classes, fetch / deepen
  implications, blame / history / search completeness consequences),
  the three preservation flags (root identity, query / session
  continuity, restore provenance), the preservation posture, the
  apply action / disclosure ids, and the support-export projection.

For determinism and replay, the projection accepts the same
envelope shape the fixtures and the headless emitter consume.

## What the record proves

- **Scope-class coverage truth.** Every governed scope ships a row
  bound to one closed `scope_kind` (`selected_workset`,
  `sparse_slice`, `policy_limited_view`, `full_workspace`); the
  optional `current_repo` rides on top.
- **Surface coverage truth.** Every required UX surface ships a row
  bound to one closed `surface_kind`. The required set is
  `workset_switcher`, `scope_chip`, `search`, `tree`, `graph`,
  `review`, and `support_export`; the optional set covers
  `refactor_scope_footer`, `ai_context_inspector`,
  `export_scope_footer`, and `deep_link_dispatcher`.
- **Outside-vs-omitted distinction.** Every result-bearing surface
  (`search`, `tree`, `graph`, `review`) distinguishes
  `outside_current_slice`, `omitted_path`, and `policy_hidden`
  states from `no_result` so a hidden member is never mislabeled as
  missing content.
- **Hidden-result disclosure.** Every result-bearing and
  scope-labeling surface discloses the hidden-result count;
  export-propagating surfaces carry the count into the export
  envelope.
- **Slice-ref propagation truth.** Every surface carries the slice
  / profile ref into deep-link flows; export-propagating surfaces
  additionally carry it into export envelopes so reopened or
  exported views land on the exact same scope identity.
- **Widen-preview truth.** Every widen-scope preview previews
  hidden-result counts, omitted-root classes, fetch / deepen
  implications, and the consequences for blame, history, and
  search completeness before any apply commits, and references a
  non-empty apply action id plus apply disclosure id.
- **Widen-preservation truth.** Every widen preview preserves root
  identity, query / session continuity, and restore provenance.
  The preservation posture must be
  `preserves_identity_and_continuity` or
  `preserved_after_review_with_disclosure`; a
  `creates_new_workspace_truth` posture is forbidden on Stable
  rows.
- **Policy-limited disclosure truth.** Every `policy_limited_view`
  declares one closed `narrowing_cause`. A scope with `admin_policy`
  or `license_or_export_control` cause refuses to expose the hidden
  member list anywhere (UI, exports, deep links).
- **Readiness truth.** Every scope row declares one closed
  `readiness_state`. A `ready` scope must disclose whether the
  hidden-result count is known.
- **Mutation no-rerun honesty for widen.** Every widen-scope
  preview references an apply action id and an apply disclosure id;
  the apply path never fires on resume / reconnect / recovery
  without explicit user action.
- **Inspection precedes destructive widen.** The controlled
  pre-action hook table
  (`inspect_scope`, `compare_before_widen`, `preview_widen`,
  `export_scope`, `rollback_widen`, `repair_scope`) is reachable
  before any destructive widen / narrow commits.
- **Support-export honesty.** Each row's support-export projection
  preserves `scope_class`, `included_roots`,
  `hidden_result_count`, `narrowing_cause`, `readiness_state`, and
  `slice_ref`, and redacts raw secrets, approval tickets,
  delegated credentials, live authority handles, and (for
  admin-policy or license-narrowed scopes) the hidden member list.
- **Producer attribution.** Each record carries the producer ref,
  the schema version, the capture timestamp, and an integrity hash
  derived from the input identities so replay and support pipelines
  can pin the source before applying.
- **Lineage and export stay honest.** Every record sets
  `raw_payload_excluded = true` and carries only opaque refs to the
  source workspace, corpus, and producer.

## Auto-narrowing

The projection auto-narrows below Stable whenever any pillar above
cannot be proven, with a named reason. Reasons include:
`required_scope_class_missing`,
`required_surface_kind_missing`,
`surface_references_unknown_scope`,
`widen_preview_references_unknown_scope`,
`outside_marker_missing`, `omitted_marker_missing`,
`policy_hidden_marker_missing`,
`hidden_result_count_not_disclosed`,
`slice_ref_not_propagated_into_deep_links`,
`slice_ref_not_propagated_into_export`,
`widen_preview_field_missing`,
`widen_loses_root_identity`,
`widen_loses_query_session_continuity`,
`widen_loses_restore_provenance`,
`policy_limited_narrowing_cause_missing`,
`policy_admin_hidden_list_exposed`,
`readiness_hidden_count_unknown`,
`apply_action_metadata_missing`,
`inspection_hook_unavailable`,
`support_export_fields_dropped`,
`support_export_redaction_unsafe`,
`producer_attribution_incomplete`, and
`lineage_export_unsafe`. The level field flips to
`narrowed_below_stable` and the narrow_reasons list enumerates the
failing pillars so consumers cannot inherit an adjacent green row.

## Status-surface projection

The workspace scope status surface, the headless CLI emitter,
Help/About, and support export all consume the same
[`workset_scope_ux_lineage_lines`](../../../crates/aureline-workspace/src/workset_scope_ux_lineage/mod.rs)
projection so the user-facing text, the reviewer-facing text, and
the support packet stay in lockstep with the lineage record.

## Replay gate

[`tests/workset_scope_ux_lineage_replay.rs`](../../../crates/aureline-workspace/tests/workset_scope_ux_lineage_replay.rs)
re-projects each fixture in
[`fixtures/workspace/m4/workset_scope_ux_lineage/`](../../../fixtures/workspace/m4/workset_scope_ux_lineage/)
and asserts the projected record equals the checked-in `expected`,
proving the projection cannot drift without failing CI. The replay
gate additionally asserts every fixture is support-export safe,
that the corpus covers both Stable and narrowed-below-Stable
postures, and that the contract narrows on a missing widen-preview
disclosure or a missing inspection hook.
