# M5 Anchor-Remap History Set

- Packet: `m5-anchor-remap-history:stable:0001`
- Label: `M5 Anchor-Remap History Set`
- Workspace: `workspace:m5:anchor-remap`
- Minted: `2026-06-19T00:00:00Z`
- Histories: 5
- Drift lanes covered: 5

| History | Anchor family | Entries | Current state | Current anchor | Lanes | Disclosure |
| --- | --- | --- | --- | --- | --- | --- |
| `history:m5:file-edit:0001` | `anchor-family:m5:file-edit:0001` | 2 | contextual | anchor:file-edit:rev1 | file_edit | true |
| `history:m5:notebook-cell:0001` | `anchor-family:m5:notebook-cell:0001` | 2 | stale | anchor:notebook-cell:cell-a-stale | notebook_cell_identity_change | true |
| `history:m5:generated-artifact:0001` | `anchor-family:m5:generated-artifact:0001` | 2 | unmapped | (dropped) | generated_artifact_churn | true |
| `history:m5:imported-snapshot:0001` | `anchor-family:m5:imported-snapshot:0001` | 2 | contextual | anchor:imported-snapshot:mapped | imported_snapshot_comparison | true |
| `history:m5:imported-replay:0001` | `anchor-family:m5:imported-replay:0001` | 1 | imported_static | anchor:imported-replay:static | imported_replay_comparison | true |

- `history:m5:file-edit:0001` — A file edit moved the anchored range; the finding now only contextually survives. (diagnostic:m5:file-edit:0001)
  - [0] file_edit / exact_range_preserved / exact → anchor:file-edit:rev0
  - [1] file_edit / surrounding_context_match / contextual → anchor:file-edit:rev1
- `history:m5:notebook-cell:0001` — A notebook cell identity change left no fresh mapping; the finding is retained against a stale epoch. (diagnostic:m5:notebook-cell:0001)
  - [0] notebook_cell_identity_change / exact_range_preserved / exact → anchor:notebook-cell:cell-a
  - [1] notebook_cell_identity_change / stale_epoch_retained / stale → anchor:notebook-cell:cell-a-stale
- `history:m5:generated-artifact:0001` — Generated-artifact churn dropped the anchored region; the finding is now unmapped, not silently discarded. (diagnostic:m5:generated-artifact:0001)
  - [0] generated_artifact_churn / exact_range_preserved / exact → anchor:generated:region-1
  - [1] generated_artifact_churn / no_mapping_found / unmapped → (dropped)
- `history:m5:imported-snapshot:0001` — An imported snapshot's static location was compared against a later local revision and mapped contextually. (diagnostic:m5:imported-snapshot:0001)
  - [0] imported_snapshot_comparison / imported_static_location / imported_static → anchor:imported-snapshot:static
  - [1] imported_snapshot_comparison / surrounding_context_match / contextual → anchor:imported-snapshot:mapped
- `history:m5:imported-replay:0001` — A replayed support bundle carries an imported-static location preserved as snapshot-only evidence. (diagnostic:m5:imported-replay:0001)
  - [0] imported_replay_comparison / imported_static_location / imported_static → anchor:imported-replay:static
