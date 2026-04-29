# Restore artifact family fixtures

These fixtures are short, reviewable scenarios that anchor the
restore-fidelity vocabulary frozen in
[`/docs/state/restore_artifact_family_contract.md`](../../../docs/state/restore_artifact_family_contract.md)
and validated by the workspace-authority checkpoint and window-topology
snapshot schemas:

- [`/schemas/state/workspace_authority_checkpoint.schema.json`](../../../schemas/state/workspace_authority_checkpoint.schema.json)
- [`/schemas/state/window_topology_snapshot.schema.json`](../../../schemas/state/window_topology_snapshot.schema.json)

Each fixture is one record body. Authority checkpoints and topology
snapshot packets are paired by `workspace_authority_checkpoint_ref`
(or, equivalently, by `checkpoint_id`) so a reviewer can read the
authority claim and the topology claim side by side without flattening
authority and view topology into one opaque payload.

## Scope rules

- Fixtures validate against the family schemas; they do not encode raw
  pane-tree bodies, raw secrets, raw absolute paths, raw command lines,
  raw logs, or raw source content. The recursive pane-tree body is
  validated by the workspace-shell pane-tree schema and is referenced
  here by `pane_tree_record_ref` only.
- A new fixture MUST exercise at least one restore-fidelity class
  (`exact_restore`, `compatible_restore`, `layout_only`,
  `recovered_drafts`, `evidence_only`, or `no_restore`), and MUST cite
  the motivating section of the family contract.
- Opaque ids and monotonic timestamps are chosen for review clarity
  rather than to mirror a real machine.
- Excluded live-authority classes stay explicit instead of vanishing
  silently. Placeholder behaviors and topology adjustments stay
  inventory-shaped so support can explain what the user is seeing.

## Index

| Fixture | Body | Restore class | Key coverage |
|---|---|---|---|
| [`exact_restore_authority_checkpoint.yaml`](./exact_restore_authority_checkpoint.yaml) | workspace-authority checkpoint | `exact_restore` | trusted roots, active worksets, dirty-buffer journal identities round-trip; live-authority classes stay excluded |
| [`exact_restore_window_topology_snapshot.yaml`](./exact_restore_window_topology_snapshot.yaml) | window-topology snapshot | `exact_restore` | pane identity, tab order, inspectors, focus chain, follow state, monitor affinity rehydrate without placeholders or adjustments |
| [`compatible_restore_authority_checkpoint.yaml`](./compatible_restore_authority_checkpoint.yaml) | workspace-authority checkpoint | `compatible_restore` | producer schema translation, rollback checkpoint, preserved prior artifact |
| [`layout_only_window_topology_snapshot.yaml`](./layout_only_window_topology_snapshot.yaml) | window-topology snapshot | `layout_only` | missing extension, missing remote, expired remote authority, display drift; placeholder rows and recenter / re-dock / safe-bounds topology adjustments |
| [`recovered_drafts_authority_checkpoint.yaml`](./recovered_drafts_authority_checkpoint.yaml) | workspace-authority checkpoint | `recovered_drafts` | dirty-buffer rehydration as drafts; preserved on-disk artifacts; rollback checkpoint |
| [`evidence_only_window_topology_snapshot.yaml`](./evidence_only_window_topology_snapshot.yaml) | window-topology snapshot | `evidence_only` | every live surface recorded as evidence; no rerun, no silent authority reacquisition, no monitor-affinity claim |

## Coverage contract

The shared fixture set MUST keep:

- at least one case for each restore-fidelity class
  (`exact_restore`, `compatible_restore`, `layout_only`,
  `recovered_drafts`, `evidence_only`);
- at least one paired case where the workspace-authority checkpoint
  and the window-topology snapshot agree on `exact_restore`;
- at least one case that exercises every closed
  `placeholder_reason_class` listed in the family contract over the
  course of the corpus (`missing_extension`, `missing_remote`,
  `missing_remote_authority`, `revoked_permission`,
  `unsupported_display_topology`, `non_reentrant_live_surface`,
  `schema_migration_review_required`, `manual_recovery_required`);
- at least one case that exercises every closed
  `topology_adjustment_class` (`snapped_to_safe_bounds`,
  `moved_to_primary_display`, `scale_normalized`,
  `fullscreen_cleared`, `stacking_repaired`,
  `recentered_to_visible_region`, `redocked_to_safe_pane`); and
- at least one case where the authority checkpoint declares an
  `excluded_live_authority_classes[]` floor that the topology packet
  inherits by reference instead of restating.
