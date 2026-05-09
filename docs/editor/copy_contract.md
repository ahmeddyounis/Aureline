# Editor copy/cut/paste clipboard contract (groundwork)

This document describes the baseline copy/cut/paste behavior for editor text
surfaces and the representation-aware plumbing required to extend copy/export
semantics without rewriting the editor clipboard API.

The canonical representation vocabulary lives in
`docs/ux/clipboard_history_contract.md` (§5). Editor surfaces default to the
lossless `raw` representation and may add additional representations later.

## Invariants

- Editor copy/cut reads from the buffer snapshot, not the rendered surface.
- Default `representation_class` is `raw` (exact UTF-8 bytes as selected).
- Selection mapping is grapheme-aware (`(line, grapheme)` → byte offsets).
- Copy variants are named and stable via `CopyVariantId::id()`.

## Public API

The editor clipboard helpers live in `crates/aureline-editor/src/clipboard/`:

- `aureline_editor::clipboard::plan_copy_default`
- `aureline_editor::clipboard::plan_cut_default`
- `aureline_editor::clipboard::plan_copy_variant`
- `aureline_editor::clipboard::plan_cut_variant`

Planning produces:

- `CopyPayload` (`copy_variant_id`, `representation_class`, `text`)
- `CutPayload` (`payload`, `delete_ranges`)

The planned delete ranges are byte offsets against the same `Snapshot` used for
planning. Shell/editor consumers apply them as one grouped edit transaction via
`SelectionState::apply_delete_byte_ranges`.

## Default variants

- `copy.variant.selection_raw` (`representation_class = raw`)
  - Used when any caret has a non-empty selection range.
  - Multiple non-overlapping ranges join with `\n` separators.
- `copy.variant.line` (`representation_class = raw`)
  - Used when every caret is selection-empty.
  - Copies the whole logical line(s) the caret set covers, including line
    terminators when present (LF/CRLF/CR).

## Shell wiring (current consumer)

The desktop shell (`crates/aureline-shell/src/bootstrap/native_shell.rs`)
wires `Ctrl/Cmd+C`, `Ctrl/Cmd+X`, and `Ctrl/Cmd+V` against the active editor tab
session using the planning APIs above. The system clipboard receives only the
planned `text` payload; representation metadata remains available on the
planned payload for future representation-labeled copy/export plumbing.

## Fixtures and proof

- Fixtures: `fixtures/editor/clipboard_cases/`
- Test harness: `crates/aureline-editor/tests/clipboard_cases.rs`

