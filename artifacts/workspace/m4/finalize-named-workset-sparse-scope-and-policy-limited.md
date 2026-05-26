# Finalize Named Workset, Sparse-Scope, and Policy-Limited-View UX Across Workspace Surfaces — proof packet

Reviewer-facing proof packet for the named-workset, sparse-scope, and
policy-limited-view UX lineage lane: every governed scope class
(`selected_workset`, `sparse_slice`, `policy_limited_view`,
`full_workspace`, plus the optional `current_repo`) ships a row
bound to a stable scope identity, a declared narrowing cause (where
applicable), a closed readiness state, and the widen / narrow
actions it offers. Every required UX surface (`workset_switcher`,
`scope_chip`, `search`, `tree`, `graph`, `review`, `support_export`,
plus the optional refactor / AI-context / export / deep-link
surfaces) discloses which scope it labels and (where the surface
yields rows) distinguishes `outside_current_slice`, `omitted_path`,
and `policy_hidden` states from `no_result`. Every widen-scope
preview previews hidden-result counts, omitted-root classes,
fetch / deepen implications, and the consequences for blame,
history, and search completeness before any apply commits, and
preserves root identity, query / session continuity, and restore
provenance across the widen. Policy-limited views with an
admin-policy or license / export-control narrowing cause never
expose the hidden member list anywhere — including in support
exports. A destructive widen never fires without the controlled
inspection / repair hook table (`inspect_scope`,
`compare_before_widen`, `preview_widen`, `export_scope`,
`rollback_widen`, `repair_scope`) being reachable; a missing hook
narrows the record below Stable with a named reason. This packet
is the stable-line anchor for this lane; dashboards, docs,
Help/About surfaces, and support exports should ingest the typed
sources below rather than cloning this packet's text.

## Canonical machine sources

- Lineage projection and contract types:
  [`/crates/aureline-workspace/src/workset_scope_ux_lineage/`](../../../crates/aureline-workspace/src/workset_scope_ux_lineage/)
- Schema:
  [`/schemas/workspace/workset_scope_ux_lineage.schema.json`](../../../schemas/workspace/workset_scope_ux_lineage.schema.json)
- Headless emitter / CLI:
  [`/crates/aureline-workspace/src/bin/aureline_workset_scope_ux_lineage.rs`](../../../crates/aureline-workspace/src/bin/aureline_workset_scope_ux_lineage.rs)
- Fixtures:
  [`/fixtures/workspace/m4/workset_scope_ux_lineage/`](../../../fixtures/workspace/m4/workset_scope_ux_lineage/)
- Replay gate:
  [`/crates/aureline-workspace/tests/workset_scope_ux_lineage_replay.rs`](../../../crates/aureline-workspace/tests/workset_scope_ux_lineage_replay.rs)
- Companion contract doc:
  [`/docs/workspace/m4/finalize-named-workset-sparse-scope-and-policy-limited.md`](../../../docs/workspace/m4/finalize-named-workset-sparse-scope-and-policy-limited.md)
- Typed consumer:
  `aureline_workspace::project_workset_scope_ux_lineage`

## What this packet proves

1. **Scope-class coverage truth.** Each record carries one row per
   governed scope declaring one closed `scope_kind`. A corpus
   missing any of the four required classes (`selected_workset`,
   `sparse_slice`, `policy_limited_view`, `full_workspace`) narrows
   the record with `required_scope_class_missing`. Worked example:
   [`baseline_workset_scope_ux_stable.json`](../../../fixtures/workspace/m4/workset_scope_ux_lineage/baseline_workset_scope_ux_stable.json).

2. **Surface coverage truth.** Each record carries one row per UX
   surface declaring one closed `surface_kind`. The required set is
   `workset_switcher`, `scope_chip`, `search`, `tree`, `graph`,
   `review`, and `support_export`. Optional surfaces
   (`refactor_scope_footer`, `ai_context_inspector`,
   `export_scope_footer`, `deep_link_dispatcher`) ride on top
   without changing the required set. Worked example:
   [`extended_with_optional_surfaces_stable.json`](../../../fixtures/workspace/m4/workset_scope_ux_lineage/extended_with_optional_surfaces_stable.json).

3. **Outside-vs-omitted distinction.** Every result-bearing surface
   (`search`, `tree`, `graph`, `review`) distinguishes
   `outside_current_slice`, `omitted_path`, and `policy_hidden`
   from `no_result`. Missing any of those narrows the record with
   `outside_marker_missing`, `omitted_marker_missing`, or
   `policy_hidden_marker_missing`, so a hidden member is never
   mislabeled as missing content.

4. **Hidden-result disclosure.** Every result-bearing and
   scope-labeling surface (and every export-propagating surface
   except the bare deep-link dispatcher) discloses the hidden-result
   count. Missing a disclosure narrows with
   `hidden_result_count_not_disclosed`.

5. **Slice-ref propagation.** Every surface carries the slice /
   profile ref into deep-link flows so reopened scopes land on the
   exact same identity. Export-propagating surfaces additionally
   carry the slice ref into export envelopes. Missing propagation
   narrows with `slice_ref_not_propagated_into_deep_links` or
   `slice_ref_not_propagated_into_export`.

6. **Widen-preview truth.** Every widen-scope preview previews
   hidden-result counts, omitted-root classes, fetch / deepen
   implications, and the consequences for blame / history / search
   completeness before any apply commits, and references a
   non-empty apply action id plus apply disclosure id. Missing any
   of the four preview disclosures narrows with
   `widen_preview_field_missing`; missing the apply metadata
   narrows with `apply_action_metadata_missing`. Worked example:
   [`widen_preview_missing_omitted_root_classes_narrowed.json`](../../../fixtures/workspace/m4/workset_scope_ux_lineage/widen_preview_missing_omitted_root_classes_narrowed.json)
   drops the omitted-root-classes preview and narrows below Stable.

