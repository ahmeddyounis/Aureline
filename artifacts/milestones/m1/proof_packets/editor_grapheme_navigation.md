# Proof packet: editor grapheme-aware navigation

Purpose: anchor proof captures for grapheme-aware cursor navigation, deletion,
and byte-offset translation across the buffer/editor/shell boundary.

Canonical sources (non-exhaustive):

- `.plans/M01-044.md`
- `.t2/docs/Aureline_Technical_Architecture_Document.md` (§11.1–11.2)
- `crates/aureline-buffer/src/piece_tree/buffer.rs` (byte↔(line, grapheme) translation)
- `crates/aureline-editor/src/selection/` (grapheme-aware deletion)
- `crates/aureline-editor/src/text_nav/` (word navigation + translation helpers)
- `crates/aureline-editor/tests/grapheme_nav_cases.rs`
- `fixtures/editor/grapheme_nav_cases/`
- `crates/aureline-input/src/text_input/mod.rs` (Delete + word-motion normalization)
- `crates/aureline-shell/src/bootstrap/native_shell.rs` (live consumer)

Evidence storage:

- Captures: `artifacts/milestones/m1/captures/`

## Latest validation capture

- Capture: `artifacts/milestones/m1/captures/editor_grapheme_navigation_validation_capture.json`
- Command: `cargo test -p aureline-editor -p aureline-shell`

