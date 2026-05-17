# Local-history timeline alpha

This document freezes the alpha projection used by local-history compare,
restore, export, and support surfaces. It sits above the lower-level
`local_history_entry` records and does not duplicate raw snapshot bodies.

Companion artifacts:

- Schema: [`schemas/state/local_history_timeline_alpha.schema.json`](../../../schemas/state/local_history_timeline_alpha.schema.json)
- Fixtures: [`fixtures/recovery/m3/local_history_timeline/`](../../../fixtures/recovery/m3/local_history_timeline/)
- Report: [`artifacts/support/m3/local_history_timeline_alpha_report.md`](../../../artifacts/support/m3/local_history_timeline_alpha_report.md)
- Rust projection: [`crates/aureline-history/src/local_history/timeline.rs`](../../../crates/aureline-history/src/local_history/timeline.rs)
- Support export consumer: [`crates/aureline-support/src/local_history_timeline.rs`](../../../crates/aureline-support/src/local_history_timeline.rs)

## Vocabulary

Every row and every visible `compare`, `restore`, and `export` action carries
one shared fidelity label:

| Label | Meaning | Restore posture |
|---|---|---|
| `exact` | The captured checkpoint and current target are the same object and byte restore is available. | `exact_restore`; restore writes a new checkpoint. |
| `compatible` | The target is compatible but needs review because identity or schema changed. | `compatible_restore`; restore writes a new checkpoint after review. |
| `layout_only` | Only layout or placement state can be restored. | `layout_only`; execution surfaces remain ended or require rerun. |
| `evidence_only` | Only evidence, logs, or metadata can be reopened/exported. | `evidence_only`; restore is disabled and export remains available. |

The timeline intentionally omits `Recovered drafts` from this projection. Dirty
buffer recovery remains covered by the session-restore and crash-journal
records; this alpha lane is specifically for local-history compare, restore,
and evidence export.

## Row Contract

A timeline row must include:

- `source_entry_ref` and optional `source_group_ref`;
- `actor_lineage_class` inherited from the local-history actor-lineage packet;
- `target_posture` and `compare_basis`, which derive the row's fidelity label;
- exactly one `compare`, `restore`, and `export` action;
- a no-rerun guard that states whether the row is file-only, context-not-live,
  static evidence, or verified authority without hidden rerun;
- a metadata-safe support-export projection.

Restore actions that can write must name `new_checkpoint_on_restore_ref`.
Evidence-only restore actions must be `disabled_export_only`, must not write a
checkpoint, and must keep the evidence-only label visible.

## Support Export

Support export consumes the same Rust enums as the timeline packet. The support
envelope preserves row id, source checkpoint refs, fidelity labels, action
availability, and no-rerun flags while excluding raw payloads, private material,
and live authority handles. An evidence-only support row is non-conforming if it
claims `live_session_resumed` or `privileged_run_resumed`.

## Fixture Coverage

The protected corpus covers:

- exact byte restore for a normal file checkpoint;
- compatible restore after schema or identity drift;
- layout-only placeholder restore for an ended execution surface;
- evidence-only export for an omitted crash/recovery capture.

The evaluator rejects missing labels, missing actions, mismatched row/action
labels, missing restore checkpoint refs, unsafe support-export rows, and
evidence-only live-session claims.
