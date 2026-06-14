# M5 grouped-history continuity: named undo groups, exact-versus-compensating recovery labels, back/forward continuity, and reopen-closed affordances

This contract makes undo, recovery, navigation history, and reopen / recover
behavior an explicit, reviewable product surface across the new Milestone 5
mutation and navigation surfaces — notebook reformats, data/API result sets, docs
publishes, preview navigations, review threads, runtime sessions, and provider-
linked companion threads, plus the editor-core baseline — instead of one opaque
`Undo` or generic `Back`. It binds the grouped-history half of the frozen
[keyboard-continuity matrix](./freeze-the-m5-keyboard-mode-modal-sequence-clipboard-route-drag-drop-verb-and-grouped-hist.md):
*history distinguishes exact undo, compensating undo, and checkpoint restore*, and
*reopen / recover paths degrade honestly*. A history entry names its undo class,
attributes its source, summarizes the objects it touched, preserves back/forward
identity where navigation matters, and distinguishes an intentionally closed
surface from one lost to a crash or disconnect.

The canonical packet is built by
`aureline_shell::ship_named_undo_groups_exact_versus_compensating_recovery_labels_back_forward_history_cont`.

## What each record binds

A `HistoryContinuityRecord` binds one claimed M5 surface (keyed by a
`KeyboardSurfaceKind` and a non-display `KeyboardSurfaceSubject`) to one history
entry:

- the `HistoryEntryKind` — whether the entry is a **mutation** (a change to be
  undone / recovered) or a **navigation** (a move whose back/forward target
  identity matters);
- the `AffectedObjectSummary` — the object class, an opaque / workspace-relative
  token, the count of objects and files touched, and a reviewable label, so a
  grouped history names *what* changed rather than collapsing into opaque payload;
- the `SourceAttributionClass` — who or what produced the entry (user, automation
  agent, provider sync, generated-from-source, or checkpoint subsystem);
- the canonical `UndoClass`, `HistoryClass`, and `ReopenRecoverClass` from the
  frozen matrix vocabulary;
- the `SurfaceLossCause` — whether the tied surface is open, intentionally closed,
  or lost to a crash / disconnect — plus an optional close timestamp;
- a reopenable `AxisVerification` proof and the resolved `RecoveryAffordanceClass`.

## Triggers and the minimum-safety floor

A history entry is **never flattened into one opaque label when it is
consequential**. Each condition below fires a `HistoryContractTrigger` that
imposes a minimum-safety floor on the resolution, so the record can never resolve
to a flat `exact_step_undo` when a trigger fires:

| Trigger | Condition | Minimum resolution |
| --- | --- | --- |
| `broad_multi_object_mutation` | a mutation touches >1 object or file | `named_group_exact_undo` |
| `cross_surface_navigation` | a navigation crosses surfaces | `back_forward_continuity_preserved` |
| `not_literally_invertible` | a mutation has no literal inverse | `compensating_action_labeled` |
| `generated_or_automated_change` | the entry was generated / automated | `regenerate_from_source` |
| `checkpoint_only_recovery` | recovery is only possible from a checkpoint | `checkpoint_restore_labeled` |
| `surface_closed_or_lost` | the tied surface is closed / lost | `reopen_or_recover_labeled` |
| `stale_or_missing_history_proof` | the history proof is stale / missing | `named_group_exact_undo` |

The required floor is the maximum over all fired triggers. A record that flattens
a consequential entry into the bare exact-step baseline, or that resolves below
its required floor, fails `HistoryContinuityPacket::validate`.

## Recovery affordances stay distinct

The `RecoveryAffordanceClass` ladder keeps exact undo, named group undo,
back/forward continuity, compensating action, regenerate-from-source, checkpoint
restore, and reopen / recover **distinct**. Each non-baseline resolution must cite
exactly the precise label it requires — a generic non-answer (`undo`, `back`,
`group`, `restored`, …) is rejected. Support and export surfaces reconstruct the
exact-versus-compensating distinction from the `undo_class` (`exact_undo` /
`grouped_exact_undo` versus `compensating_undo` / `checkpoint_restore`) cross-
referenced with the resolution.

## Reopen / recover distinguishes intentional close from loss

A reopen / recover always names its `SurfaceLossCause`: an `intentional_close`
reopens the surface exactly, while a `crash_loss` or `disconnect_loss` recovers an
approximate state and says so. A close timestamp is preserved where useful. A
closed / lost surface must carry a real loss cause and a close timestamp; an open
surface carries neither. A provider-linked surface is verified with imported proof
and never reads as a locally verified history.

## Boundary safety

Raw provider payloads, file contents, and absolute private paths never cross this
boundary. The packet carries only typed class tokens, booleans, opaque / relative
ids, fingerprint digests, and redaction-aware reviewable labels.

## Artifacts

- Schema:
  `schemas/interaction/ship-named-undo-groups-exact-versus-compensating-recovery-labels-back-forward-history-cont.schema.json`
- Support export:
  `artifacts/interaction/m5/ship-named-undo-groups-exact-versus-compensating-recovery-labels-back-forward-history-cont/support_export.json`
- Markdown summary:
  `artifacts/interaction/m5/ship-named-undo-groups-exact-versus-compensating-recovery-labels-back-forward-history-cont.md`
- Protected fixtures:
  `fixtures/interaction/m5/ship-named-undo-groups-exact-versus-compensating-recovery-labels-back-forward-history-cont/`

Regenerate the checked artifacts with
`cargo run -p aureline-shell --example dump_history_continuity_recovery [support|summary|fixture]`.
