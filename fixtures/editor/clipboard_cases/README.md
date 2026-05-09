# Editor clipboard planning fixtures

These fixtures exercise the editor clipboard planning helpers in
`crates/aureline-editor/src/clipboard/` against real buffer snapshots and
selection state. They cover:

- default copy choosing `copy.variant.selection_raw` vs `copy.variant.line`
- raw-byte fidelity for line endings (LF/CRLF)
- multi-range copy joins for multiple carets/selections
- cut planning plus applying the planned byte-range deletions

The canonical vocabulary for copy variants and representation classes lives in
`docs/ux/clipboard_history_contract.md` (§5).

