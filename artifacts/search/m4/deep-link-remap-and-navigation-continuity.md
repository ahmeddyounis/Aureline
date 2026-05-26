# Deep-link remap and navigation continuity — stable truth packet

This artifact is the human-readable companion to
`artifacts/search/m4/deep_link_navigation_truth_packet.json`. It documents the
stable knowledge-plane truth packet that binds deep-link remap decisions and
workspace navigation-continuity records to a single export-safe contract across
moved files, renamed symbols, and workspace changes.

## What the packet certifies

The packet is the canonical record that the following surfaces consume verbatim
on every claimed stable row — no surface reconstructs remap truth from raw
paths or arbitrary heuristics:

- **Search shell** quick-open, bookmark, and recent-place rows.
- **Workspace navigation history** (back/forward and session restore).
- **Docs/help** explainer of remap, drift, and continuity vocabulary.
- **AI context inspector** when deep-link rows feed AI workflows.
- **CLI / headless inspector** invoked from automation.
- **Support export** bundles.
- **Release proof index** entries.

## What it preserves

Every row carries the closed vocabularies required to keep remap and
continuity claims honest:

- **Outcome class:** `resolved_exact`, `remapped`, `recoverable_placeholder`,
  `failed_explicit_reason` — the packet's `covered_outcomes` must match the
  outcomes carried by its rows exactly.
- **Drift state:** `resolved_exact`, `resolved_remapped`, `resolved_ambiguous`,
  `target_missing`, `target_moved`, `target_renamed`, `target_branch_drifted`,
  `target_policy_blocked`, `target_scope_excluded`, `index_not_ready_for_target`,
  `unresolvable`.
- **Target kind:** `workspace_file`, `buffer_anchor`, `graph_symbol`,
  `graph_edge`, `docs_page_anchor`, `bookmark`, `recent_location`.
- **Confidence class:** `exact_identity`, `high_semantic`, `structural`,
  `heuristic`, `insufficient`, `unavailable`.
- **Evidence family:** `filesystem_identity`, `rename_move_history`,
  `symbol_stable_id`, `graph_remap_edge`, `planner_result_identity`,
  `user_curated_alias`, `path_similarity`, `indexed_candidate`.
- **Recovery action:** `open_remapped_target`, `preview_remapped_target`,
  `widen_scope`, `open_target_root`, `keep_current_scope`,
  `locate_missing_target`, `rebuild_index`, `inspect_remap_packet`,
  `remove_artifact`, `retry_after_index_ready`.
- **Failure reason:** `target_missing`, `target_scope_excluded`,
  `target_policy_blocked`, `root_unavailable`, `ambiguous_candidates`,
  `index_not_ready`, `workspace_mismatch`, `confidence_too_low`,
  `unsupported_target_kind`.
- **Destination visibility:** when the destination crosses the active root,
  workset, or workspace, rows carry one explicit identity row per surface
  (`peek`, `preview`, `split`, `open_in_new_pane`, `back_navigation`).

## Rows shipped in the checked-in packet

| Row | Outcome | Drift | Highlights |
|---|---|---|---|
| `row:resolved_exact:workspace_file` | `resolved_exact` | `resolved_exact` | Bookmark to a file whose canonical identity still resolves inside the active root. |
| `row:remapped:moved_file` | `remapped` | `resolved_remapped` | Bookmark follows a recorded file move via `filesystem_identity` + `rename_move_history`. |
| `row:recoverable_placeholder:cross_root` | `recoverable_placeholder` | `target_scope_excluded` | Back/forward target lives outside the active workset; placeholder ref + bounded recovery actions + cross-root destination visibility. |
| `row:failed_explicit_reason:missing` | `failed_explicit_reason` | `target_missing` | Bookmark to a retired module fails with `target_missing`, exposes `locate_missing_target` and `remove_artifact`. |

## Narrowing-below-stable findings

The validator emits a closed vocabulary of findings — any of them blocks stable
publication and forces the packet's `promotion_state` to `narrowed_below_stable`
or `blocks_stable`:

- `wrong_record_kind`, `wrong_schema_version`, `missing_identity`
- `invalid_remap_packet`, `invalid_continuity_record`
- `continuity_remap_packet_mismatch`, `continuity_action_drift`
- `destination_visibility_drift`, `destination_visibility_dropped`
- `drift_state_dropped`, `recovery_action_vocabulary_dropped`,
  `evidence_class_dropped`
- `raw_boundary_material_present`
- `outcome_coverage_dropped`, `drift_state_coverage_dropped`,
  `outcome_coverage_over_declared`, `drift_state_coverage_over_declared`
- `missing_consumer_projection`, `consumer_projection_drift`
- `projection_drift_state_dropped`, `projection_recovery_action_dropped`,
  `projection_confidence_evidence_dropped`,
  `projection_destination_visibility_dropped`
- `promotion_state_mismatch`

## How to verify locally

```
cargo test -p aureline-search --test deep_link_navigation_truth_cases
```

The fixture corpus under
`fixtures/search/m4/deep_link_navigation_truth_packet/` proves the baseline
stable posture plus the narrowed-below-stable postures, and the test asserts
that the checked-in packet validates with no findings and preserves every
required consumer projection.
