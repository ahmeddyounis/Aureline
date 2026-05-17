# Local-history timeline alpha report

Report id: `report:local-history-timeline:baseline`

Inputs:

- `fixtures/recovery/m3/local_history_timeline/manifest.yaml`
- `schemas/state/local_history_timeline_alpha.schema.json`
- `docs/state/m3/local_history_timeline_alpha.md`

Required fidelity labels:

| Label | Covered by |
|---|---|
| `exact` | `exact_snapshot_restore.yaml` |
| `compatible` | `compatible_schema_compare.yaml` |
| `layout_only` | `layout_only_placeholder.yaml` |
| `evidence_only` | `evidence_only_export.yaml` |

Required actions on every row:

| Action | Requirement |
|---|---|
| `compare` | Carries the same fidelity label as the row. |
| `restore` | Carries the same fidelity label as the row; writes a new checkpoint unless the row is evidence-only. |
| `export` | Carries the same fidelity label as the row and has a support-export ref. |

Safety baseline:

- Raw local-history payloads excluded.
- Raw private material excluded.
- Live authority and privilege handles excluded.
- Evidence-only rows do not claim live-session or privileged-run resumption.

Support-export consumer:

- `crates/aureline-support/src/local_history_timeline.rs`
- Emits `local_history_timeline_support_export_envelope`.
- Quotes the same exact, compatible, layout-only, and evidence-only labels as
  the timeline packet.
