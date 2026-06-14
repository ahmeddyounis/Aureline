# M5 drag/drop transfer safety: verb disclosure, insertion indicators, cross-window detach, and long-transfer progress

This contract makes drag-and-drop behavior an explicit, reviewable product
surface across the new Milestone 5 transfer surfaces — notebook cells, data/API
result rows, artifacts, work items, and preview-runtime assets, plus the editor
core baseline — instead of ad hoc per-surface drag behavior. It binds the
drag/drop half of the frozen
[keyboard-continuity matrix](./freeze-the-m5-keyboard-mode-modal-sequence-clipboard-route-drag-drop-verb-and-grouped-hist.md):
*drag/drop never hides verbs or scope*. A drop advertises the resulting verb and
insertion point before it commits, preserves context and recovery when it detaches
across windows, and tracks progress / cancellation / a post-action summary for
large transfers.

The canonical packet is built by
`aureline_shell::add_drag_and_drop_verb_disclosure_insertion_indicators_cross_window_detach_and_long_transf`.
Its boundary schema is
[`schemas/interaction/add-drag-and-drop-verb-disclosure-insertion-indicators-cross-window-detach-and-long-transf.schema.json`](../../../schemas/interaction/add-drag-and-drop-verb-disclosure-insertion-indicators-cross-window-detach-and-long-transf.schema.json),
the checked support export is
[`artifacts/interaction/m5/add-drag-and-drop-verb-disclosure-insertion-indicators-cross-window-detach-and-long-transf/support_export.json`](../../../artifacts/interaction/m5/add-drag-and-drop-verb-disclosure-insertion-indicators-cross-window-detach-and-long-transf/support_export.json),
and the protected fixtures live under
[`fixtures/interaction/m5/add-drag-and-drop-verb-disclosure-insertion-indicators-cross-window-detach-and-long-transf/`](../../../fixtures/interaction/m5/add-drag-and-drop-verb-disclosure-insertion-indicators-cross-window-detach-and-long-transf/).

## What one record binds

A `TransferSafetyRecord` binds one claimed M5 surface (keyed by a
`KeyboardSurfaceKind` and a non-display `KeyboardSurfaceSubject`) to one drop
attempt:

- the **dragged object** (`TransferObjectRef`): a `TransferObjectClass`
  (`notebook_cell`, `result_grid_row`, `artifact_item`, `work_item`,
  `preview_runtime_asset`, `editor_text_fragment`) plus an opaque or
  workspace-relative `object_token` — never an absolute private path;
- the resolved **drop verb** (`DragDropVerbClass`, reused from the frozen matrix):
  `move_verb_explicit`, `copy_verb_explicit`, `link_verb_explicit`,
  `verb_choice_on_modifier`, or the denied `destructive_default_denied`;
- the **window scope** (`WindowScopeClass`): `same_pane_reorder`,
  `cross_pane_same_window`, `cross_window_detach`, or `reattach_to_window`;
- the payload **magnitude** (`TransferMagnitudeClass`): `trivial_inline` or
  `large_needs_progress`;
- whether the **verb and insertion point** were disclosed before commit;
- a reopenable **verification proof** (`AxisVerification`); and
- the resolved **transfer-safety posture** (`TransferDisclosureClass`).

## Transfer postures and the safety floor

`TransferDisclosureClass` is the canonical transfer-disclosure vocabulary that
product, help, migration, and support all name:

| Resolution | Rank | Meaning |
| --- | --- | --- |
| `disclosed_inline_commit` | 0 | The verb and insertion point are shown before an inline, instantaneous commit in the same context. |
| `explicit_verb_choice_disclosed` | 1 | The resulting verb (move / copy / import / open / split) is explicitly chosen and advertised before commit. |
| `cross_window_continuity_preserved` | 2 | A cross-window detach / reattach preserves context and recovery rather than orphaning the tab or asset. |
| `progress_cancel_summary_tracked` | 3 | A large transfer shows progress, a cancel control, and a post-action summary. |
| `confirmed_before_mutation` | 4 | A destructive / import semantic is confirmed (verb and scope) before the destination state mutates. |
| `rejected_ambiguous_or_unsafe` | 5 | An ambiguous, destructive-default, or otherwise unsafe transfer is rejected. |

Only `disclosed_inline_commit` commits inline without escalation. A drop is held
off that inline lane whenever a `TransferContractTrigger` fires, and each trigger
imposes a minimum resolution rank (the **floor**):

| Trigger | Floor |
| --- | --- |
| `ambiguous_or_multi_verb` | `explicit_verb_choice_disclosed` |
| `stale_or_missing_transfer_proof` | `explicit_verb_choice_disclosed` |
| `cross_window_detach` | `cross_window_continuity_preserved` |
| `large_transfer_magnitude` | `progress_cancel_summary_tracked` |
| `destructive_or_import_semantics` | `confirmed_before_mutation` |
| `destructive_default_verb` | `rejected_ambiguous_or_unsafe` |

The recorded trigger set must equal the set computed from the record's verb,
window scope, magnitude, import/destructive flag, and proof, and the resolution
must meet the floor. The resolution must carry exactly the detail field it
requires — a `verb_choice_label`, `continuity_note`, `progress_note`,
`confirmation_label`, or `rejection_reason_label` — and that label may not be a
generic non-answer (`moved`, `copied`, `imported`, `rejected`, …).

## Invariants enforced by `validate`

- **Verb disclosed before commit.** Every record advertises the resulting verb
  before the drop commits; a record that hides its verb is rejected.
- **Insertion point disclosed before commit.** Every committing record shows its
  insertion point before commit; only a rejected drop — which never reaches an
  insertion point — may omit it.
- **Never destructive or ambiguous by default.** A `destructive_default_denied`
  verb raises the floor to a reject; a non-trivial drop can never commit inline.
- **Cross-window continuity.** A detach / reattach preserves context and offers a
  recovery path rather than orphaning a tab or asset (`orphaned_on_detach` is
  always false).
- **Long transfers are observable.** A large transfer shows progress, a cancel
  control, and a post-action summary rather than freezing into a generic spinner.
- **Provider transfers never read as local.** A provider-linked or imported
  surface carries imported proof and never reads as a locally verified transfer.
- **No raw boundary material.** Raw drag payload byte buffers, raw provider
  payloads, file contents, and absolute private paths never cross this boundary.

## Coverage the packet must prove

The seeded packet represents the six core transfer surfaces (`editor_core`,
`notebook_surface`, `data_api_surface`, `preview_surface`, `review_surface`,
`runtime_surface`), the five parity-critical object classes (`notebook_cell`,
`result_grid_row`, `artifact_item`, `work_item`, `preview_runtime_asset`), every
resolution class, at least one clean inline-commit baseline, at least one drop
forced off the inline lane, at least one cross-window continuity record, at least
one long-transfer progress record, and at least one provider-linked / imported
transfer.

## Consumers

The packet is the canonical source so product, help / migration guidance, support
export, and release-control surfaces name the same transfer verbs and disclosure
classes the product actually exposes — rather than re-deriving drag/drop semantics
per surface.
