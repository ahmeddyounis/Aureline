# M5 keyboard-mode / clipboard-route / drag-drop-verb / grouped-history continuity matrix

Aureline's switching promise depends on keyboard-first, recoverable interaction
across **every** new M5 surface — editor, notebook, data/API, preview, docs,
review, runtime, and companion-adjacent panes — not just the original editor
core. This contract freezes those expectations into one machine-readable matrix
so modal-mode state, leader-key sequences, clipboard/register routes, drag/drop
verbs, grouped undo/history classes, reopen/recover paths, and orientation aids
are explicit product contracts instead of ad hoc per-surface behavior.

The canonical record is the
[`KeyboardContinuityMatrixPacket`](../../../crates/aureline-shell/src/freeze_the_m5_keyboard_mode_modal_sequence_clipboard_route_drag_drop_verb_and_grouped_history_matrix/mod.rs)
in `aureline-shell`. Product, help/migration, accessibility, diagnostics, and
release-control surfaces ingest this one packet rather than cloning
switching-wedge language per feature.

## Boundary

- Schema: [`schemas/interaction/freeze-the-m5-keyboard-mode-modal-sequence-clipboard-route-drag-drop-verb-and-grouped-hist.schema.json`](../../../schemas/interaction/freeze-the-m5-keyboard-mode-modal-sequence-clipboard-route-drag-drop-verb-and-grouped-hist.schema.json)
- Support export: [`artifacts/interaction/m5/freeze-the-m5-keyboard-mode-modal-sequence-clipboard-route-drag-drop-verb-and-grouped-hist/support_export.json`](../../../artifacts/interaction/m5/freeze-the-m5-keyboard-mode-modal-sequence-clipboard-route-drag-drop-verb-and-grouped-hist/support_export.json)
- Markdown summary: [`artifacts/interaction/m5/freeze-the-m5-keyboard-mode-modal-sequence-clipboard-route-drag-drop-verb-and-grouped-hist.md`](../../../artifacts/interaction/m5/freeze-the-m5-keyboard-mode-modal-sequence-clipboard-route-drag-drop-verb-and-grouped-hist.md)
- Fixtures: [`fixtures/interaction/m5/freeze-the-m5-keyboard-mode-modal-sequence-clipboard-route-drag-drop-verb-and-grouped-hist/`](../../../fixtures/interaction/m5/freeze-the-m5-keyboard-mode-modal-sequence-clipboard-route-drag-drop-verb-and-grouped-hist/)

Raw clipboard bodies, raw key buffers, raw provider payloads, file contents,
private paths, and credentials never cross this boundary; the packet carries only
typed class tokens, booleans, opaque ids, fingerprint digests, and
redaction-aware reviewable labels.

## Canonical vocabulary

Every claimed M5 surface row identifies its behavior on each of the following
axes. The axes are the canonical names downstream surfaces must reuse.

| Axis | Field | Honest states | Downgraded / denied state |
| --- | --- | --- | --- |
| Mode strip | `mode_strip` | `modal_parity_complete`, `modal_read_only_navigation`, `non_modal_keyboard_complete` | `mode_unsupported_downgraded` |
| Sequence guide | `sequence_guide` | `leader_sequence_complete`, `prefix_discoverable`, `single_stroke_only` | `sequence_unsupported_downgraded` |
| Clipboard route | `clipboard_route` | `plain_text_preserved`, `rich_with_plain_fallback`, `named_register_routed`, `sensitive_copy_warned` | `rich_only_denied` |
| Drag/drop verb | `drag_drop_verb` | `move_verb_explicit`, `copy_verb_explicit`, `link_verb_explicit`, `verb_choice_on_modifier` | `destructive_default_denied` |
| Undo class | `undo_class` | `exact_undo`, `grouped_exact_undo`, `compensating_undo`, `checkpoint_restore`, `no_undo_honest` | — (never flattened) |
| History class | `history_class` | `linear_step_history`, `grouped_step_history`, `branching_history`, `checkpoint_lineage`, `cross_surface_continuity` | — |
| Reopen / recover | `reopen_recover` | `exact_reopen`, `recovered_approximate`, `checkpoint_recover`, `reopen_unavailable_honest` | — (absence is honest) |
| Orientation aid | `orientation_aid` | `full_orientation_aids`, `reduced_orientation_aids_honest`, `orientation_aids_degraded_honest` | `orientation_aids_absent_downgraded` |

Each row also records whether the surface stays `keyboard_complete`, whether its
mode changes and macro replay are `macro_replay_explicit`, and a reopenable
`verification` proof (`proof_currency` plus a `proof_ref` keyed by a non-display
fingerprint).

## Auto-downgrade

A row claims a `continuity_parity_grade` and carries an `effective_grade`. The
row **auto-downgrades** — the effective grade ranks strictly below the claim,
with a recorded `downgrade_trigger` and a precise `downgraded_label` — whenever:

- any axis is in its downgraded / denied state (an unsupported modal sequence, a
  rich-only clipboard route, a destructive-by-default drop verb, collapsed
  orientation aids);
- the surface stops being keyboard-complete or its macro replay stops being
  explicit; or
- its verification proof is stale, missing, review-pending, or imported proof
  standing in for a local-surface claim.

A claimed switching or power-user row that cannot identify its keyboard,
clipboard, or history behavior therefore narrows below its claim **before
promotion** rather than coasting on the editor core's parity. The downgrade is
honest and precise: a generic "unavailable" label is rejected.

## Guardrails

- Unsupported modal sequences are **never** silently approximated — they
  downgrade honestly.
- Copy/export always preserves a useful plain-text representation; rich text is
  never the only copy representation (`rich_only_denied` forces a downgrade).
- Drag/drop never defaults to a destructive or ambiguous verb; the verb is always
  explicit.
- Exact undo, grouped exact undo, compensating action, and checkpoint restore
  stay distinct and are **never** flattened into one vague history label.
- Orientation aids degrade honestly rather than collapsing silently.
- A provider-linked / imported surface never reads as a locally verified surface;
  its parity rests on current imported proof, which backs the imported claim but
  never a local one.

## Out of scope

This matrix does not introduce a new macro language or a new editor core. It
freezes only the parity and safety controls needed so claimed M5 surfaces
preserve keyboard-first, copy/drop, and recovery behavior consistent with
Aureline's switching promise.
