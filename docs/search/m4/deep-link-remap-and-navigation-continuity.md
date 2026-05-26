# Deep-link remap and navigation continuity — reviewer doc

This is the reviewer-facing contract for the stable M4 lane that ships
**deep-link remap and navigation-continuity truth across moved files, renamed
symbols, and workspace changes** as one knowledge-plane packet every consumer
surface reads instead of reconstructing remap truth from raw paths.

The packet lives at
`artifacts/search/m4/deep_link_navigation_truth_packet.json`. Its boundary
schema lives at
`schemas/search/deep_link_navigation_truth_packet.schema.json`. The Rust
contract that materializes, validates, and emits support exports lives at
`crates/aureline-search/src/deep_link_navigation_truth/mod.rs`.

## Why this row exists

Search results, bookmarks, navigation-history entries, recent-place pickers,
and AI context handoffs all need a single decision record to answer "what
happens when this saved target moved, was renamed, or now lives outside the
active workset?" — without leaking raw paths, secrets, or destination
contents. Before this row was certified, surfaces reconstructed remap truth
from row-local state and quietly drifted: cross-root targets disappeared into
generic "not found" banners, recovery actions diverged between bookmark
panes and back/forward chrome, and confidence/evidence vocabularies were
flattened into a single "best guess" badge.

The certified packet pins:

- The deep-link **remap decision** (`DeepLinkRemapPacket`): old/new target
  identity, drift state, active scope/workset, confidence class, evidence
  families, recovery actions, and destination-visibility rows.
- The workspace **continuity record** (`NavigationContinuityRecord`):
  durable bookmark or history artifact id, continuity state, placeholder ref
  when the row remains visible, and the recovery vocabulary the surface may
  offer.
- The **glue invariants** every surface must honour: continuity must cite
  the embedded remap packet; recovery actions and destination visibility
  must agree between remap and continuity; cross-boundary rows must publish
  identity on peek/preview/split/open/back surfaces; and every consumer
  projection must preserve drift state, recovery vocabulary, confidence,
  evidence, and destination visibility verbatim.

## Closed vocabularies

| Vocabulary | Tokens |
|---|---|
| Outcome | `resolved_exact`, `remapped`, `recoverable_placeholder`, `failed_explicit_reason` |
| Drift state | `resolved_exact`, `resolved_remapped`, `resolved_ambiguous`, `target_missing`, `target_moved`, `target_renamed`, `target_branch_drifted`, `target_policy_blocked`, `target_scope_excluded`, `index_not_ready_for_target`, `unresolvable` |
| Confidence | `exact_identity`, `high_semantic`, `structural`, `heuristic`, `insufficient`, `unavailable` |
| Evidence | `filesystem_identity`, `rename_move_history`, `symbol_stable_id`, `graph_remap_edge`, `planner_result_identity`, `user_curated_alias`, `path_similarity`, `indexed_candidate` |
| Recovery action | `open_remapped_target`, `preview_remapped_target`, `widen_scope`, `open_target_root`, `keep_current_scope`, `locate_missing_target`, `rebuild_index`, `inspect_remap_packet`, `remove_artifact`, `retry_after_index_ready` |
| Failure reason | `target_missing`, `target_scope_excluded`, `target_policy_blocked`, `root_unavailable`, `ambiguous_candidates`, `index_not_ready`, `workspace_mismatch`, `confidence_too_low`, `unsupported_target_kind` |
| Consumer surface | `search_shell`, `navigation_history`, `docs_help`, `ai_context_inspector`, `cli_headless`, `support_export`, `release_proof_index` |

## Validation findings that block stable

The packet validator narrows below stable on any of:

- `wrong_record_kind`, `wrong_schema_version`, `missing_identity`
- `invalid_remap_packet`, `invalid_continuity_record`
- `continuity_remap_packet_mismatch` (continuity must cite its embedded
  remap packet)
- `continuity_action_drift` (continuity recovery actions must equal the
  remap packet's recovery actions)
- `destination_visibility_drift` (destination visibility must agree between
  remap and continuity)
- `destination_visibility_dropped` (cross-boundary row missing destination
  visibility on a required surface)
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
stable posture plus narrowed-below-stable postures (continuity mismatch,
missing projection, projection drops drift vocabulary, outcome coverage
over-declared, destination visibility drift). The checked-in artifact is
re-validated by the same test so the published packet stays canonical.
