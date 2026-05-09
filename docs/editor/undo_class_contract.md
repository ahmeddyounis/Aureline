# Undo/redo lineage contract

This document is the reviewer-facing contract for Aureline’s undo/redo
lineage on the editor hot path: every committed mutation is attributable to a
named undo class and a stable actor/source identifier so surfaces can explain
what an undo step represents.

Normative design sources:

- `docs/adr/0003-buffer-undo-large-file.md` (buffer undo journal + frozen undo
  class taxonomy).
- `.t2/docs/Aureline_Technical_Architecture_Document.md` §12.4.3 (unified
  mutation lineage + undo-class posture).
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` §9.10 (clipboard/undo/history
  expectations).

## Scope

In scope:

- the stable metadata carried by each committed undo group:
  - undo class (`UndoClass` + `class_id`),
  - compensation posture (`CompensationPosture`),
  - actor/source lineage (`originator`), and
  - optional human-readable labels for named groups and selected commands;
- editor helpers for projecting buffer journal entries into UI-ready summaries;
- one live shell surface that exercises undo/redo (typing/paste/multi-cursor)
  plus an external reload path that is itself undoable.

Out of scope:

- durable mutation-journal persistence and retention policy;
- local history timeline UI and checkpoint storage;
- compensating/regenerate/restore class execution beyond the buffer’s current
  `CompensationPosture` split.

## Canonical surface

The buffer journal is canonical and lives under:

- `crates/aureline-buffer/src/piece_tree/`

Editor-facing undo projections live under:

- `crates/aureline-editor/src/undo/mod.rs`

The protected consumer path is the native shell:

- `crates/aureline-shell/src/bootstrap/native_shell.rs`

## Undo groups and metadata

Each committed buffer transaction becomes one undo group. A group carries:

- `UndoClass`: the frozen taxonomy (for example `text_edit`, `external_reload`);
- `CompensationPosture`: whether redo can survive divergence
  (`compensatable` vs `only_revertible`);
- `originator`: a stable string describing who/what initiated the change; and
- `label` (optional): a human-readable name used for named-group classes and
  select commands that need an inspectable description.

Surfaces must not infer undo semantics from UI context alone. They should read
the group metadata from the buffer journal and present it directly.

## Originator strings (actor/source lineage)

The editor crate defines stable originator identifiers for common user-facing
actions under `aureline_editor::undo::originator`:

- `user_keystroke`: typing/backspace/delete edits
- `paste`: clipboard paste edits
- `command:editor.cut`: cut (clipboard + delete)
- `command:editor.externalChange.reload`: reload a clean buffer from disk

Multi-cursor edits keep the same base originator but add a suffix so downstream
history consumers can distinguish them:

- `:multi_cursor`

## Querying the next undo/redo action

Consumers that need to display “what will undo do?” should not guess based on
the last command. Instead, they should consult the buffer journal:

- `Buffer::peek_undo()` / `Buffer::peek_redo()` expose the next undo/redo group
  without mutating state.
- `aureline_editor::undo::next_undo()` / `next_redo()` project those entries
  into `UndoGroupSummary` for UI/structured logging.

`UndoGroupSummary::label_or_class_id()` provides a human-oriented label when
present, otherwise falling back to the undo-class id.

## Protected shell flow

The native shell wires undo/redo on the live editor path:

- Undo: `Ctrl/Cmd+Z`
- Redo: `Ctrl/Cmd+Shift+Z` (plus `Ctrl/Cmd+Y` as an alternate redo binding)

The shell logs the undo/redo summary as `undo: <label-or-class> (<class_id>)`
so the lineage remains inspectable during dogfood and failure drills.

The shell also exposes an external reload command:

- Reload from disk (clean buffers only): `Ctrl/Cmd+R`

Reload is recorded as an `external_reload` undo group with a stable originator
and a human label, and it is itself undoable.

