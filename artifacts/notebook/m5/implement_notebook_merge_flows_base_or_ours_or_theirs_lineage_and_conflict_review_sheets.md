# Artifact: Implement notebook merge flows, base or ours or theirs lineage, and conflict-review sheets

## Packet

- **Path**: `artifacts/notebook/m5/implement_notebook_merge_flows_base_or_ours_or_theirs_lineage_and_conflict_review_sheets.json`
- **Schema version**: 1
- **Record kind**: `notebook_merge_packet`
- **As of**: 2026-06-09T00:00:00Z

## Closed vocabularies

### Merge kinds

- `three_way_merge` — three-way notebook merge
- `fast_forward` — fast-forward merge
- `squash` — squashed merge
- `rebase` — rebase-style history rewrite
- `cherry_pick` — cherry-pick merge
- `revert` — revert-style merge

### Resolution strategies

- `cell_aware` — resolve at cell granularity
- `metadata_aware` — resolve at metadata-field granularity
- `raw_fallback` — side-by-side raw merge fallback

### Conflict classes

- `source_conflict` — conflicting cell source
- `metadata_conflict` — conflicting metadata field
- `output_conflict` — conflicting output
- `cell_deleted_both` — cell deleted in both branches
- `cell_added_both` — cell added independently in both branches
- `type_conflict` — conflicting cell type change

### Sheet actions

- `accept_ours` — accept the ours revision
- `accept_theirs` — accept the theirs revision
- `accept_base` — accept the base revision
- `mark_resolved` — mark the conflict resolved
- `edit_result` — edit the merge result manually
- `raw_merge` — fall back to raw merge for this cell
- `abort` — abort the merge flow

### Merge resolution classes

- `base` — resolved using base
- `ours` — resolved using ours
- `theirs` — resolved using theirs
- `result` — resolved as an edited result
- `unresolved` — still unresolved

## Invariants

1. A merge flow MUST carry non-empty `base_ref`, `ours_ref`, and `theirs_ref`.
2. When `unresolved_count > 0` and the resolution strategy is not `raw_fallback`,
   `result_ref` MUST be `null` because the merge result is incomplete.
3. A conflict-review sheet MUST offer at least one `available_actions` entry.
4. A conflict-review sheet MUST carry a non-empty `rollback_path_ref`.
5. Lineage records MUST carry non-empty `merge_flow_ref` and `cell_id_ref`.

## Downstream consumers

- `crates/aureline-notebook` — canonical record definitions and validators
- `crates/aureline-review` — merge/conflict review surface integration
- `crates/aureline-collab` — collaboration anchor and share-scope integration
- `docs/notebook/m5/implement_notebook_merge_flows_base_or_ours_or_theirs_lineage_and_conflict_review_sheets.md` — human-readable spec