7. **Widen-preservation truth.** Every widen preview preserves
   root identity, query / session continuity, and restore
   provenance. Losing any of those narrows with
   `widen_loses_root_identity`,
   `widen_loses_query_session_continuity`, or
   `widen_loses_restore_provenance`. The `preservation_posture`
   may declare `preserves_identity_and_continuity` or
   `preserved_after_review_with_disclosure`; a
   `creates_new_workspace_truth` posture is forbidden on Stable
   rows and surfaces the same narrow path so a widen never silently
   mints a second ambiguous workspace truth.

8. **Policy-limited disclosure truth.** Every
   `policy_limited_view` scope declares one closed
   `narrowing_cause`. A scope with `admin_policy` or
   `license_or_export_control` narrowing cause refuses to expose
   the hidden member list (`hidden_member_list_visible = false`);
   exposing it narrows with `policy_admin_hidden_list_exposed`. A
   policy-limited view missing the cause narrows with
   `policy_limited_narrowing_cause_missing`. Worked example:
   [`policy_limited_admin_redacted_hidden_list_stable.json`](../../../fixtures/workspace/m4/workset_scope_ux_lineage/policy_limited_admin_redacted_hidden_list_stable.json).

9. **Readiness truth.** Every scope row declares one closed
   `readiness_state`. A `ready` scope must disclose whether the
   hidden-result count is known; missing that disclosure narrows
   with `readiness_hidden_count_unknown` so consumers cannot
   conflate "fully indexed" with "no hidden members."

10. **Inspection precedes destructive widen.** The controlled
    inspection / repair hook table
    (`inspect_scope`, `compare_before_widen`, `preview_widen`,
    `export_scope`, `rollback_widen`, `repair_scope`) must be
    reachable before any destructive widen / narrow commits. A
    missing hook narrows with `inspection_hook_unavailable`. Worked
    example:
    [`missing_preview_widen_hook_narrowed.json`](../../../fixtures/workspace/m4/workset_scope_ux_lineage/missing_preview_widen_hook_narrowed.json)
    demonstrates the narrow path when `preview_widen` is
    unavailable.

11. **Support-export honesty.** Each row's support-export
    projection must preserve `scope_class`, `included_roots`,
    `hidden_result_count`, `narrowing_cause`, `readiness_state`,
    and `slice_ref`, and redact raw secrets, approval tickets,
    delegated credentials, live authority handles, and (for
    admin-policy or license-narrowed scopes) the hidden member
    list. Dropping a field narrows with
    `support_export_fields_dropped`; raising raw material narrows
    with `support_export_redaction_unsafe`.

12. **Producer attribution is pinnable for replay.** Each record
    carries the producer ref, the schema version, the capture
    timestamp, and an integrity hash derived from the input
    identities so replay and support pipelines can pin the source
    before applying. Incomplete attribution narrows with
    `producer_attribution_incomplete`.

13. **Lineage and export stay honest.** Every record sets
    `raw_payload_excluded = true` and carries only opaque refs to
    the source workspace, corpus, and producer. An empty workspace
    or corpus ref narrows with `lineage_export_unsafe`.

14. **The record is replay-gated.** The replay gate re-projects
    each fixture and asserts it equals the checked-in `expected`,
    so the projection cannot drift without failing CI.

## Fixture corpus

| Fixture                                                  | Workspace state covered                                                                  | Qualification           | Proves                                                                                                                  |
| -------------------------------------------------------- | ---------------------------------------------------------------------------------------- | ----------------------- | ----------------------------------------------------------------------------------------------------------------------- |
| `baseline_workset_scope_ux_stable`                       | Four required scope classes + seven required UX surfaces + two widen previews            | `stable`                | A baseline release-branch corpus can prove the full contract                                                            |
| `extended_with_optional_surfaces_stable`                 | Adds `current_repo` scope + refactor / AI-context / export / deep-link surfaces           | `stable`                | The optional scope classes and UX surfaces ride safely on the same projection                                           |
| `policy_limited_admin_redacted_hidden_list_stable`       | A policy-limited view narrowed by admin policy that redacts the hidden member list        | `stable`                | An admin / license narrowing remains Stable when the hidden member list is excluded across surfaces and exports         |
| `widen_preview_missing_omitted_root_classes_narrowed`    | A widen preview drops the omitted-root-classes disclosure                                | `narrowed_below_stable` | The contract refuses to ship Stable when a widen preview cannot truthfully preview implications before apply           |
| `missing_preview_widen_hook_narrowed`                    | `preview_widen` inspection hook unavailable                                              | `narrowed_below_stable` | The contract refuses to ship Stable when a required pre-action hook is missing                                          |

## How to verify

```sh
# Unit + replay gate for the workset / scope UX lineage projection.
cargo test -p aureline-workspace --lib workset_scope_ux_lineage
cargo test -p aureline-workspace --test workset_scope_ux_lineage_replay

# Headless emitter (JSON or --lines projection).
cargo run -p aureline-workspace --bin aureline_workset_scope_ux_lineage -- --lines \
  fixtures/workspace/m4/workset_scope_ux_lineage/baseline_workset_scope_ux_stable.json
```

## Stable-line registration

This lane's truth is the checked-in record, schema, fixtures, and
replay gate above. The lineage record self-describes its stable
qualification: postures that cannot prove the contract carry
`stable_qualification.level = narrowed_below_stable` with a named
reason, so they never inherit an adjacent green row. No public
scope is widened from this row.
